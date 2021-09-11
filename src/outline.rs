use crate::document::Document;
use crate::destination::Destination;

/// Outline handle type.
pub struct Outline<'a> {
    outline: libharu_sys::HPDF_Outline,
    _doc: &'a Document,
}

impl<'a> Outline<'a> {
    pub(crate) fn new(_doc: &'a Document, outline: libharu_sys::HPDF_Outline) -> Self {
        Self { _doc, outline }
    }

    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Outline {
        self.outline
    }

    /// Set whether this node is opened or not when the outline is displayed for the first time.
    #[must_use]
    pub fn set_opened(&self, opened: bool) -> anyhow::Result<()> {
        let opened = match opened {
            true => libharu_sys::HPDF_TRUE,
            false => libharu_sys::HPDF_FALSE,
        };

        let status = unsafe {
            libharu_sys::HPDF_Outline_SetOpened(self.handle(), opened)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Outline_SetOpened failed (status={})", status);
        }

        Ok(())
    }
    
    /// Set a destination object which becomes to a target to jump when the outline is clicked.
    #[must_use]
    pub fn set_destination(&self, dst: &Destination) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Outline_SetDestination(self.handle(), dst.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Outline_SetDestination failed (status={})", status);
        }

        Ok(())
    }
}
