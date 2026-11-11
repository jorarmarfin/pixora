//! Lotes: `start_batch` (bloquea hasta terminar, emite progreso) y `cancel_batch`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::domain::{ImageJob, JobResult};
use crate::imaging::{self, pipeline};
use crate::jobs::{self, BatchEvent, BatchSummary};

pub const PROGRESS_EVENT: &str = "batch://progress";
const THROTTLE: Duration = Duration::from_millis(100);

#[derive(Debug, Deserialize)]
pub struct BatchRequest {
    pub jobs: Vec<ImageJob>,
    #[serde(default)]
    pub overwrite_confirmed: bool,
    pub concurrency: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct BatchReport {
    pub summary: BatchSummary,
    pub results: Vec<JobResult>,
}

/// Payload de progreso: cambios acumulados desde el último evento.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProgressPayload {
    pub total: usize,
    /// Trabajos terminados hasta ahora (cualquier estado terminal).
    pub done: usize,
    /// Ids que empezaron a procesarse desde el último evento.
    pub started: Vec<String>,
    /// Resultados terminados desde el último evento.
    pub finished: Vec<JobResult>,
}

/// Agrupa eventos y los libera como mucho cada `THROTTLE` (evita saturar el webview).
struct Throttle<F: Fn(ProgressPayload)> {
    inner: Mutex<ThrottleState>,
    emit: F,
}

struct ThrottleState {
    pending: ProgressPayload,
    last: Instant,
}

impl<F: Fn(ProgressPayload)> Throttle<F> {
    fn new(total: usize, emit: F) -> Self {
        Self {
            inner: Mutex::new(ThrottleState {
                pending: ProgressPayload {
                    total,
                    ..Default::default()
                },
                last: Instant::now() - THROTTLE,
            }),
            emit,
        }
    }

    fn push(&self, ev: BatchEvent) {
        let mut st = self.inner.lock().unwrap();
        match ev {
            BatchEvent::Started(id) => st.pending.started.push(id),
            BatchEvent::Finished(r) => {
                st.pending.done += 1;
                st.pending.finished.push(r);
            }
        }
        if st.last.elapsed() >= THROTTLE {
            self.flush_locked(&mut st);
        }
    }

    fn flush_locked(&self, st: &mut ThrottleState) {
        st.last = Instant::now();
        let done = st.pending.done;
        let total = st.pending.total;
        let payload = std::mem::replace(
            &mut st.pending,
            ProgressPayload {
                total,
                done,
                ..Default::default()
            },
        );
        (self.emit)(payload);
    }

    fn flush(&self) {
        let mut st = self.inner.lock().unwrap();
        if !st.pending.started.is_empty() || !st.pending.finished.is_empty() {
            self.flush_locked(&mut st);
        }
    }
}

/// Lote en curso (uno a la vez): bandera de cancelación compartida.
#[derive(Default)]
pub struct BatchState {
    current: Mutex<Option<Arc<AtomicBool>>>,
}

impl BatchState {
    fn begin(&self) -> Result<Arc<AtomicBool>, String> {
        let mut cur = self.current.lock().unwrap();
        if cur.is_some() {
            return Err("ya hay un lote en curso".into());
        }
        let flag = Arc::new(AtomicBool::new(false));
        *cur = Some(flag.clone());
        Ok(flag)
    }

    fn end(&self) {
        *self.current.lock().unwrap() = None;
    }

    /// `true` si había un lote que cancelar.
    pub fn cancel(&self) -> bool {
        match self.current.lock().unwrap().as_ref() {
            Some(f) => {
                f.store(true, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }
}

pub fn run_batch_blocking(
    req: &BatchRequest,
    cancel: &AtomicBool,
    emit: impl Fn(ProgressPayload) + Sync,
) -> Result<BatchReport, String> {
    let engine = imaging::engine()?;
    let throttle = Throttle::new(req.jobs.len(), emit);
    let (results, summary) = jobs::run_batch(
        engine,
        &req.jobs,
        &pipeline::Options {
            overwrite_confirmed: req.overwrite_confirmed,
        },
        jobs::clamp_concurrency(req.concurrency),
        cancel,
        &|ev| throttle.push(ev),
    );
    throttle.flush();
    Ok(BatchReport { summary, results })
}

#[tauri::command]
pub async fn start_batch(
    app: AppHandle,
    state: State<'_, BatchState>,
    req: BatchRequest,
) -> Result<BatchReport, String> {
    let cancel = state.begin()?;
    let out = tauri::async_runtime::spawn_blocking(move || {
        run_batch_blocking(&req, &cancel, |p| {
            let _ = app.emit(PROGRESS_EVENT, p);
        })
    })
    .await
    .map_err(|e| e.to_string());
    state.end();
    out?
}

/// Cancelación cooperativa: no inicia nuevos trabajos; los activos terminan.
#[tauri::command]
pub fn cancel_batch(state: State<'_, BatchState>) -> bool {
    state.cancel()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ConflictPolicy, ImageFormat, MetadataPolicy, QualityMode, ResizeOptions};

    #[test]
    fn throttle_agrupa_y_no_pierde_eventos() {
        let got = Mutex::new(Vec::<ProgressPayload>::new());
        let t = Throttle::new(500, |p| got.lock().unwrap().push(p));
        for i in 0..500 {
            t.push(BatchEvent::Started(format!("{i}")));
            t.push(BatchEvent::Finished(JobResult::failed(
                &format!("{i}"),
                crate::domain::JobError::new(crate::domain::JobErrorCode::Io),
                0,
                0,
            )));
        }
        t.flush();
        let got = got.into_inner().unwrap();
        assert!(got.len() < 50, "demasiados eventos: {}", got.len());
        assert_eq!(got.iter().map(|p| p.finished.len()).sum::<usize>(), 500);
        assert_eq!(got.iter().map(|p| p.started.len()).sum::<usize>(), 500);
        assert_eq!(got.last().unwrap().done, 500);
    }

    #[test]
    fn solo_un_lote_a_la_vez_y_cancelar() {
        let s = BatchState::default();
        assert!(!s.cancel());
        let f = s.begin().unwrap();
        assert!(s.begin().is_err());
        assert!(s.cancel() && f.load(Ordering::SeqCst));
        s.end();
        assert!(s.begin().is_ok());
    }

    #[test]
    fn lote_end_to_end_sin_tauri() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        image::RgbImage::new(10, 10).save(&src).unwrap();
        let req = BatchRequest {
            jobs: vec![ImageJob {
                id: "x".into(),
                input_path: src.to_string_lossy().into_owned(),
                output_directory: d.path().to_string_lossy().into_owned(),
                output_format: ImageFormat::Webp,
                quality_mode: QualityMode::Lossy,
                quality_value: 80,
                resize: ResizeOptions::None,
                metadata_policy: MetadataPolicy::Strip,
                name_suffix: "-pixora".into(),
                conflict_policy: ConflictPolicy::Rename,
            }],
            overwrite_confirmed: false,
            concurrency: None,
        };
        let r = run_batch_blocking(&req, &AtomicBool::new(false), |_| {}).unwrap();
        assert_eq!(r.summary.completed, 1);
    }
}
