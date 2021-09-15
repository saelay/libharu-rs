//! Rust binding of libharu PDF library.

#![warn(missing_docs)]
mod document;
mod page;
mod outline;
mod destination;
mod encoder;
mod error;
mod context;
pub mod prelude;
/*
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
    PageSize,
    PageDirection,
    TextAlignment,
};

pub use outline::{
    Outline,
};

pub use destination::{
    Destination,
};

pub use context::{
    PageTextMode,
    PagePathMode,
    PageDescriptionMode,
    PageDescTextCommon,
    PageDescPathCommon,
};
*/
/// Floating-point type used in libharu.
pub type Real = libharu_sys::HPDF_REAL;

/// RGB color type.
#[derive(Debug, Clone)]
pub struct Color {
    /// red (0.0 ~ 1.0)
    pub red: Real,

    /// green (0.0 ~ 1.0)
    pub green: Real,

    /// blue (0.0 ~ 1.0)
    pub blue: Real,
}

impl Copy for Color {}

impl From<(Real, Real, Real)> for Color {
    fn from(v: (Real, Real, Real)) -> Self {
        Self { red: v.0, green: v.1, blue: v.2 }
    }
}

/// CMYK color type
#[derive(Debug, Clone)]
pub struct CmykColor {
    pub cyan: Real,
    pub magenta: Real,
    pub yellow: Real,
    pub keyplate: Real,
}

impl Copy for CmykColor {}

impl From<(Real, Real, Real, Real)> for CmykColor {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self { cyan: v.0, magenta: v.1, yellow: v.2, keyplate: v.3 }
    }
}
/// Point
#[derive(Debug, Clone)]
pub struct Point {
    /// x
    pub x: Real,

    /// y
    pub y: Real,
}

impl Copy for Point {}

impl From<(Real, Real)> for Point {
    fn from(v: (Real, Real)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

/// Rect
#[derive(Debug, Clone)]
pub struct Rect {
    pub left: Real,
    pub top: Real,
    pub right: Real,
    pub bottom: Real,
}

impl Copy for Rect {}

impl From<(Real, Real, Real, Real)> for Rect {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self { left: v.0, top: v.1, right: v.2, bottom: v.3 }
    }
}

/// Font handle type.
pub struct Font<'a> {
    font: libharu_sys::HPDF_Font,
    _doc: &'a prelude::Document,
}

impl<'a> Font<'a> {
    pub(crate) fn new(_doc: &'a prelude::Document, font: libharu_sys::HPDF_Font) -> Self {
        Self { font, _doc }
    }

    #[inline]
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
