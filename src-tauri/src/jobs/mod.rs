//! Scheduler de lotes: pool acotado de workers, cancelación cooperativa y resumen agregado.
//!
//! Cada worker toma el siguiente trabajo pendiente y ejecuta el pipeline completo
//! (validar → codificar a temporal → publicar). La cancelación se comprueba antes de
//! iniciar cada trabajo: los activos terminan (granularidad por archivo, libvips no es
//! interrumpible) y los pendientes quedan como `cancelled`. Un error en un archivo nunca
//! aborta el lote.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use serde::Serialize;

use crate::domain::{ImageJob, JobError, JobErrorCode, JobResult, JobStatus};
use crate::imaging::{ImageEngine, pipeline};

pub const MIN_CONCURRENCY: usize = 1;
pub const MAX_CONCURRENCY: usize = 8;
pub const DEFAULT_CONCURRENCY: usize = 3;

/// Evento emitido por el scheduler (desde hilos de trabajo).
#[derive(Debug, Clone)]
pub enum BatchEvent {
    /// El trabajo `id` empezó a procesarse.
    Started(String),
    /// El trabajo terminó (cualquier estado terminal).
    Finished(JobResult),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct BatchSummary {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub cancelled: usize,
    /// Bytes de entrada de los trabajos completados.
    pub input_bytes: u64,
    /// Bytes de salida de los trabajos completados.
    pub output_bytes: u64,
    /// `input_bytes - output_bytes` (negativo si la salida pesa más).
    pub saved_bytes: i64,
    pub duration_ms: u64,
    pub was_cancelled: bool,
}

impl BatchSummary {
    pub fn from_results(results: &[JobResult], duration_ms: u64, was_cancelled: bool) -> Self {
        let mut s = Self {
            total: results.len(),
            duration_ms,
            was_cancelled,
            ..Self::default()
        };
        for r in results {
            match r.status {
                JobStatus::Completed => {
                    s.completed += 1;
                    s.input_bytes += r.input_bytes;
                    s.output_bytes += r.output_bytes;
                }
                JobStatus::Failed => s.failed += 1,
                JobStatus::Skipped => s.skipped += 1,
                JobStatus::Cancelled => s.cancelled += 1,
                _ => {}
            }
        }
        s.saved_bytes = s.input_bytes as i64 - s.output_bytes as i64;
        s
    }
}

pub fn clamp_concurrency(n: Option<usize>) -> usize {
    n.unwrap_or(DEFAULT_CONCURRENCY)
        .clamp(MIN_CONCURRENCY, MAX_CONCURRENCY)
}

fn cancelled_result(job: &ImageJob) -> JobResult {
    JobResult {
        id: job.id.clone(),
        status: JobStatus::Cancelled,
        output_path: None,
        width: 0,
        height: 0,
        input_bytes: 0,
        output_bytes: 0,
        duration_ms: 0,
        error: None,
    }
}

/// Ejecuta el lote y devuelve un resultado por trabajo, en el orden de entrada.
pub fn run_batch(
    engine: &dyn ImageEngine,
    jobs: &[ImageJob],
    opts: &pipeline::Options,
    concurrency: usize,
    cancel: &AtomicBool,
    on_event: &(dyn Fn(BatchEvent) + Sync),
) -> (Vec<JobResult>, BatchSummary) {
    let started = Instant::now();
    let next = AtomicUsize::new(0);
    let slots: Vec<std::sync::Mutex<Option<JobResult>>> =
        jobs.iter().map(|_| std::sync::Mutex::new(None)).collect();
    let workers = concurrency
        .clamp(MIN_CONCURRENCY, MAX_CONCURRENCY)
        .min(jobs.len().max(1));

    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(job) = jobs.get(i) else { break };
                    let result = if cancel.load(Ordering::SeqCst) {
                        cancelled_result(job)
                    } else {
                        on_event(BatchEvent::Started(job.id.clone()));
                        catch_unwind(AssertUnwindSafe(|| pipeline::run_job(engine, job, opts)))
                            .unwrap_or_else(|_| {
                                JobResult::failed(
                                    &job.id,
                                    JobError::with_detail(JobErrorCode::Engine, "fallo interno"),
                                    0,
                                    0,
                                )
                            })
                    };
                    on_event(BatchEvent::Finished(result.clone()));
                    *slots[i].lock().unwrap() = Some(result);
                }
            });
        }
    });

    let results: Vec<JobResult> = slots
        .into_iter()
        .zip(jobs)
        .map(|(s, job)| {
            s.into_inner()
                .unwrap()
                .unwrap_or_else(|| cancelled_result(job))
        })
        .collect();
    let summary = BatchSummary::from_results(
        &results,
        started.elapsed().as_millis() as u64,
        cancel.load(Ordering::SeqCst),
    );
    (results, summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ConflictPolicy, ImageFormat, MetadataPolicy, QualityMode, ResizeOptions};
    use crate::imaging;
    use image::{ImageFormat as F, Rgb, RgbImage};
    use std::path::Path;
    use std::sync::Mutex;

    fn job(id: &str, input: &Path, out: &Path) -> ImageJob {
        ImageJob {
            id: id.into(),
            input_path: input.to_string_lossy().into_owned(),
            output_directory: out.to_string_lossy().into_owned(),
            output_format: ImageFormat::Webp,
            quality_mode: QualityMode::Lossy,
            quality_value: 70,
            resize: ResizeOptions::None,
            metadata_policy: MetadataPolicy::Strip,
            name_suffix: "-pixora".into(),
            conflict_policy: ConflictPolicy::Rename,
        }
    }

    fn png(p: &Path, seed: u32) {
        RgbImage::from_fn(48, 48, |x, y| {
            Rgb([(x * seed) as u8, (y * 3) as u8, seed as u8])
        })
        .save_with_format(p, F::Png)
        .unwrap();
    }

    fn tmp_files(dir: &Path) -> usize {
        std::fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().contains(".tmp."))
            .count()
    }

    const OPTS: pipeline::Options = pipeline::Options {
        overwrite_confirmed: false,
    };

    #[test]
    fn lote_de_100_mezcla_ok_y_errores_controlados() {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let mut jobs = Vec::new();
        for i in 0..100u32 {
            let p = d.path().join(format!("img{i}.png"));
            match i % 10 {
                3 => std::fs::write(&p, b"corrupto").unwrap(),
                7 => {} // no existe
                _ => png(&p, i + 1),
            }
            jobs.push(job(&format!("j{i}"), &p, &out));
        }
        let events = Mutex::new(0usize);
        let (results, s) = run_batch(
            imaging::engine().unwrap(),
            &jobs,
            &OPTS,
            4,
            &AtomicBool::new(false),
            &|e| {
                if matches!(e, BatchEvent::Finished(_)) {
                    *events.lock().unwrap() += 1;
                }
            },
        );
        assert_eq!(results.len(), 100);
        assert_eq!((s.total, s.completed, s.failed), (100, 80, 20));
        assert_eq!(*events.lock().unwrap(), 100);
        for (r, j) in results.iter().zip(&jobs) {
            assert_eq!(r.id, j.id, "orden preservado");
        }
        assert!(results[3].error.as_ref().unwrap().code == JobErrorCode::Corrupt);
        assert!(results[7].error.as_ref().unwrap().code == JobErrorCode::InputNotFound);
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), 80);
        assert_eq!(tmp_files(&out), 0);
        assert_eq!(s.saved_bytes, s.input_bytes as i64 - s.output_bytes as i64);
    }

    #[test]
    fn cancelar_a_mitad_deja_salidas_consistentes_sin_tmp() {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let jobs: Vec<_> = (0..40u32)
            .map(|i| {
                let p = d.path().join(format!("i{i}.png"));
                png(&p, i + 1);
                job(&format!("j{i}"), &p, &out)
            })
            .collect();
        let cancel = AtomicBool::new(false);
        let done = AtomicUsize::new(0);
        let (results, s) = run_batch(imaging::engine().unwrap(), &jobs, &OPTS, 2, &cancel, &|e| {
            if matches!(e, BatchEvent::Finished(_)) && done.fetch_add(1, Ordering::SeqCst) == 5 {
                cancel.store(true, Ordering::SeqCst);
            }
        });
        assert!(s.was_cancelled);
        assert!(s.cancelled > 0 && s.completed >= 6);
        assert_eq!(s.completed + s.cancelled, 40);
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), s.completed);
        assert_eq!(tmp_files(&out), 0);
        for r in results.iter().filter(|r| r.status == JobStatus::Completed) {
            assert!(Path::new(r.output_path.as_ref().unwrap()).exists());
        }
    }

    #[test]
    fn colisiones_en_lote_no_pisan_salidas() {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let src = d.path().join("a.png");
        png(&src, 3);
        // Mismo origen/destino 12 veces en paralelo.
        let jobs: Vec<_> = (0..12).map(|i| job(&format!("j{i}"), &src, &out)).collect();
        let (results, s) = run_batch(
            imaging::engine().unwrap(),
            &jobs,
            &OPTS,
            4,
            &AtomicBool::new(false),
            &|_| {},
        );
        assert_eq!(s.completed, 12, "{results:?}");
        let mut paths: Vec<_> = results
            .iter()
            .map(|r| r.output_path.clone().unwrap())
            .collect();
        paths.sort();
        paths.dedup();
        assert_eq!(paths.len(), 12);
        assert_eq!(tmp_files(&out), 0);
    }

    #[test]
    fn lote_vacio_y_concurrencia_acotada() {
        let (r, s) = run_batch(
            imaging::engine().unwrap(),
            &[],
            &OPTS,
            3,
            &AtomicBool::new(false),
            &|_| {},
        );
        assert!(r.is_empty() && s.total == 0);
        assert_eq!(clamp_concurrency(None), DEFAULT_CONCURRENCY);
        assert_eq!(clamp_concurrency(Some(0)), 1);
        assert_eq!(clamp_concurrency(Some(99)), MAX_CONCURRENCY);
    }

    /// Memoria pico del proceso (KB) según /proc (solo Linux).
    fn peak_rss_kb() -> u64 {
        std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("VmHWM:"))
                    .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
            })
            .unwrap_or(0)
    }

    /// Benchmark manual: `cargo test --release bench_lotes -- --ignored --nocapture`.
    #[test]
    #[ignore = "benchmark: ejecutar a mano"]
    fn bench_lotes_10_100_1000() {
        let d = tempfile::tempdir().unwrap();
        let base = d.path().join("base.png");
        RgbImage::from_fn(1600, 1200, |x, y| {
            Rgb([(x ^ y) as u8, (x.wrapping_mul(y) >> 3) as u8, (x + y) as u8])
        })
        .save(&base)
        .unwrap();
        for n in [10usize, 100, 1000] {
            let out = d.path().join(format!("o{n}"));
            std::fs::create_dir_all(&out).unwrap();
            let jobs: Vec<_> = (0..n)
                .map(|i| {
                    let p = d.path().join(format!("in{n}_{i}.png"));
                    std::fs::hard_link(&base, &p).unwrap();
                    let mut j = job(&format!("j{i}"), &p, &out);
                    j.resize = ResizeOptions::Width { px: 800 };
                    j
                })
                .collect();
            let before = peak_rss_kb();
            let (_, s) = run_batch(
                imaging::engine().unwrap(),
                &jobs,
                &OPTS,
                DEFAULT_CONCURRENCY,
                &AtomicBool::new(false),
                &|_| {},
            );
            println!(
                "BENCH n={n} ok={} {} ms ({:.1} img/s) pico RSS {} MB (antes {} MB)",
                s.completed,
                s.duration_ms,
                n as f64 * 1000.0 / s.duration_ms.max(1) as f64,
                peak_rss_kb() / 1024,
                before / 1024,
            );
            assert_eq!(s.completed, n);
            assert_eq!(tmp_files(&out), 0);
        }
    }
}
