use crate::prelude::*;//{Page, Rect, Color, CmykColor, Real, Font, Point, LineCap, LineJoin, TextRenderingMode, TextAlignment};
use std::ops::Deref;
use std::ffi::CString;

pub struct PageDescTextCommon<'doc, 'page> {
    page: &'page Page<'doc>,
}

impl<'doc, 'page> PageDescTextCommon<'doc, 'page> {
    pub(crate) fn new(page: &'page Page<'doc>) -> Self {
        Self { page }
    }

    pub fn set_line_width(&self, width: Real) -> anyhow::Result<()> {
        self.page.set_line_width(width)
    }

    pub fn set_line_cap(&self, line_cap: LineCap) -> anyhow::Result<()> {
        self.page.set_line_cap(line_cap)
    }

    pub fn set_line_join(&self, line_join: LineJoin) -> anyhow::Result<()> {
        self.page.set_line_join(line_join)
    }

    pub fn set_dash(&self, dash_mode: &[u16], phase: usize) -> anyhow::Result<()> {
        self.page.set_dash(dash_mode, phase)
    }

    pub fn set_char_space(&self, value: Real) -> anyhow::Result<()> {
        self.page.set_char_space(value)
    }

    pub fn set_word_space(&self, value: Real) -> anyhow::Result<()> {
        self.page.set_word_space(value)
    }

    pub fn set_horizontal_scalling(&self, value: Real) -> anyhow::Result<()> {
        self.page.set_horizontal_scalling(value)
    }

    pub fn set_text_leading(&self, value: Real) -> anyhow::Result<()> {
        self.page.set_text_leading(value)
    }

    pub fn set_font_and_size(&self, font: &Font, size: Real) -> anyhow::Result<()> {
        self.page.set_font_and_size(font, size)
    }

    pub fn set_text_rendering_mode(&self, mode: TextRenderingMode) -> anyhow::Result<()> {
        self.page.set_text_rendering_mode(mode)
    }

    /// Print the text at the current position on the page.
    pub fn show_text(&self, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(self.page.handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowText failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text at the current position on the page. (bytes data)
    pub fn show_text_bytes(&self, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text).unwrap();
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(self.page.handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowText failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line,
    pub fn show_text_next_line(&self, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(self.page.handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLine failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line, (bytes data)
    pub fn show_text_next_line_bytes(&self, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(self.page.handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLine failed (status={})", status);
        }

        Ok(())
    }

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page.
    pub fn show_text_next_line_ex(&self, word_space: Real, char_space: Real, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(self.page.handle(), word_space, char_space, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLineEx failed (status={})", status);
        }

        Ok(())
    }

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page. (bytes data)
    pub fn show_text_next_line_ex_bytes(&self, word_space: Real, char_space: Real, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(self.page.handle(), word_space, char_space, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLineEx failed (status={})", status);
        }

        Ok(())
    }

    /// Set the filling color.
    pub fn set_gray_fill(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayFill(self.page.handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    pub fn set_gray_stroke(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayStroke(self.page.handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set filling color.
    pub fn set_rgb_fill<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBFill(self.page.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    pub fn set_rgb_stroke<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBStroke(self.page.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set the filling color.
    pub fn set_cmyk_fill<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKFill(self.page.handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCMYKFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    pub fn set_cmyk_stroke<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKStroke(self.page.handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCMYKStroke failed (status={})", status);
        }

        Ok(())
    }
}

impl<'doc, 'page> Deref for PageDescTextCommon<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}

//------------------------------------------------------------------------------


pub struct PageDescPathCommon<'doc, 'page> {
    page: &'page Page<'doc>,
}

impl<'doc, 'page> PageDescPathCommon<'doc, 'page> {
    pub(crate) fn new(page: &'page Page<'doc>) -> Self {
        Self { page }
    }

}

impl<'doc, 'page> Deref for PageDescPathCommon<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}

//--------------------------------------------------------------------------
pub struct PageDescriptionMode<'doc, 'page> {
    page_dt: PageDescTextCommon<'doc, 'page>,
    page_dp: PageDescPathCommon<'doc, 'page>,
}

impl<'doc, 'page> PageDescriptionMode<'doc, 'page> {

    pub fn new(page: &'page Page<'doc>) -> Self {
        Self {
            page_dt: PageDescTextCommon::new(page),
            page_dp: PageDescPathCommon::new(page),
        }
    }

    fn begin_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_BeginText(self.page_dt.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_BeginText failed (status={})", status);
        }

        Ok(())
    }

    pub(crate) fn end_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_EndText(self.page_dt.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    pub fn run_text_mode<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(PageTextMode) -> anyhow::Result<()>
    {
        self.begin_text()?;
        let page = PageTextMode::new(&self.page_dt);
        let ret = f(page);
        self.end_text()?;

        ret
    }

    pub fn run_path_mode<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(PagePathMode) -> anyhow::Result<()>
    {
        // TODO: implement
        Ok(())
    }
}

impl<'doc, 'page> Deref for PageDescriptionMode<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        // page_dtもpage_dpも差しているPageは同じなのでどちらでもいいがpage_dtを使っている
        self.page_dt.deref()
    }
}

impl<'doc, 'page> AsRef<PageDescTextCommon<'doc, 'page>> for PageDescriptionMode<'doc, 'page> {
    fn as_ref(&self) -> &PageDescTextCommon<'doc, 'page> {
        &self.page_dt
    }
}

impl<'doc, 'page> AsRef<PageDescPathCommon<'doc, 'page>> for PageDescriptionMode<'doc, 'page> {
    fn as_ref(&self) -> &PageDescPathCommon<'doc, 'page> {
        &self.page_dp
    }
}
//--------------------------------------------------------------------------------------

pub struct PageTextMode<'doc, 'page, 'c> {
    page: &'c PageDescTextCommon<'doc, 'page>,
}

impl<'doc, 'page, 'c> PageTextMode<'doc, 'page, 'c> {
    pub(crate) fn new(page: &'c PageDescTextCommon<'doc, 'page>) -> Self {
        Self { page }
    }
    
    /// Move the current text position to the start of the next line with using specified offset values.
    pub fn move_text_pos<T>(&self, pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_MoveTextPos(self.page.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveTextPos failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line with using specified offset values.
    pub fn move_text_pos2<T>(&self, pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_MoveTextPos2(self.page.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveTextPos2 failed (status={})", status);
        }

        Ok(())
    }

    /// Set text affine transformation matrix.
    pub fn set_text_matrix(&self, a: Real, b: Real, c: Real, d: Real, x: Real, y: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextMatrix(self.page.handle(), a, b, c, d, x, y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextRenderingMode failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line with using specified offset values.
    pub fn move_to_next_line(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_MoveToNextLine(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveToNextLine failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text on the specified position.
    pub fn text_out<T>(&self, pos: T, text: &str) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_TextOut(self.page.handle(), pos.x, pos.y, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text on the specified position. (bytes data)
    pub fn text_out_bytes<T>(&self, pos: T, text: &[u8]) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_TextOut(self.page.handle(), pos.x, pos.y, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text inside the specified region.
    pub fn text_rect<T>(&self, rect: T, text: &str, align: TextAlignment) -> anyhow::Result<()>
    where
        T: Into<Rect>
    {
        let rect = rect.into();
        let text = CString::new(text)?;
        let align = match align {
            TextAlignment::Left => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_LEFT,
            TextAlignment::Right => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_RIGHT,
            TextAlignment::Center => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_CENTER,
            TextAlignment::Justify => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_JUSTIFY,
        };
        let mut len = 0;
        let _status = unsafe {
            libharu_sys::HPDF_Page_TextRect(self.page.handle(), rect.left, rect.top, rect.right, rect.bottom, std::mem::transmute(text.as_ptr()), align, &mut len)
        };

        //if status != 0 {
        //    anyhow::bail!("HPDF_Page_TextRect failed (status={})", status);
        //}

        Ok(())
    }

    /// Print the text inside the specified region. (byte data)
    pub fn text_rect_bytes<T>(&self, rect: T, text: &[u8], align: TextAlignment) -> anyhow::Result<()>
    where
        T: Into<Rect>
    {
        let rect = rect.into();
        let text = CString::new(text)?;
        let align = match align {
            TextAlignment::Left => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_LEFT,
            TextAlignment::Right => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_RIGHT,
            TextAlignment::Center => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_CENTER,
            TextAlignment::Justify => libharu_sys::HPDF_TextAlignment::HPDF_TALIGN_JUSTIFY,
        };
        let mut len = 0;
        let _status = unsafe {
            libharu_sys::HPDF_Page_TextRect(self.page.handle(), rect.left, rect.top, rect.right, rect.bottom, std::mem::transmute(text.as_ptr()), align, &mut len)
        };

        //if status != 0 {
        //    anyhow::bail!("HPDF_Page_TextRect failed (status={})", status);
        //}

        Ok(())
    }
}

impl<'doc, 'page, 'c> Deref for PageTextMode<'doc, 'page, 'c> {
    type Target = PageDescTextCommon<'doc, 'page>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}


//-------------------------------------------------------------------------------------------

pub struct PagePathMode<'doc, 'page, 'c> {
    page: &'c PageDescPathCommon<'doc, 'page>,
}

impl<'doc, 'page, 'c> PagePathMode<'doc, 'page, 'c> {
    pub(crate) fn new(page: &'c PageDescPathCommon<'doc, 'page>) -> Self {
        Self { page }
    }
    
    pub fn move_to<T>(&self, pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_MoveTo(self.page.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveTo failed (status={})", status);
        }

        Ok(())
    }

    pub fn rectangle<T>(&self, pos: T, width: Real, height: Real) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_Rectangle(self.page.handle(), pos.x, pos.y, width, height)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Rectangle failed (status={})", status);
        }

        Ok(())
    }

    pub fn circle<T>(&self, pos: T, ray: Real) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_Circle(self.page.handle(), pos.x, pos.y, ray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Circle failed (status={})", status);
        }

        Ok(())
    }

    pub fn arc<T>(&self, pos: T, ray: Real, ang1: Real, ang2: Real) -> anyhow::Result<()>
    where
        T: Into<Point>
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_Arc(self.page.handle(), pos.x, pos.y, ray, ang1, ang2)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Arc failed (status={})", status);
        }

        Ok(())
    }

}

impl<'doc, 'page, 'c> Deref for PagePathMode<'doc, 'page, 'c> {
    type Target = PageDescPathCommon<'doc, 'page>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}
