use serde::Deserialize;

use crate::domain::{ImageJob, JobError, JobErrorCode, JobResult};
use crate::imaging::{self, pipeline};

#[derive(Debug, Deserialize)]
pub struct ProcessRequest {
    pub job: ImageJob,
    #[serde(default)]
    pub overwrite_confirmed: bool,
}

pub fn process_blocking(req: ProcessRequest) -> JobResult {
    match imaging::engine() {
        Ok(engine) => pipeline::run_job(
            engine,
            &req.job,
            &pipeline::Options {
                overwrite_confirmed: req.overwrite_confirmed,
            },
        ),
        Err(msg) => JobResult::failed(
            &req.job.id,
            JobError::with_detail(JobErrorCode::Engine, msg),
            0,
            0,
        ),
    }
}

/// Procesa una imagen. Los fallos se devuelven dentro de `JobResult` (status `failed`).
#[tauri::command]
pub async fn process_single(req: ProcessRequest) -> Result<JobResult, String> {
    tauri::async_runtime::spawn_blocking(move || process_blocking(req))
        .await
        .map_err(|e| e.to_string())
}
