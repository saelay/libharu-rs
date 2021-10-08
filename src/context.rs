use crate::prelude::*;//{Page, Rect, Color, CmykColor, Real, Font, Point, LineCap, LineJoin, TextRenderingMode, TextAlignment};
use std::ops::Deref;
use std::ffi::CString;


/// Page functions in Description mode or Text mode. 
pub trait PageDescTeextCommonFunction<'doc> : Deref<Target=Page<'doc>> {
    /// Get Page
    fn handle(&self) -> &Page;
    
    /// Set line width of page.
    fn set_line_width(&self, width: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineWidth(self.handle().handle(), width)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineWidth failed (status={})", status);
        }

        Ok(())
    }

    /// Set the shape to be used at the ends of line.
    fn set_line_cap(&self, line_cap: LineCap) -> anyhow::Result<()> {
        let line_cap = match line_cap {
            LineCap::Butt => libharu_sys::HPDF_LineCap::HPDF_BUTT_END,
            LineCap::Round => libharu_sys::HPDF_LineCap::HPDF_ROUND_END,
            LineCap::ProjectingSquare => libharu_sys::HPDF_LineCap::HPDF_PROJECTING_SCUARE_END,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineCap(self.handle().handle(), line_cap)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineCap failed (status={})", status);
        }

        Ok(())
    }

    /// Set the line join style in the page.
    fn set_line_join(&self, line_join: LineJoin) -> anyhow::Result<()> {
        let line_join = match line_join {
            LineJoin::Miter => libharu_sys::HPDF_LineJoin::HPDF_MITER_JOIN,
            LineJoin::Round => libharu_sys::HPDF_LineJoin::HPDF_ROUND_JOIN,
            LineJoin::Bevel => libharu_sys::HPDF_LineJoin::HPDF_BEVEL_JOIN,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineJoin(self.handle().handle(), line_join)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineJoin failed (status={})", status);
        }

        Ok(())
    }

    /// Set the line dash pattern in the page.
    fn set_dash(&self, dash_mode: &[u16], phase: usize) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetDash(self.handle().handle(), dash_mode.as_ptr(), dash_mode.len() as u32, phase as u32)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetDash failed (status={})", status);
        }

        Ok(())
    }

    /// Set the character spacing for text showing.
    fn set_char_space(&self, value: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCharSpace(self.handle().handle(), value)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCharSpace failed (status={})", status);
        }

        Ok(())
    }

    /// Set the word spacing for text showing.
    fn set_word_space(&self, value: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWordSpace(self.handle().handle(), value)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetWordSpace failed (status={})", status);
        }

        Ok(())
    }
    
    /// Set the horizontal scalling for text showing.
    fn set_horizontal_scalling(&self, value: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetHorizontalScalling(self.handle().handle(), value)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetHorizontalScalling failed (status={})", status);
        }

        Ok(())
    }

    /// Set text leading
    fn set_text_leading(&self, value: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextLeading(self.handle().handle(), value)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextLeading failed (status={})", status);
        }

        Ok(())
    }

    /// Set font and size.
    fn set_font_and_size(&self, font: &Font, size: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetFontAndSize(self.handle().handle(), font.font, size)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetFontAndSize failed (status={})", status);
        }

        Ok(())
    }

    /// Sets the text rendering mode.
    fn set_text_rendering_mode(&self, mode: TextRenderingMode) -> anyhow::Result<()> {
        let mode = match mode {
            TextRenderingMode::Fill => libharu_sys::HPDF_TextRenderingMode::HPDF_FILL,
            TextRenderingMode::Stroke => libharu_sys::HPDF_TextRenderingMode::HPDF_STROKE,
            TextRenderingMode::FillThenStroke => libharu_sys::HPDF_TextRenderingMode::HPDF_FILL_THEN_STROKE,
            TextRenderingMode::Invisible => libharu_sys::HPDF_TextRenderingMode::HPDF_INVISIBLE,
            TextRenderingMode::FillClipping => libharu_sys::HPDF_TextRenderingMode::HPDF_FILL_CLIPPING,
            TextRenderingMode::StrokeClipping => libharu_sys::HPDF_TextRenderingMode::HPDF_STROKE_CLIPPING,
            TextRenderingMode::FillStrokeClipping => libharu_sys::HPDF_TextRenderingMode::HPDF_FILL_STROKE_CLIPPING,
            TextRenderingMode::Clipping => libharu_sys::HPDF_TextRenderingMode::HPDF_CLIPPING,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextRenderingMode(self.handle().handle(), mode)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextRenderingMode failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text at the current position on the page.
    fn show_text(&self, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(self.handle().handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowText failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text at the current position on the page. (bytes data)
    fn show_text_bytes(&self, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text).unwrap();
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(self.handle().handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowText failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line,
    fn show_text_next_line(&self, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(self.handle().handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLine failed (status={})", status);
        }

        Ok(())
    }

    /// Move the current text position to the start of the next line, (bytes data)
    fn show_text_next_line_bytes(&self, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(self.handle().handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLine failed (status={})", status);
        }

        Ok(())
    }

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page.
    fn show_text_next_line_ex(&self, word_space: Real, char_space: Real, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(self.handle().handle(), word_space, char_space, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLineEx failed (status={})", status);
        }

        Ok(())
    }

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page. (bytes data)
    fn show_text_next_line_ex_bytes(&self, word_space: Real, char_space: Real, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(self.handle().handle(), word_space, char_space, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLineEx failed (status={})", status);
        }

        Ok(())
    }

    /// Set the filling color.
    fn set_gray_fill(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayFill(self.handle().handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    fn set_gray_stroke(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayStroke(self.handle().handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set filling color.
    fn set_rgb_fill<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBFill(self.handle().handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    fn set_rgb_stroke<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBStroke(self.handle().handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set the filling color.
    fn set_cmyk_fill<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKFill(self.handle().handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCMYKFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    fn set_cmyk_stroke<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKStroke(self.handle().handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCMYKStroke failed (status={})", status);
        }

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Page functions in Description mode or Path mode.
pub trait PageDescPathCommonFunction<'doc> : Deref<Target=Page<'doc>> {
    /// Get Page
    fn handle(&self) -> &Page;
}


//--------------------------------------------------------------------------
/// Page object in Description mode.
pub struct PageDescriptionMode<'doc, 'page> {
    page: &'page Page<'doc>,
}

impl<'doc, 'page> PageDescriptionMode<'doc, 'page> {
    /// Create new PageDescriptionMode instance.
    pub fn new(page: &'page Page<'doc>) -> Self {
        Self { page }
    }

    fn begin_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_BeginText(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_BeginText failed (status={})", status);
        }

        Ok(())
    }

    pub(crate) fn end_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_EndText(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Enter text mode.
    pub fn run_text_mode<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&PageTextMode) -> anyhow::Result<()>
    {
        self.begin_text()?;
        let page = PageTextMode::new(&self.page);
        let ret = f(&page);
        self.end_text()?;

        ret
    }
/*
    fn end_path(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_EndPath(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_EndPath failed (status={})", status);
        }
        Ok(())
    }
*/
    /// Enter path mode.
    pub fn run_path_mode<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&PagePathMode) -> anyhow::Result<()>
    {
        let page = PagePathMode::new(&self.page);
        let ret = f(&page);

        // f()内のstroke(), fill()などの呼び出しでDESCRIPTIONモードに戻る。
        // 呼び忘れた場合にここでDESCRIPTIONモードに戻したいがうまくいかない。
        // end_pathだとすでにDESCRIPTIONモードだったときにGStateが壊れる。
        //let _ = self.end_path();

        ret
    }
}

impl<'doc, 'page> Deref for PageDescriptionMode<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}

impl<'doc, 'page> PageDescTeextCommonFunction<'doc> for PageDescriptionMode<'doc, 'page> {
    fn handle(&self) -> &Page {
        &self.page
    }
}

impl<'doc, 'page> PageDescPathCommonFunction<'doc> for PageDescriptionMode<'doc, 'page> {
    fn handle(&self) -> &Page {
        &self.page
    }
}

//--------------------------------------------------------------------------------------

/// Page object in text mode.
pub struct PageTextMode<'doc, 'page> {
    page: &'page Page<'doc>,
}

impl<'doc, 'page> PageTextMode<'doc, 'page> {
    pub(crate) fn new(page: &'page Page<'doc>) -> Self {
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

impl<'doc, 'page> Deref for PageTextMode<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}


impl<'doc, 'page> PageDescTeextCommonFunction<'doc> for PageTextMode<'doc, 'page> {
    fn handle(&self) -> &Page {
        &self.page
    }
}


//-------------------------------------------------------------------------------------------

/// Page object in Path mode.
pub struct PagePathMode<'doc, 'page> {
    page: &'page Page<'doc>,
}

impl<'doc, 'page> PagePathMode<'doc, 'page> {
    pub(crate) fn new(page: &'page Page<'doc>) -> Self {
        Self { page }
    }
    
    /// Start a new subpath and move the current point for drawing path,
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

    /// Append a Bézier curve to the current path using three spesified points.
    pub fn curve_to<T1, T2, T3>(&self, point1: T1, point2: T2, point3: T3) -> anyhow::Result<()>
    where
        T1: Into<Point>,
        T2: Into<Point>,
        T3: Into<Point>,
    {
        let point1 = point1.into();
        let point2 = point2.into();
        let point3 = point3.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_CurveTo(self.page.handle(), point1.x, point1.y, point2.x, point2.y, point3.x, point3.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_CurveTo failed (status={})", status);
        }

        Ok(())
    }
    
    /// Append a Bézier curve to the current path using two spesified points.
    pub fn curve_to_2<T1, T2>(&self, point2: T1, point3: T2) -> anyhow::Result<()>
    where
        T1: Into<Point>,
        T2: Into<Point>,
    {
        let point2 = point2.into();
        let point3 = point3.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_CurveTo2(self.page.handle(), point2.x, point2.y, point3.x, point3.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_CurveTo2 failed (status={})", status);
        }

        Ok(())
    }

    /// Append a Bézier curve to the current path using two spesified points.
    pub fn curve_to_3<T1, T2>(&self, point1: T1, point3: T2) -> anyhow::Result<()>
    where
        T1: Into<Point>,
        T2: Into<Point>,
    {
        let point1 = point1.into();
        let point3 = point3.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_CurveTo3(self.page.handle(), point1.x, point1.y, point3.x, point3.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_CurveTo3 failed (status={})", status);
        }

        Ok(())
    }

    /// Append a path from the current point to the specified point.
    pub fn line_to<T>(&self,  pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>,
    {
        let pos = pos.into();
        let status = unsafe {
            libharu_sys::HPDF_Page_LineTo(self.page.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_LineTo failed (status={})", status);
        }

        Ok(())
    }

    /// Append a rectangle to the current path.
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

    /// Append a circle to the current path.
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

    /// Append a arc to the current path.
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

    /// Paint the current path.
    pub fn stroke(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Stroke(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Stroke failed (status={})", status);
        }

        Ok(())
    }

    /// Fill the current path using the nonzero winding number rule.
    pub fn fill(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Fill(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Fill failed (status={})", status);
        }

        Ok(())
    }

    /// Fill the current path using the nonzero winding number rule, then paint the current path.
    pub fn fill_stroke(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_FillStroke(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_FillStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Paint the current path and set clipping region.
    pub fn clip(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Clip(self.page.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Clip failed (status={})", status);
        }

        Ok(())
    }


}

impl<'doc, 'page> Deref for PagePathMode<'doc, 'page> {
    type Target = Page<'doc>;
    fn deref(&self) -> &Self::Target {
        self.page
    }
}

impl<'doc, 'page> PageDescPathCommonFunction<'doc> for PagePathMode<'doc, 'page> {
    fn handle(&self) -> &Page {
        self.page
    }
}
