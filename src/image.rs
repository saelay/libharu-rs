
use super::core;


use std::ops::{Deref};

pub struct Image<'doc, T>
where
    T: Deref<Target=core::Image<'doc>>
{
    img: T,
}

impl<'doc> Image<'doc, Box<core::Image<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Image) -> Self {
        Self { img: Box::new(core::Image::new(doc, handle)) }
    }
}

impl<'doc, T> Image<'doc, T>
where
    T: Deref<Target=core::Image<'doc>>
{
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Image {
        self.img.handle()
    }

}
