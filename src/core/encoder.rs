use super::{
    Document,
};

pub struct Encoder<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Encoder,
}

impl<'doc> Encoder<'doc> {
    #[inline]
    pub fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Encoder) -> Self {
        Self { doc, handle }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Encoder {
        self.handle
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc Document {
        self.doc
    }
}
