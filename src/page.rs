use crate::document::Document;
use crate::destination::Destination;
use crate::{Real, Font, Color, Point};

use std::ffi::CString;

/// The style of line-cap.
#[derive(Debug)]
pub enum LineCap {
    /// The line is squared off at the endpoint of the path.
    Butt,

    /// The end of a line becomes a semicircle whose center is the end point of the path.
    Round,

    /// The line continues to the point that exceeds half of the stroke width the end point.
    ProjectingSquare,
}

/// The style of line-join.
#[derive(Debug)]
pub enum LineJoin {
    /// HPDF_MITER_JOIN
    Miter,

    /// HPDF_ROUND_JOIN
    Round,

    /// HPDF_BEVEL_JOIN
    Bevel,
}

/// Text rendering mode
#[derive(Debug)]
pub enum TextRenderingMode {
    /// HPDF_FILL
    Fill,

    /// HPDF_STROKE
    Stroke,

    /// HPDF_FILL_THEN_STROKE
    FillThenStroke,

    /// HPDF_INVISIBLE
    Invisible,

    /// HPDF_FILL_CLIPPING
    FillClipping,

    /// HPDF_STROKE_CLIPPING
    StrokeClipping,

    /// HPDF_FILL_STROKE_CLIPPING
    FillStrokeClipping,

    /// CLIPPING
    Clipping,
}

/// Size of page.
#[derive(Debug)]
pub enum PageSize {
    /// 8½ x 11 (Inches), 612 x 792 (pixel)
    Letter,

    /// 8 ½ x 14 (Inches), 612 x 1008 (pixel)
    Legal,

    /// 297 × 420 (mm), 841.89 x 1199.551 (pixel)
    A3,

    /// 210 × 297 (mm), 595.276 x 841.89 (pixel)
    A4,

    /// 148 × 210 (mm), 419.528 x 595.276 (pixel)
    A5,

    /// 250 × 353 (mm), 708.661 x 1000.63 (pixel)
    B4,

    /// 176 × 250 (mm), 498.898 x 708.661 (pixel)
    B5,

    /// 7½ x 10½ (Inches), 522 x 756 (pixel)
    Executive,

    /// 4 x 6 (Inches), 288 x 432 (pixel)
    US4x6,

    /// 4 x 8 (Inches), 288 x 576 (pixel)
    US4x8,

    /// 5 x 7 (Inches), 360 x 504 (pixel)
    US5x7,

    /// 4.125 x 9.5 (Inches), 297x 684 (pixel)
    Comm10,
}

/// Direction of page.
#[derive(Debug)]
pub enum PageDirection {
    /// longer value to horizontal
    Portrait,

    /// longer value to vertical
    Landscape,
}

/// Page handle type.
pub struct Page<'a> {
    page: libharu_sys::HPDF_Page,
    doc: &'a Document,
}

impl<'a> Page<'a> {
    /// Construct new Page object.
    pub(crate) fn new(doc: &'a Document, page: libharu_sys::HPDF_Page) -> Self {
        Self { page, doc }
    }

    /// Get internal handle.
    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Page {
        self.page
    }

    /// Get height of page.
    pub fn height(&self) -> anyhow::Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Page_GetHeight(self.handle())
        };

        Ok(ret)
    }

    /// Set height of page.
    pub fn set_height(&self, val: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetHeight(self.handle(), val)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetHeight failed (status={})", status);
        }

        Ok(())
    }

    /// Get width of page.
    pub fn width(&self) -> anyhow::Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Page_GetWidth(self.handle())
        };

        Ok(ret)
    }

    /// Set width of page.
    pub fn set_width(&self, val: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWidth(self.handle(), val)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetWidth failed (status={})", status);
        }

        Ok(())
    }

    /// Get line width of page.
    pub fn line_width(&self) -> Real {
        unsafe {
            libharu_sys::HPDF_Page_GetLineWidth(self.handle())
        }
    }

    /// Set line width of page.
    pub fn set_line_width(&self, width: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineWidth(self.handle(), width)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineWidth failed (status={})", status);
        }

        Ok(())
    }

    /// Append a rectangle to the current path.
    pub fn rectangle(&self, x: Real, y: Real, width: Real, height: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Rectangle(self.handle(), x, y, width, height)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineWidth failed (status={})", status);
        }

        Ok(())
    }

    /// Paint the current path.
    pub fn stroke(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Stroke(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Stroke failed (status={})", status);
        }

        Ok(())
    }

    /// Fill the current path using the nonzero winding number rule.
    pub fn fill(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Fill(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Fill failed (status={})", status);
        }

        Ok(())
    }

    /// Fill the current path using the nonzero winding number rule, then paint the current path.
    pub fn fill_stroke(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_FillStroke(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_FillStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Paint the current path and set clipping region.
    pub fn clip(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_Clip(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_Clip failed (status={})", status);
        }

        Ok(())
    }

    /// Push the page's current graphics state to the stack.
    pub fn gsave(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_GSave(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_GSave failed (status={})", status);
        }

        Ok(())
    }

    /// Pop the graphics state from the stack.
    pub fn grestore(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_GRestore(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_GRestore failed (status={})", status);
        }

        Ok(())
    }

    /// Gets the handle of the page's current font.
    pub fn current_font(&self) -> anyhow::Result<Font> {
        let font = unsafe {
            libharu_sys::HPDF_Page_GetCurrentFont(self.handle())
        };

        if font == std::ptr::null_mut() {
            anyhow::bail!("HPDF_Page_GetCurrentFont failed");
        }

        Ok(Font::new(self.doc, font))
    }

    /// Gets the size of the page's current font.
    pub fn current_font_size(&self) -> anyhow::Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Page_GetCurrentFontSize(self.handle())
        };

        Ok(ret)
    }

    /// Set font and size.
    pub fn set_font_and_size(&self, font: &Font, size: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetFontAndSize(self.handle(), font.font, size)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetFontAndSize failed (status={})", status);
        }

        Ok(())
    }

    /// Set text leading
    pub fn set_text_leading(&self, val: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextLeading(self.handle(), val)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextLeading failed (status={})", status);
        }

        Ok(())
    }
    
    /// Get the current value of the page's line spacing.
    pub fn text_leading(&self) -> anyhow::Result<Real> {
        let leading = unsafe {
            libharu_sys::HPDF_Page_GetTextLeading(self.handle())
        };

        Ok(leading)

    }
    /// Get the width of the text in current fontsize, character spacing and word spacing.
    pub fn text_width(&self, txt: &str) -> anyhow::Result<Real> {
        let txt = CString::new(txt)?;
        let ret = unsafe {
            libharu_sys::HPDF_Page_TextWidth(self.handle(), std::mem::transmute(txt.as_ptr()))
        };

        Ok(ret)
    }

    /// Calculate the byte length which can be included within the specified width.
    pub fn measure_text(&self, text: &str, width: Real, wordwrap: bool) -> anyhow::Result<(usize, Real)> {
        let text = CString::new(text)?;
        let wordwrap = match wordwrap {
            true => 1,
            false => 0,
        };

        let mut real_width = 0.0;
        let ret = unsafe {
            libharu_sys::HPDF_Page_MeasureText(self.handle(), text.as_ptr() as *const i8, width, wordwrap, &mut real_width)
        };

        Ok((ret as usize, real_width))
    }

    /// Calculate the byte length which can be included within the specified width. (bytes data)
    pub fn measure_text_bytes(&self, text: &[u8], width: Real, wordwrap: bool) -> anyhow::Result<(usize, Real)> {
        let text = CString::new(text)?;
        let wordwrap = match wordwrap {
            true => 1,
            false => 0,
        };

        let mut real_width = 0.0;
        let ret = unsafe {
            libharu_sys::HPDF_Page_MeasureText(self.handle(), text.as_ptr() as *const i8, width, wordwrap, &mut real_width)
        };

        Ok((ret as usize, real_width))
    }

    /// Begin a text object and sets the current text position to the point (0, 0).
    pub fn begin_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_BeginText(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_BeginText failed (status={})", status);
        }

        Ok(())
    }

    /// End a text object.
    pub fn end_text(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_EndText(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text on the specified position.
    pub fn text_out(&self, xpos: Real, ypos: Real, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_TextOut(self.handle(), xpos, ypos, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text on the specified position. (bytes data)
    pub fn text_out_bytes(&self, xpos: Real, ypos: Real, text: &[u8]) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_TextOut(self.handle(), xpos, ypos, std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_TextOut failed (status={})", status);
        }

        Ok(())
    }

    /// Print the text at the current position on the page.
    pub fn show_text(&self, text: &str) -> anyhow::Result<()> {
        let text = CString::new(text)?;
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(self.handle(), std::mem::transmute(text.as_ptr()))
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
            libharu_sys::HPDF_Page_ShowText(self.handle(), std::mem::transmute(text.as_ptr()))
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
            libharu_sys::HPDF_Page_ShowTextNextLine(self.handle(), std::mem::transmute(text.as_ptr()))
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
            libharu_sys::HPDF_Page_ShowTextNextLine(self.handle(), std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_ShowTextNextLine failed (status={})", status);
        }

        Ok(())
    }
    
    /// Move the current text position to the start of the next line with using specified offset values.
    pub fn move_text_pos<T>(&self, pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>,
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_MoveTextPos(self.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveTextPos failed (status={})", status);
        }

        Ok(())
    }
    
    /// Get the current position for text showing.
    pub fn current_text_pos(&self) -> anyhow::Result<Point> {
        let point = unsafe {
            libharu_sys::HPDF_Page_GetCurrentTextPos(self.handle())
        };

        Ok(Point{x:point.x, y:point.y})
    }

    /// Start a new subpath and move the current point for drawing path,
    pub fn move_to<T>(&self, pos: T) -> anyhow::Result<()>
    where
        T: Into<Point>,
    {
        let pos = pos.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_MoveTo(self.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_MoveTo failed (status={})", status);
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
            libharu_sys::HPDF_Page_LineTo(self.handle(), pos.x, pos.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_LineTo failed (status={})", status);
        }

        Ok(())
    }

    /// Set the line dash pattern in the page.
    pub fn set_dash(&self, dash_mode: &[u16], phase: usize) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetDash(self.handle(), dash_mode.as_ptr(), dash_mode.len() as u32, phase as u32)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetDash failed (status={})", status);
        }

        Ok(())
    }

    /// Clear the line dash pattern in the page.
    pub fn clear_dash(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetDash(self.handle(), std::ptr::null_mut(), 0, 0)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetDash failed (status={})", status);
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
            libharu_sys::HPDF_Page_SetRGBStroke(self.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set the filling color.
    pub fn set_gray_fill(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayFill(self.handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayFill failed (status={})", status);
        }

        Ok(())
    }

    /// Set the stroking color.
    pub fn set_gray_stroke(&self, gray: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayStroke(self.handle(), gray)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetGrayStroke failed (status={})", status);
        }

        Ok(())
    }

    /// Set the shape to be used at the ends of line.
    pub fn set_line_cap(&self, line_cap: LineCap) -> anyhow::Result<()> {
        let line_cap = match line_cap {
            LineCap::Butt => libharu_sys::HPDF_LineCap::HPDF_BUTT_END,
            LineCap::Round => libharu_sys::HPDF_LineCap::HPDF_ROUND_END,
            LineCap::ProjectingSquare => libharu_sys::HPDF_LineCap::HPDF_PROJECTING_SCUARE_END,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineCap(self.handle(), line_cap)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineCap failed (status={})", status);
        }

        Ok(())
    }

    /// Set the line join style in the page.
    pub fn set_line_join(&self, line_join: LineJoin) -> anyhow::Result<()> {
        let line_join = match line_join {
            LineJoin::Miter => libharu_sys::HPDF_LineJoin::HPDF_MITER_JOIN,
            LineJoin::Round => libharu_sys::HPDF_LineJoin::HPDF_ROUND_JOIN,
            LineJoin::Bevel => libharu_sys::HPDF_LineJoin::HPDF_BEVEL_JOIN,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineJoin(self.handle(), line_join)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetLineJoin failed (status={})", status);
        }

        Ok(())
    }

    /// Get current value of the page's filling color
    pub fn rgb_fill(&self) -> anyhow::Result<Color> {
        let c = unsafe {
            libharu_sys::HPDF_Page_GetRGBFill(self.handle())
        };

        Ok(Color{ red: c.r, green: c.g, blue: c.b })
    }

    /// Set filling color.
    pub fn set_rgb_fill<T>(&self, color: T) -> anyhow::Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBFill(self.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRGBFill failed (status={})", status);
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
            libharu_sys::HPDF_Page_CurveTo(self.handle(), point1.x, point1.y, point2.x, point2.y, point3.x, point3.y)
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
            libharu_sys::HPDF_Page_CurveTo2(self.handle(), point2.x, point2.y, point3.x, point3.y)
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
            libharu_sys::HPDF_Page_CurveTo3(self.handle(), point1.x, point1.y, point3.x, point3.y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_CurveTo3 failed (status={})", status);
        }

        Ok(())
    }

    /// Set text affine transformation matrix.
    pub fn set_text_matrix(&self, a: Real, b: Real, c: Real, d: Real, x: Real, y: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextMatrix(self.handle(), a, b, c, d, x, y)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextRenderingMode failed (status={})", status);
        }

        Ok(())
    }

    /// Sets the text rendering mode.
    pub fn set_text_rendering_mode(&self, mode: TextRenderingMode) -> anyhow::Result<()> {
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
            libharu_sys::HPDF_Page_SetTextRenderingMode(self.handle(), mode)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetTextRenderingMode failed (status={})", status);
        }

        Ok(())
    }
    
    /// Set the character spacing for text showing.
    pub fn set_char_space(&self, val: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCharSpace(self.handle(), val)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetCharSpace failed (status={})", status);
        }

        Ok(())
    }

    /// Set the word spacing for text showing.
    pub fn set_word_space(&self, val: Real) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWordSpace(self.handle(), val)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetWordSpace failed (status={})", status);
        }

        Ok(())
    }

    /// Create a new destination object for the page.
    pub fn create_destination(&self) -> anyhow::Result<Destination> {
        let dst = unsafe {
            libharu_sys::HPDF_Page_CreateDestination(self.handle())
        };

        if dst == std::ptr::null_mut() {
            anyhow::bail!("HPDF_Page_CreateDestination failed");
        }

        Ok(Destination::new(self, dst))
    }
    
    /// Get the current position for path painting.
    pub fn current_pos(&self) -> anyhow::Result<Point> {
        let point = unsafe {
            libharu_sys::HPDF_Page_GetCurrentPos(self.handle())
        };

        Ok(Point{ x: point.x, y: point.y })
    }

    /// Set the size and direction of a page to a predefined size.
    pub fn set_size(&self, size: PageSize, direction: PageDirection) -> anyhow::Result<()> {
        let size = match size {
            PageSize::Letter => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_LETTER,
            PageSize::Legal => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_LEGAL,
            PageSize::A3 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_A3,
            PageSize::A4 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_A4,
            PageSize::A5 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_A5,
            PageSize::B4 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_B4,
            PageSize::B5 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_B5,
            PageSize::Executive => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_EXECUTIVE,
            PageSize::US4x6 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_US4x6,
            PageSize::US4x8 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_US4x8,
            PageSize::US5x7 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_US5x7,
            PageSize::Comm10 => libharu_sys::HPDF_PageSizes::HPDF_PAGE_SIZE_COMM10,
        };
        
        let direction = match direction {
            PageDirection::Portrait => libharu_sys::HPDF_PageDirection::HPDF_PAGE_PORTRAIT,
            PageDirection::Landscape => libharu_sys::HPDF_PageDirection::HPDF_PAGE_LANDSCAPE,
        };

        let status = unsafe {
            libharu_sys::HPDF_Page_SetSize(self.handle(), size, direction)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetSize failed (status={})", status);
        }

        Ok(())
    }

    /// Set rotation angle of the page.
    pub fn set_rotate(&self, angle: u16) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRotate(self.handle(), angle)
        };

        if status != 0 {
            anyhow::bail!("HPDF_Page_SetRotate failed (status={})", status);
        }

        Ok(())
    }
}
