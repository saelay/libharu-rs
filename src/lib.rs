//! Rust binding of libharu PDF library.

#![warn(missing_docs)]
mod document;
mod page;
mod outline;
mod destination;
mod error;

pub use document::{
    Document,
    PageNumStyle,
    CompressionMode,
    PageMode,
};

pub use page::{
    Page,
    LineCap,
    LineJoin,
    TextRenderingMode,
};

pub use outline::{
    Outline,
};

pub use destination::{
    Destination,
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

    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Font {
        self.font
    }

    /// Get the name of the font.
    pub fn name(&self) -> anyhow::Result<&str> {
        unsafe {
            let name = libharu_sys::HPDF_Font_GetFontName(self.handle());

            let s = std::ffi::CStr::from_ptr(name).to_str()?;

            Ok(s)
        }
    }
}
