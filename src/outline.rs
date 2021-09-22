
use super::core;


use std::ops::{Deref};

pub struct Outline<'doc, T>
where
    T: Deref<Target=core::Outline<'doc>>
{
    oline: T,
}

impl<'doc> Outline<'doc, Box<core::Outline<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Outline) -> Self {
        Self { oline: Box::new(core::Outline::new(doc, handle)) }
    }
}

impl<'doc, T> Outline<'doc, T>
where
    T: Deref<Target=core::Outline<'doc>>
{
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Outline {
        self.oline.handle()
    }

}
