use super::{
    Document,
};

pub struct Image<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Image,
}

impl<'doc> Image<'doc> {
    #[inline]
    pub fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Image) -> Self {
        Self { doc, handle }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Image {
        self.handle
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc Document {
        self.doc
    }
}
