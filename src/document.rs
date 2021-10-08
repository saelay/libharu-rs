#![warn(missing_docs)]

use crate::error::Error;
use crate::page::Page;
use crate::outline::Outline;
use crate::Font;
use crate::encoder::Encoder;

use bitflags::bitflags;

use std::ffi::CString;
use std::convert::TryInto;

/// Page label style.
#[derive(Debug)]
pub enum PageNumStyle {
    /// Page label is displayed by Arabic numerals.
    Decimal,
    
    /// Page label is displayed by Uppercase roman numerals.
    UpperRoman,

    /// Page label is displayed by Lowercase roman numerals.
    LowerRoman,

    /// Page label is displayed by Uppercase letters (using A to Z).
    UpperLetters,

    /// Page label is displayed by Lowercase letters (using a to Z).
    LowerLetters,
}

bitflags! {
    /// The flags specifying which type of contents should be compressed.
    pub struct CompressionMode: u32 {
        /// All contents are not compressed.
        const NONE     = 0x00;

        /// Compress the contents stream of the page.
        const TEXT     = 0x01;

        /// Compress the streams of the image objects.
        const IMAGE    = 0x02;

        /// Other stream datas (fonts, cmaps and so on) are compressed.
        const METADATA = 0x04;

        /// All stream datas are compressed. (The same as `CompressionMode::Text | CompressionMode::Image | CompressionMode::Metadata`)
        const ALL = Self::TEXT.bits | Self::IMAGE.bits | Self::METADATA.bits;
    }
}

/// Page display style.
#[derive(Debug)]
pub enum PageMode {
    /// Display the document with neither outline nor thumbnail.
    None,

    /// Display the document with outline pain.
    Outline,

    /// Display the document with thumbnail pain.
    Thumbs,
    
    /// Display the document with full screen mode.
    FullScreen,
}


/// Page layout style.
#[derive(Debug)]
pub enum PageLayout {
    /// Only one page is displayed.
    Single,
    
    /// Display the pages in one column.
    OneColumn,
    
    /// Display the pages in two column. The page of the odd number is displayed left.
    TwoColumnLeft,
    
    /// Display the pages in two column. The page of the odd number is displayed right.
    TwoColumnRight,
}
// onerrorのクロージャをBoxで持ちたいためInnerを別にしている。
// TODO: onerrorは必要か？
struct DocumentInner {
    onerror: Box<dyn Fn(Error)>,
    last_errno: libharu_sys::HPDF_STATUS,
    last_detailno: libharu_sys::HPDF_STATUS,
}

/// PDF Document handle type.
/// 
/// The document handle is a handle to operate a document object. 
pub struct Document {
    doc: libharu_sys::HPDF_Doc,

    #[allow(dead_code)]
    inner: Box<DocumentInner>,
}

extern "C" {
    fn HPDF_UseUTFEncodings(doc: libharu_sys::HPDF_Doc) -> libharu_sys::HPDF_STATUS;
}

impl Document {
    /// Create a new instance of document.
    pub fn new(onerror: impl Fn(Error) + 'static) -> anyhow::Result<Self>
    {
        let onerror = Box::new(onerror);
        let mut inner = Box::new(DocumentInner{onerror, last_errno: 0, last_detailno: 0});

        let doc = unsafe {
            libharu_sys::HPDF_New(
                onerror_callback,
                std::mem::transmute(inner.as_mut()),
            )
        };
        
        if doc == std::ptr::null_mut() {
            anyhow::bail!("HPDF_New() failed");
        }

        Ok(Self { doc, inner })
    }

    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Doc {
        self.doc
    }

    /// Create a new page and adds it after the last page of a document.
    pub fn add_page(&self) -> anyhow::Result<Page> {
        let page = unsafe {
            libharu_sys::HPDF_AddPage(self.handle())
        };

        if page == std::ptr::null_mut() {
            anyhow::bail!("HPDF_AddPage failed");
        }

        Ok(Page::new(self, page))
    }

    /// Set how the document should be displayed.
    pub fn set_page_mode(&self, mode: PageMode) -> anyhow::Result<()> {
        let mode = match mode {
            PageMode::None => libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_NONE,
            PageMode::Outline => libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_OUTLINE,
            PageMode::Thumbs => libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_THUMBS,
            PageMode::FullScreen => libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_FULL_SCREEN,
        };

        let status = unsafe {
            libharu_sys::HPDF_SetPageMode(self.handle(), mode)
        };

        if status != 0 {
            anyhow::bail!("HPDF_SetPageMode failed (status={})", status);
        }

        Ok(())
    }

    /// Get how the document should be displayed.
    pub fn page_mode(&self) -> anyhow::Result<PageMode> {
        let mode = unsafe {
            libharu_sys::HPDF_GetPageMode(self.handle())
        };

        let mode = match mode {
            libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_NONE => PageMode::None,
            libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_OUTLINE => PageMode::Outline,
            libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_USE_THUMBS => PageMode::Thumbs,
            libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_FULL_SCREEN => PageMode::FullScreen,
            _ => {
                anyhow::bail!("HPDF_GetPageMode failed");
            }
        };

        Ok(mode)
    }

    /// Create a new page and inserts it just before the specified page.
    pub fn insert_page(&self, target: &Page) -> anyhow::Result<Page> {
        let page = unsafe {
            libharu_sys::HPDF_InsertPage(self.handle(), target.handle())
        };

        if page == std::ptr::null_mut() {
            anyhow::bail!("HPDF_InsertPage failed");
        }

        Ok(Page::new(self, page))
    }

    /// Gets the handle of a corresponding font object by specified name and encoding.
    pub fn font(&self, font_name: &str, encoding_name: Option<&str>) -> anyhow::Result<Font> {
        let font_name = CString::new(font_name)?;
        let encoding_name = match encoding_name {
            Some(s) => Some(CString::new(s)?),
            None => None,
        };

        let font = unsafe {
            libharu_sys::HPDF_GetFont(self.handle(),
                std::mem::transmute(font_name.as_ptr()),
                match encoding_name {
                    Some(ref s) => std::mem::transmute(s.as_ptr()),
                    None => std::ptr::null_mut(),
                })
        };

        if font == std::ptr::null_mut() {
            anyhow::bail!("HPDF_GetFont failed");
        }

        Ok(Font::new(self, font))
    }

    /// Add a page labeling range for the document.
    pub fn add_page_label(&self, page_num: usize, style: PageNumStyle, first_page: usize, prefix: Option<&str>) -> anyhow::Result<()> {
        let style = match style {
            PageNumStyle::Decimal => libharu_sys::HPDF_PageNumStyle::HPDF_PAGE_NUM_STYLE_DECIMAL,
            PageNumStyle::UpperRoman => libharu_sys::HPDF_PageNumStyle::HPDF_PAGE_NUM_STYLE_UPPER_ROMAN,
            PageNumStyle::LowerRoman => libharu_sys::HPDF_PageNumStyle::HPDF_PAGE_NUM_STYLE_LOWER_ROMAN,
            PageNumStyle::UpperLetters => libharu_sys::HPDF_PageNumStyle::HPDF_PAGE_NUM_STYLE_UPPER_LETTERS,
            PageNumStyle::LowerLetters => libharu_sys::HPDF_PageNumStyle::HPDF_PAGE_NUM_STYLE_LOWER_LETTERS,
        };

        let page_num = page_num.try_into()?;
        let first_page = first_page.try_into()?;

        let prefix = match prefix {
            Some(s) => CString::new(s)?,
            None => CString::new("")?,
        };
        let status = unsafe {
            libharu_sys::HPDF_AddPageLabel(self.handle(), page_num, style, first_page, std::mem::transmute(prefix.as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_AddPageLabelf failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable Japanese fonts. After the method invoked, an application can use the following Japanese fonts.
    /// * MS-mincho
    /// * MS-mincho,Bold
    /// * MS-mincho,Bold
    /// * MS-mincho,Italic
    /// * MS-mincho,BoldItalic
    /// * MS-Gothic
    /// * MS-Gothic,Bold
    /// * MS-Gothic,Italic
    /// * MS-Gothic,BoldItalic
    /// * MS-Pmincho
    /// * MS-Pmincho,Bold
    /// * MS-Pmincho,Italic
    /// * MS-Pmincho,BoldItalic
    /// * MS-PGothic
    /// * MS-PGothic,Bold
    /// * MS-PGothic,Italic
    /// * MS-PGothic,BoldItalic
    pub fn use_jpfonts(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseJPFonts(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseJPFonts failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable Korian fonts. After the method invoked, an application can use the following Korean fonts.
    /// * DotumChe
    /// * DotumChe,Bold
    /// * DotumChe,Italic
    /// * DotumChe,BoldItalic
    /// * Dotum
    /// * Dotum,Bold
    /// * Dotum,Italic
    /// * Dotum,BoldItalic
    /// * BatangChe
    /// * BatangChe,Bold
    /// * BatangChe,Italic
    /// * BatangChe,BoldItalic
    /// * Batang
    /// * Batang,Bold
    /// * Batang,Italic
    /// * Batang,BoldItalic
    pub fn use_krfonts(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseKRFonts(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseKRFonts failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable simplified Chinese fonts. After the method invoked, an application can use the following simplified Chinese fonts.
    /// * SimSun
    /// * SimSun,Bold
    /// * SimSun,Italic
    /// * SimSun,BoldItalic
    /// * SimHei
    /// * SimHei,Bold
    /// * SimHei,Italic
    /// * SimHei,BoldItalic
    pub fn use_cnsfonts(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNSFonts(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseCNSFonts failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable traditional Chinese fonts. After the method invoked, an application can use the following traditional Chinese fonts.
    /// * MingLiU
    /// * MingLiU,Bold
    /// * MingLiU,Italic
    /// * MingLiU,BoldItalic
    pub fn use_cntfonts(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNTFonts(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseCNTFonts failed (status = {})", status);
        }

        Ok(())
    }
    
    /// Enable Japanese encodings. After the method invoked, an application can use the following Japanese encodings.
    /// * 90ms-RKSJ-H
    /// * 90ms-RKSJ-V
    /// * 90msp-RKSJ-H
    /// * EUC-H
    /// * EUC-V
    pub fn use_jpencodings(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseJPEncodings(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseJPEncodings failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable Korean encodings. After the method is invoked, an application can use the following Korean encodings.
    /// * KSC-EUC-H
    /// * KSC-EUC-V
    /// * KSCms-UHC-H
    /// * KSCms-UHC-HW-H
    /// * KSCms-UHC-HW-V
    pub fn use_krencodings(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseKREncodings(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseKREncodings failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable simplified Chinese encodings. After the method is invoked, an application can use the following simplified Chinese encodings.
    /// * GB-EUC-H
    /// * GB-EUC-V
    /// * GBK-EUC-H
    /// * GBK-EUC-V
    pub fn use_cnsencodings(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNSEncodings(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseCNSEncodings failed (status = {})", status);
        }

        Ok(())
    }

    /// Enable traditional Chinese encodings. After the method is invoked, an application can use the following traditional Chinese encodings.
    /// * GB-EUC-H
    /// * GB-EUC-V
    /// * GBK-EUC-H
    /// * GBK-EUC-V
    pub fn use_cntencodings(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNTEncodings(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseCNTEncodings failed (status = {})", status);
        }

        Ok(())
    }

    pub fn use_utfencodings(&self) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseUTFEncodings(self.handle())
        };

        if status != 0 {
            anyhow::bail!("HPDF_UseCNTEncodings failed (status = {})", status);
        }

        Ok(())
    }

    /// Save the current document to a file.
    pub fn save_to_file(&self, name: &str) -> anyhow::Result<()> {
        let name = CString::new(name).unwrap();
        let status = unsafe {
            libharu_sys::HPDF_SaveToFile(self.handle(), std::mem::transmute(name.as_bytes().as_ptr()))
        };

        if status != 0 {
            anyhow::bail!("HPDF_SaveToFile failed (status = {})", status);
        }

        Ok(())
    }

    /// Set the mode of compression.
    pub fn set_compression_mode(&self, mode: CompressionMode) -> anyhow::Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetCompressionMode(self.handle(), mode.bits())
        };

        if status != 0 {
            anyhow::bail!("HPDF_SetCompressionMode failed (status = {})", status);
        }

        Ok(())
    }

    /// creates root outline object.
    pub fn create_outline(&self, title: &str, parent: Option<&Outline>, enc: Option<&Encoder>) -> anyhow::Result<Outline> {
        let title = CString::new(title)?;
        
        let outline = unsafe {
            libharu_sys::HPDF_CreateOutline(
                self.handle(),
                match parent {
                    Some(p) => p.handle(),
                    None => std::ptr::null_mut(),
                },
                title.as_ptr() as *const i8,
                match enc {
                    Some(e) => e.handle(),
                    None => std::ptr::null_mut(),
                }
            )
        };

        if outline == std::ptr::null_mut() {
            anyhow::bail!("HPDF_CreateOutline failed");
        }

        Ok(Outline::new(self, outline))
    }

    /// creates root outline object. (raw bytes)
    pub fn create_outline_bytes(&self, title: &[u8], parent: Option<&Outline>, enc: Option<&Encoder>) -> anyhow::Result<Outline> {
        let title = CString::new(title)?;
        
        let outline = unsafe {
            libharu_sys::HPDF_CreateOutline(
                self.handle(),
                match parent {
                    Some(p) => p.handle(),
                    None => std::ptr::null_mut(),
                },
                title.as_ptr() as *const i8,
                match enc {
                    Some(e) => e.handle(),
                    None => std::ptr::null_mut(),
                }
            )
        };

        if outline == std::ptr::null_mut() {
            anyhow::bail!("HPDF_CreateOutline failed");
        }

        Ok(Outline::new(self, outline))
    }

    /// Get the handle of a corresponding encoder object by specified encoding name.
    pub fn find_encoder(&self, encoding_name: &str) -> anyhow::Result<Encoder> {
        let encoding_name = CString::new(encoding_name)?;
        let enc = unsafe {
            libharu_sys::HPDF_GetEncoder(self.handle(), encoding_name.as_ptr())
        };

        if enc == std::ptr::null_mut() {
            anyhow::bail!("HPDF_GetEncoder failed");
        }

        Ok(Encoder::new(self, enc))
    }

    /// Get the handle of the current encoder of the document object.
    pub fn current_encoder(&self) -> anyhow::Result<Encoder> {
        let enc = unsafe {
            libharu_sys::HPDF_GetCurrentEncoder(self.handle())
        };

        if enc == std::ptr::null_mut() {
            anyhow::bail!("HPDF_GetCurrentEncoder failed");
        }

        Ok(Encoder::new(self, enc))
    }

    /// Set the handle of the current encoder of the document object.
    pub fn set_current_encoder(&self, encoding_name: &str) -> anyhow::Result<()> {
        let encoding_name = CString::new(encoding_name)?;
        let status = unsafe {
            libharu_sys::HPDF_SetCurrentEncoder(self.handle(), encoding_name.as_ptr())
        };

        if status != 0 {
            anyhow::bail!("HPDF_SetCurrentEncoder failed (status={})", status);
        }

        Ok(())
    }
    
    /// Get the current setting for page layout.
    pub fn page_layout(&self) -> anyhow::Result<PageLayout> {
        let layout = unsafe {
            libharu_sys::HPDF_GetPageLayout(self.handle())
        };

        Ok(match layout {
            libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_SINGLE => PageLayout::Single,
            libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_ONE_COLUMN => PageLayout::OneColumn,
            libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => PageLayout::TwoColumnLeft,
            libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT  => PageLayout::TwoColumnRight,
            _ => anyhow::bail!("HPDF_GetPageLayout failed"),
        })
    }

    /// Set how the page should be displayed. If this attribute is not set, the setting of a viewer application is used.
    pub fn set_page_layout(&self, layout: PageLayout) -> anyhow::Result<()> {
        let layout = match layout {
            PageLayout::Single => libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_SINGLE,
            PageLayout::OneColumn => libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_ONE_COLUMN,
            PageLayout::TwoColumnLeft => libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
            PageLayout::TwoColumnRight  => libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
        };

        let status = unsafe {
            libharu_sys::HPDF_SetPageLayout(self.handle(), layout)
        };

        if status != 0 {
            anyhow::bail!("HPDF_SetPageLayout failed (status={})", status);
        }

        Ok(())
    }

    /// load a TrueType font from an external file and register it to a document object.
    pub fn load_ttf_font(&self, name: &str, embedding: bool) -> anyhow::Result<&str> {
        let name = CString::new(name)?;
        let ret = unsafe {
            libharu_sys::HPDF_LoadTTFontFromFile(self.handle(), name.as_ptr(), if embedding { 1 } else { 0 } )
        };

        if ret == std::ptr::null_mut() {
            anyhow::bail!("HPDF_LoadTTFontFromFile failed");
        }
        
        let s = unsafe { std::ffi::CStr::from_ptr(ret).to_str()? };

        //let ret = unsafe { CString::from_raw(ret as *mut i8).into_string()? };
        Ok(s)
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            libharu_sys::HPDF_Free(self.handle());
        }
    }
}

extern "C" fn onerror_callback(
    errno: libharu_sys::HPDF_STATUS,
    detailno: libharu_sys::HPDF_STATUS,
    userdata: libharu_sys::HPDF_HANDLE)
{
    let inner: &mut DocumentInner = unsafe { std::mem::transmute(userdata) };
    inner.last_errno = errno;
    inner.last_detailno = detailno;

    (inner.onerror)(Error::from_num(errno));
}
