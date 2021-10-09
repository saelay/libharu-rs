use crate::document::Document;
use crate::Real;

/// Image handle type.
pub struct Image<'a> {
    _doc: &'a Document,
    img: libharu_sys::HPDF_Image,
}

impl<'a> Image<'a> {
    pub(crate) fn new(_doc: &'a Document, img: libharu_sys::HPDF_Destination) -> Self {
        Self { _doc, img }
    }
    
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Destination {
        self.img
    }
    
    /// Get the width of the image of an image object.
    pub fn width(&self) -> anyhow::Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Image_GetWidth(self.handle())
        };

        Ok(ret as Real)
    }

    /// Get the height of the image of an image object.
    pub fn height(&self) -> anyhow::Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Image_GetHeight(self.handle())
        };

        Ok(ret as Real)
    }
}
