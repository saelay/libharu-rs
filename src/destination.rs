use super::core;
use super::Real;
use super::Result;

use std::ops::{Deref};

pub struct Destination<'doc, T>
where
    T: Deref<Target=core::Destination<'doc>>
{
    dst: T,
}

impl<'doc> Destination<'doc, Box<core::Destination<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Destination) -> Self {
        Self { dst: Box::new(core::Destination::new(doc, handle)) }
    }
}

impl<'doc, T> Destination<'doc, T>
where
    T: Deref<Target=core::Destination<'doc>>
{
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Destination {
        self.dst.handle()
    }

    #[inline]
    pub fn set_xyz(&mut self, left: Real, top: Real, bottom: Real) -> Result<()> {
        self.dst.deref().set_xyz(left, top, bottom)
    }

    #[inline]
    pub fn set_fit(&mut self) -> Result<()> {
        self.dst.deref().set_fit()
    }

    #[inline]
    pub fn set_fit_h(&mut self, top: Real) -> Result<()> {
        self.dst.deref().set_fit_h(top)
    }

    #[inline]
    pub fn set_fit_v(&mut self, left: Real) -> Result<()> {
        self.dst.deref().set_fit_v(left)
    }

    #[inline]
    pub fn set_fit_r(&mut self, left: Real, bottom: Real, right: Real, top: Real) -> Result<()> {
        self.dst.deref().set_fit_r(left, bottom, right, top)
    }

    #[inline]
    pub fn set_fit_b(&mut self) -> Result<()> {
        self.dst.deref().set_fit_b()
    }

    #[inline]
    pub fn set_fit_bh(&mut self, top: Real) -> Result<()> {
        self.dst.deref().set_fit_bh(top)
    }

    #[inline]
    pub fn set_fit_bv(&mut self, top: Real) -> Result<()> {
        self.dst.deref().set_fit_bv(top)
    }
}
