use crate::page::Page;

/// Destination object
pub struct Destination<'a, 'b> {
    dst: libharu_sys::HPDF_Destination,
    _page: &'a Page<'b>,
}

impl<'a, 'b> Destination<'a, 'b> {
    pub(crate) fn new(_page: &'a Page<'b>, dst: libharu_sys::HPDF_Destination) -> Self {
        Self { dst, _page }
    }
    
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Destination {
        self.dst
    }
}