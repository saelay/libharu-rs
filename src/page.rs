use super::core;
use super::core::error::Result;
use super::font::Font;
use super::destination::Destination;
use super::core::page::{Page, PageDescPathCommonFunction, PageDescTextCommonFunction};
use std::ops::{Deref};
use std::ffi::{CString};

pub struct PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{
    page: T
}

impl<'doc> PageDescriptionMode<'doc, Box<core::PageDescriptionMode<'doc>>> {
    #[inline]
    pub(crate) fn new(doc: &'doc core::Document, handle: libharu_sys::HPDF_Page) -> Self {
        Self { page: Box::new(core::PageDescriptionMode::new(doc, handle)) }
    }
}

impl<'doc, 'a> PageDescriptionMode<'doc, &'a core::PageDescriptionMode<'doc>> {
    #[inline]
    pub(crate) fn new_by_ref(page: &'a core::PageDescriptionMode<'doc>) -> Self {
        Self { page }
    }
}

impl<'doc, T> PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{
    /// Gets the handle of the page's current font.
    pub fn current_font(&self) -> Result<Font<'_, Box<core::Font>>> {
        Ok(Font::new(
            self.page.deref().doc(),
            self.page.deref().current_font()?.handle()
        ))
    }

    /// Create a new destination object for the page.
    pub fn create_destination(&self) -> Result<Destination<'_, Box<core::Destination>>> {
        Ok(Destination::new(
            self.page.deref().doc(),
            self.page.deref().create_destination()?.handle()
        ))
    }
    
}

impl<'doc, T> super::core::page::private::PageHandle<'doc> for PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{
    fn handle(&self) -> libharu_sys::HPDF_Page {
        self.page.deref().handle()
    }
}

impl<'doc, T> super::core::Page<'doc> for PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{
    fn doc(&self) -> &'doc super::core::Document {
        self.page.deref().doc()
    }
}
/*
impl<'doc, T> super::core::PageDescTextCommonFunctionCStr<'doc> for PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{}
*/
impl<'doc, T> super::core::PageDescTextCommonFunction<'doc> for PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{}

pub trait PageDescTextCommonFunctionStr<'doc> : super::core::Page<'doc> {
    /// Print the text at the current position on the page.
    fn show_text(&mut self, text: &str) -> Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(
                self.handle(),
                text.as_ptr())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
}

impl<'doc, T> PageDescTextCommonFunctionStr<'doc> for PageDescriptionMode<'doc, T>
where
    T: Deref<Target=core::PageDescriptionMode<'doc>>
{
}
