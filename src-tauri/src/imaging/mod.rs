//! Motor de imagen: trait independiente de la UI y adaptadores.

pub mod engine;
pub mod geometry;
pub mod pipeline;
pub mod vips;

pub use engine::{EncodeOutput, EncodeRequest, ImageEngine};

use std::sync::OnceLock;

static ENGINE: OnceLock<Result<vips::VipsEngine, String>> = OnceLock::new();

/// Motor global (libvips se inicializa una sola vez en el proceso).
pub fn engine() -> Result<&'static dyn ImageEngine, String> {
    ENGINE
        .get_or_init(vips::VipsEngine::init)
        .as_ref()
        .map(|e| e as &dyn ImageEngine)
        .map_err(Clone::clone)
}
