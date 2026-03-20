//! Rendering primitives and sounding image compositor for sharprs.
//!
//! Provides a self-contained RGBA pixel canvas with antialiased drawing,
//! bitmap font text, wind barbs, and PNG output.  The [`compositor`] module
//! assembles all panels into the final SHARPpy-style sounding analysis image.

pub mod canvas;
pub mod compositor;
pub mod hodograph;
pub mod panels;
pub mod param_table;
pub mod skewt;

pub use canvas::{Canvas, ClippedCanvas};
pub use compositor::{render_full_sounding, compute_all_params, ComputedParams};
pub use hodograph::{
    draw_hodograph, render_hodograph, hodograph_data_from_profile,
    HodographData, WindLevel, StormMotion, CorfidiVector, SRWindLayer,
};
