use super::core;

use std::ops::{Deref};

pub struct Encoder<'doc, T>
where
    T: Deref<Target=core::Encoder<'doc>>
{
    enc: T,
}

impl<'doc> Encoder<'doc, Box<core::Encoder<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Encoder) -> Self {
        Self { enc: Box::new(core::Encoder::new(doc, handle)) }
    }
}

impl<'doc, T> Encoder<'doc, T>
where
    T: Deref<Target=core::Encoder<'doc>>
{
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Encoder {
        self.enc.handle()
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc core::Document {
        self.enc.doc_handle()
    }
}
