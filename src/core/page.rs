use super::{
    Real,
    Result,
    Document,
    Destination,
    Font,
    Point,
    Color,
    CmykColor,
};

use std::ffi::CStr;

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

impl From<libharu_sys::HPDF_LineCap> for LineCap {
    fn from(v: libharu_sys::HPDF_LineCap) -> Self {
        use libharu_sys::HPDF_LineCap::*;
        match v {
            HPDF_BUTT_END => Self::Butt,
            HPDF_ROUND_END => Self::Round,
            HPDF_PROJECTING_SCUARE_END => Self::ProjectingSquare,
            HPDF_LINECAP_EOF => Self::Butt,
        }
    }
}

impl From<LineCap> for libharu_sys::HPDF_LineCap {
    fn from(v: LineCap) -> Self {
        use libharu_sys::HPDF_LineCap::*;
        match v {
            LineCap::Butt => HPDF_BUTT_END,
            LineCap::Round => HPDF_ROUND_END,
            LineCap::ProjectingSquare => HPDF_PROJECTING_SCUARE_END,
        }
    }
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

impl From<libharu_sys::HPDF_LineJoin> for LineJoin {
    fn from(v: libharu_sys::HPDF_LineJoin) -> Self {
        use libharu_sys::HPDF_LineJoin::*;
        match v {
            HPDF_MITER_JOIN => LineJoin::Miter,
            HPDF_ROUND_JOIN => LineJoin::Round,
            HPDF_BEVEL_JOIN => LineJoin::Bevel,
            HPDF_LINEJOIN_EOF => LineJoin::Miter,
        }
    }
}

impl From<LineJoin> for libharu_sys::HPDF_LineJoin {
    fn from(v: LineJoin) -> Self {
        use libharu_sys::HPDF_LineJoin::*;
        match v {
            LineJoin::Miter => HPDF_MITER_JOIN,
            LineJoin::Round => HPDF_ROUND_JOIN,
            LineJoin::Bevel => HPDF_BEVEL_JOIN,
        }
    }
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

impl From<libharu_sys::HPDF_TextRenderingMode> for TextRenderingMode {
    fn from(v: libharu_sys::HPDF_TextRenderingMode) -> Self {
        use libharu_sys::HPDF_TextRenderingMode::*;
        match v {
            HPDF_FILL => Self::Fill,
            HPDF_STROKE => Self::Stroke,
            HPDF_FILL_THEN_STROKE => Self::FillThenStroke,
            HPDF_INVISIBLE => Self::Invisible,
            HPDF_FILL_CLIPPING => Self::FillClipping,
            HPDF_STROKE_CLIPPING => Self::StrokeClipping,
            HPDF_FILL_STROKE_CLIPPING => Self::FillStrokeClipping,
            HPDF_CLIPPING => Self::Clipping,
            HPDF_RENDERING_MODE_EOF => Self::Fill,
        }
    }
}

impl From<TextRenderingMode> for libharu_sys::HPDF_TextRenderingMode {
    fn from(v: TextRenderingMode) -> Self {
        use libharu_sys::HPDF_TextRenderingMode::*;
        match v {
            TextRenderingMode::Fill => HPDF_FILL,
            TextRenderingMode::Stroke => HPDF_STROKE,
            TextRenderingMode::FillThenStroke => HPDF_FILL_THEN_STROKE,
            TextRenderingMode::Invisible => HPDF_INVISIBLE,
            TextRenderingMode::FillClipping => HPDF_FILL_CLIPPING,
            TextRenderingMode::StrokeClipping => HPDF_STROKE_CLIPPING,
            TextRenderingMode::FillStrokeClipping => HPDF_FILL_STROKE_CLIPPING,
            TextRenderingMode::Clipping => HPDF_CLIPPING,
        }
    }
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

impl From<libharu_sys::HPDF_PageSizes> for PageSize {
    fn from(v: libharu_sys::HPDF_PageSizes) -> Self {
        use libharu_sys::HPDF_PageSizes::*;
        match v {
            HPDF_PAGE_SIZE_LETTER => Self::Letter,
            HPDF_PAGE_SIZE_LEGAL => Self::Legal,
            HPDF_PAGE_SIZE_A3 => Self::A3,
            HPDF_PAGE_SIZE_A4 => Self::A4,
            HPDF_PAGE_SIZE_A5 => Self::A5,
            HPDF_PAGE_SIZE_B4 => Self::B4,
            HPDF_PAGE_SIZE_B5 => Self::B5,
            HPDF_PAGE_SIZE_EXECUTIVE => Self::Executive,
            HPDF_PAGE_SIZE_US4x6 => Self::US4x6,
            HPDF_PAGE_SIZE_US4x8 => Self::US4x8,
            HPDF_PAGE_SIZE_US5x7 => Self::US5x7,
            HPDF_PAGE_SIZE_COMM10 => Self::Comm10,
            HPDF_PAGE_SIZE_EOF => Self::Letter,
        }
    }
}

impl From<PageSize> for libharu_sys::HPDF_PageSizes {
    fn from(v: PageSize) -> Self {
        use libharu_sys::HPDF_PageSizes::*;
        match v {
            PageSize::Letter => HPDF_PAGE_SIZE_LETTER,
            PageSize::Legal => HPDF_PAGE_SIZE_LEGAL,
            PageSize::A3 => HPDF_PAGE_SIZE_A3,
            PageSize::A4 => HPDF_PAGE_SIZE_A4,
            PageSize::A5 => HPDF_PAGE_SIZE_A5,
            PageSize::B4 => HPDF_PAGE_SIZE_B4,
            PageSize::B5 => HPDF_PAGE_SIZE_B5,
            PageSize::Executive => HPDF_PAGE_SIZE_EXECUTIVE,
            PageSize::US4x6 => HPDF_PAGE_SIZE_US4x6,
            PageSize::US4x8 => HPDF_PAGE_SIZE_US4x8,
            PageSize::US5x7 => HPDF_PAGE_SIZE_US5x7,
            PageSize::Comm10 => HPDF_PAGE_SIZE_COMM10,
        }
    }
}

/// Direction of page.
#[derive(Debug)]
pub enum PageDirection {
    /// longer value to horizontal
    Portrait,

    /// longer value to vertical
    Landscape,
}

impl From<libharu_sys::HPDF_PageDirection> for PageDirection {
    fn from(v: libharu_sys::HPDF_PageDirection) -> Self {
        use libharu_sys::HPDF_PageDirection::*;
        match v {
            HPDF_PAGE_PORTRAIT => Self::Portrait,
            HPDF_PAGE_LANDSCAPE => Self::Landscape,
        }
    }
}

impl From<PageDirection> for libharu_sys::HPDF_PageDirection {
    fn from(v: PageDirection) -> Self {
        use libharu_sys::HPDF_PageDirection::*;
        match v {
            PageDirection::Portrait => HPDF_PAGE_PORTRAIT,
            PageDirection::Landscape => HPDF_PAGE_LANDSCAPE,
        }
    }
}

/// Text alignment.
#[derive(Debug)]
pub enum TextAlignment {
    /// Left alignment
    Left,

    /// Right alignment
    Right,

    /// Center alignment
    Center,

    /// Justify alignment
    Justify,
}

pub(crate) mod private {
    pub trait PageHandle<'doc> {
        /// Get libharu Page handle
        fn handle(&self) -> libharu_sys::HPDF_Page;
    }
}

pub trait Page<'doc> : private::PageHandle<'doc> {
    /// Get Document object
    fn doc(&self) -> &'doc Document;

    /// Get the width of page.
    fn width(&self) -> Result<Real> {
        let width = unsafe {
            libharu_sys::HPDF_Page_GetWidth(self.handle())
        };

        if width == 0.0 {
            Err(self.doc().last_error())
        }
        else {
            Ok(width)
        }
    }

    /// Set the width of page.
    fn set_width(&self, width: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWidth(self.handle(), width)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Get the height of page.
    fn height(&self) -> Result<Real> {
        let height = unsafe {
            libharu_sys::HPDF_Page_GetHeight(self.handle())
        };

        if height == 0.0 {
            Err(self.doc().last_error())
        }
        else {
            Ok(height)
        }
    }

    /// Set the height of page.
    fn set_height(&self, height: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWidth(self.handle(), height)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Get line width of page.
    fn line_width(&self) -> Result<Real> {
        let line_width = unsafe {
            libharu_sys::HPDF_Page_GetLineWidth(self.handle())
        };

        if line_width == 0.0 {
            Err(self.doc().last_error())
        }
        else {
            Ok(line_width)
        }
    }

    /// Set line width of page.
    fn set_line_width(&self, line_width: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineWidth(self.handle(), line_width)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Push the page's current graphics state to the stack.
    fn gsave(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_GSave(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Pop the graphics state from the stack.
    fn grestore(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_GRestore(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Gets the handle of the page's current font.
    fn current_font(&self) -> Result<Font<'doc>> {
        let font = unsafe {
            libharu_sys::HPDF_Page_GetCurrentFont(self.handle())
        };

        if font == std::ptr::null_mut() {
            Err(self.doc().last_error())
        }
        else {
            Ok(Font::new(self.doc(), font))
        }
    }

    /// Gets the size of the page's current font.
    fn current_font_size(&self) -> Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Page_GetCurrentFontSize(self.handle())
        };

        if ret == 0.0 {
            Err(self.doc().last_error())
        }
        else {
            Ok(ret)
        }
    }

    /// Get the width of the text in current fontsize, character spacing and word spacing.
    fn text_width(&self, txt: &CStr) -> Result<Real> {
        let ret = unsafe {
            libharu_sys::HPDF_Page_TextWidth(self.handle(), txt.as_ptr())
        };

        Ok(ret)
    }

    /// Calculate the byte length which can be included within the specified width.
    fn measure_text(&self, text: &CStr, width: Real, wordwrap: bool) -> Result<(usize, Real)> {
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
    fn measure_text_bytes(&self, text: &[u8], width: Real, wordwrap: bool) -> Result<(usize, Real)> {
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
    fn current_text_pos(&self) -> Result<Point> {
        let point = unsafe {
            libharu_sys::HPDF_Page_GetCurrentTextPos(self.handle())
        };

        Ok(Point{x:point.x, y:point.y})
    }

    /// Clear the line dash pattern in the page.
    fn clear_dash(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetDash(self.handle(), std::ptr::null_mut(), 0, 0)
        };

        if status != 0 {
            Err(self.doc().last_error())
        }
        else {
            Ok(())
        }
    }

    /// Get current value of the page's filling color
    fn rgb_fill(&self) -> Result<Color> {
        let c = unsafe {
            libharu_sys::HPDF_Page_GetRGBFill(self.handle())
        };

        Ok(Color{ red: c.r, green: c.g, blue: c.b })
    }

    /// Create a new destination object for the page.
    fn create_destination(&self) -> Result<Destination<'doc>> {
        let dst = unsafe {
            libharu_sys::HPDF_Page_CreateDestination(self.handle())
        };

        if dst == std::ptr::null_mut() {
            Err(self.doc().last_error())
        }
        else {
            Ok(Destination::new(self.doc(), dst))
        }
    }
    
    /// Get the current position for path painting.
    fn current_pos(&self) -> Result<Point> {
        let point = unsafe {
            libharu_sys::HPDF_Page_GetCurrentPos(self.handle())
        };

        Ok(Point{ x: point.x, y: point.y })
    }

    /// Set the size and direction of a page to a predefined size.
    fn set_size(&self, size: PageSize, direction: PageDirection) -> Result<()> {
        let size = size.into();
        let direction = direction.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetSize(self.handle(), size, direction)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set rotation angle of the page.
    fn set_rotate(&self, angle: u16) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRotate(self.handle(), angle)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }


}

pub trait PageDescTextCommonFunctionCStr<'doc> : Page<'doc> {

    /// Print the text at the current position on the page.
    fn show_text(&self, text: &CStr) -> Result<()> {
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

    /// Move the current text position to the start of the next line,
    fn show_text_next_line(&self, text: &CStr) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(
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

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page.
    fn show_text_next_line_ex(&self, word_space: Real, char_space: Real, text: &CStr) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(
                self.handle(),
                word_space,
                char_space,
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

pub trait PageDescTextCommonFunction<'doc> : Page<'doc> {
    /// Set the shape to be used at the ends of line.
    fn set_line_cap(&self, line_cap: LineCap) -> Result<()> {
        let line_cap = line_cap.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineCap(self.handle(), line_cap)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the line join style in the page.
    fn set_line_join(&self, line_join: LineJoin) -> Result<()> {
        let line_join = line_join.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetLineJoin(self.handle(), line_join)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the line dash pattern in the page.
    fn set_dash(&self, dash_mode: &[u16], phase: usize) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetDash(self.handle(), dash_mode.as_ptr(), dash_mode.len() as u32, phase as u32)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the character spacing for text showing.
    fn set_char_space(&self, value: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCharSpace(self.handle(), value)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the word spacing for text showing.
    fn set_word_space(&self, value: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetWordSpace(self.handle(), value)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
    
    /// Set the horizontal scalling for text showing.
    fn set_horizontal_scalling(&self, value: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetHorizontalScalling(self.handle(), value)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set text leading
    fn set_text_leading(&self, value: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextLeading(self.handle(), value)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set font and size.
    fn set_font_and_size(&self, font: &Font, size: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetFontAndSize(self.handle(), font.handle(), size)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Sets the text rendering mode.
    fn set_text_rendering_mode(&self, mode: TextRenderingMode) -> Result<()> {
        let mode = mode.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetTextRenderingMode(self.handle(), mode)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Print the text at the current position on the page. (bytes data)
    fn show_text_bytes(&self, text: &[u8]) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowText(
                self.handle(),
                std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Move the current text position to the start of the next line, (bytes data)
    fn show_text_next_line_bytes(&self, text: &[u8]) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLine(
                self.handle(),
                std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Moves the current text position to the start of the next line, then sets the word spacing,
    /// character spacing and prints the text at the current position on the page. (bytes data)
    fn show_text_next_line_ex_bytes(&self, word_space: Real, char_space: Real, text: &[u8]) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_ShowTextNextLineEx(
                self.handle(),
                word_space,
                char_space,
                std::mem::transmute(text.as_ptr()))
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the filling color.
    fn set_gray_fill(&self, gray: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayFill(self.handle(), gray)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the stroking color.
    fn set_gray_stroke(&self, gray: Real) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_Page_SetGrayStroke(self.handle(), gray)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set filling color.
    fn set_rgb_fill<T>(&self, color: T) -> Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBFill(self.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the stroking color.
    fn set_rgb_stroke<T>(&self, color: T) -> Result<()>
    where
        T: Into<Color>
    {
        let color = color.into();

        let status = unsafe {
            libharu_sys::HPDF_Page_SetRGBStroke(self.handle(), color.red, color.green, color.blue)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the filling color.
    fn set_cmyk_fill<T>(&self, color: T) -> Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKFill(self.handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// Set the stroking color.
    fn set_cmyk_stroke<T>(&self, color: T) -> Result<()>
    where
        T: Into<CmykColor>
    {
        let color = color.into();
        
        let status = unsafe {
            libharu_sys::HPDF_Page_SetCMYKStroke(self.handle(), color.cyan, color.magenta, color.yellow, color.keyplate)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
}

pub trait PageDescPathCommonFunction<'doc> : Page<'doc> {
}

pub struct PageDescriptionMode<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Page,
}

impl<'doc> PageDescriptionMode<'doc> {
    #[inline]
    pub(crate) fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Page) -> Self {
        Self { doc, handle }
    }

    fn run_text_mode<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&PageTextMode) -> Result<()>
    {
        let status = unsafe {
            libharu_sys::HPDF_Page_BeginText(self.handle)
        };
        if status != 0 {
            return Err(status.into());
        }

        let page = PageTextMode::new(self.doc, self.handle);
        let ret = f(&page);

        let status = unsafe {
            libharu_sys::HPDF_Page_EndText(self.handle)
        };
        if status != 0 {
            return Err(status.into());
        }

        ret
    }
}

impl<'doc> private::PageHandle<'doc> for PageDescriptionMode<'doc> {
    fn handle(&self) -> libharu_sys::HPDF_Page {
        self.handle
    }
}

impl<'doc> Page<'doc> for PageDescriptionMode<'doc> {
    fn doc(&self) -> &'doc Document {
        self.doc
    }
}

//impl<'doc> PageDescTextCommonFunctionCStr<'doc> for PageDescriptionMode<'doc> {}
impl<'doc> PageDescTextCommonFunction<'doc> for PageDescriptionMode<'doc> {}
impl<'doc> PageDescPathCommonFunction<'doc> for PageDescriptionMode<'doc> {}




pub struct PageTextMode<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Page,
}

impl<'doc> PageTextMode<'doc> {
    #[inline]
    pub(crate) fn new(doc: &'doc Document, handle: libharu_sys::HPDF_Page) -> Self {
        Self { doc, handle }
    }
}

impl<'doc> private::PageHandle<'doc> for PageTextMode<'doc> {
    fn handle(&self) -> libharu_sys::HPDF_Page {
        self.handle
    }
}

impl<'doc> Page<'doc> for PageTextMode<'doc> {
    fn doc(&self) -> &'doc Document {
        self.doc
    }
}

//impl<'doc> PageDescTextCommonFunctionCStr<'doc> for PageTextMode<'doc> {}
impl<'doc> PageDescTextCommonFunction<'doc> for PageTextMode<'doc> {}




pub struct PagePathMode<'doc> {
    doc: &'doc Document,
    handle: libharu_sys::HPDF_Page,
}

impl<'doc> private::PageHandle<'doc> for PagePathMode<'doc> {
    fn handle(&self) -> libharu_sys::HPDF_Page {
        self.handle
    }
}
impl<'doc> Page<'doc> for PagePathMode<'doc> {
    fn doc(&self) -> &'doc Document {
        self.doc
    }
}

impl<'doc> PageDescPathCommonFunction<'doc> for PageTextMode<'doc> {}
