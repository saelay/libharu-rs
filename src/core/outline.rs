use super::{
    Document,
};

pub struct Outline<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Outline,
}

impl<'doc> Outline<'doc> {
    #[inline]
    pub fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Outline) -> Self {
        Self { doc, handle }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Outline {
        self.handle
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc Document {
        self.doc
    }
}
