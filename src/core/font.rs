use super::{
    Document,
};

pub struct Font<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Font,
}

impl<'doc> Font<'doc> {
    #[inline]
    pub fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Font) -> Self {
        Self { doc, handle }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Font {
        self.handle
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc Document {
        self.doc
    }
}
