
pub mod document;
pub mod page;
pub mod image;
pub mod font;
pub mod encoder;
pub mod outline;
pub mod destination;
pub mod annotation;
//pub mod error;
pub mod prelude;
pub mod core;

/// 本ライブラリで使用する浮動小数点数。
pub type Real = libharu_sys::HPDF_REAL;

pub use crate::core::error::{Error, Result};
