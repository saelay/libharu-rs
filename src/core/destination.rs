use super::{
    Real,
    Result,
    Document,
};

pub struct Destination<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Destination,
}

impl<'doc> Destination<'doc> {
    #[inline]
    pub fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Destination) -> Self {
        Self { doc, handle }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Destination {
        self.handle
    }

    #[inline]
    pub fn doc_handle(&self) -> &'doc Document {
        self.doc
    }

    pub fn set_xyz(&self, left: Real, top: Real, bottom: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetXYZ(self.handle(), left, top, bottom)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFit(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_h(&self, top: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitH(self.handle(), top)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_v(&self, left: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitV(self.handle(), left)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_r(&self, left: Real, bottom: Real, right: Real, top: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitR(self.handle(), left, bottom, right, top)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_b(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitB(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_bh(&self, top: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitBH(self.handle(), top)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_fit_bv(&self, top: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Destination_SetFitBV(self.handle(), top)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
}
