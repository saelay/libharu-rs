use crate::page::Page;
use crate::Real;

/// Destination handle type.
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

    /// Define the appearance of a page with three parameters which are left, top and zoom.
    pub fn set_xyz(&self, left: Real, top: Real, zoom: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetXYZ(self.handle(), left, top, zoom)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Destination_SetXYZ failed (status={})", status);
        }
        Ok(())
    }
}