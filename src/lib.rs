//! Rust binding of libharu PDF library.

#![warn(missing_docs)]
mod document;
mod page;
mod error;

pub use document::{
    Document,
    PageNumStyle,
    CompressionMode,
};

pub use page::{
    Page,
    LineCap,
    LineJoin,
    TextRenderingMode,
};

/// Floating-point type used in libharu.
pub type Real = libharu_sys::HPDF_REAL;

/// RGB color type.
pub struct Color {
    /// red (0.0 ~ 1.0)
    pub red: Real,

    /// green (0.0 ~ 1.0)
    pub green: Real,

    /// blue (0.0 ~ 1.0)
    pub blue: Real,
}

/// Font object
pub struct Font<'a> {
    font: libharu_sys::HPDF_Font,
    _doc: &'a Document,
}

impl<'a> Font<'a> {
    pub(crate) fn new(_doc: &'a Document, font: libharu_sys::HPDF_Font) -> Self {
        Self { font, _doc }
    }
}
