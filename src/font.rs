use super::core;


use std::ops::{Deref};

pub struct Font<'doc, T>
where
    T: Deref<Target=core::Font<'doc>>
{
    font: T,
}

impl<'doc> Font<'doc, Box<core::Font<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Font) -> Self {
        Self { font: Box::new(core::Font::new(doc, handle)) }
    }
}

impl<'doc, T> Font<'doc, T>
where
    T: Deref<Target=core::Font<'doc>>
{
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Font {
        self.font.handle()
    }

}
