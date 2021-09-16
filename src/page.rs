use crate::prelude::*;

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

#[derive(Debug)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
    Justify,
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

    /// Get the current position for text showing.
    pub fn current_text_pos(&self) -> anyhow::Result<Point> {
        let point = unsafe {
            libharu_sys::HPDF_Page_GetCurrentTextPos(self.handle())
        };

        Ok(Point{x:point.x, y:point.y})
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

    /// Get current value of the page's filling color
    pub fn rgb_fill(&self) -> anyhow::Result<Color> {
        let c = unsafe {
            libharu_sys::HPDF_Page_GetRGBFill(self.handle())
        };

        Ok(Color{ red: c.r, green: c.g, blue: c.b })
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
