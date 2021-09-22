use super::{
    Error, Result,
    Encoder,
    Font,
    Page,
    Outline,
    PageDescriptionMode,
    Destination,
    Image,
};

use std::ptr;
use std::ffi::CStr;
use std::mem;
use std::convert::TryInto;

extern "C" {
    fn HPDF_GetError(doc: libharu_sys::HPDF_Doc) -> libharu_sys::HPDF_STATUS;
    fn HPDF_GetInfoAttr(doc: libharu_sys::HPDF_Doc, info_type: libharu_sys::HPDF_InfoType) -> *const std::ffi::c_void;
    fn HPDF_SetInfoAttr(doc: libharu_sys::HPDF_Doc, info_type: libharu_sys::HPDF_InfoType, value: *const std::ffi::c_void) -> libharu_sys::HPDF_STATUS;
}

extern "C" fn onerror_callback(
    _errno: libharu_sys::HPDF_STATUS,
    _detailno: libharu_sys::HPDF_STATUS,
    _userdata: libharu_sys::HPDF_HANDLE)
{
}

pub struct Document {
    handle: libharu_sys::HPDF_Doc
}

impl Document {
    #[inline]
    pub fn new() -> Result<Self> {
        let handle = unsafe {
            libharu_sys::HPDF_New(onerror_callback, ptr::null_mut())
        };

        if handle == ptr::null_mut() {
            Err(Error::CantCreatePdfDoc)
        }
        else {
            Ok(Self { handle })
        }
    }

    #[inline]
    pub fn handle(&self) -> libharu_sys::HPDF_Doc {
        self.handle
    }

    #[inline]
    pub fn last_error(&self) -> Error {
        let status = unsafe {
            HPDF_GetError(self.handle())
        };

        status.into()
    }

    pub fn save(&self, filename: &CStr) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SaveToFile(self.handle(), filename.as_ptr())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /* ******************************************************************** */
    /*  Encoder API                                                         */
    /* ******************************************************************** */

    /// 引数'encoding_name'で与えられたエンコーディング名を持つエンコーダオブジェクトを生成する。
    pub fn find_encoder(&self, encoding_name: &CStr) -> Result<Encoder> {
        let encoder_handle = unsafe {
            libharu_sys::HPDF_GetEncoder(self.handle(), encoding_name.as_ptr())
        };

        if encoder_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Encoder::new(self, encoder_handle))
        }
    }

    /// このドキュメントに設定されているエンコーダを返す。
    pub fn current_encoder(&self) -> Result<Encoder> {
        let encoder_handle = unsafe {
            libharu_sys::HPDF_GetCurrentEncoder(self.handle())
        };

        if encoder_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Encoder::new(self, encoder_handle))
        }
    }

    /// このドキュメントに新しいエンコーダを設定する。
    pub fn set_current_encoder(&self, encoding_name: &CStr) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetCurrentEncoder(self.handle(), mem::transmute(encoding_name.as_ptr()))
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 日本語エンコーディングを有効にする。
    pub fn enable_jp_encodings(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseJPEncodings(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 韓国語エンコーディングを有効にする。
    pub fn enable_kr_encodings(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseKREncodings(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 簡体字中国語エンコーディングを有効にする。
    pub fn enable_cns_encodings(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNSEncodings(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 繁体字中国語エンコーディングを有効にする。
    pub fn enable_cnt_encodings(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNTEncodings(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
    



    /* ******************************************************************** */
    /*  Font API                                                            */
    /* ******************************************************************** */


    /// 引数で与えられたフォント名とエンコーディング名を持つフォントオブジェクトを生成する。
    pub fn find_font(&self, font_name: &CStr, encoding_name: &CStr) -> Result<Font> {

        let font_handle = unsafe {
            libharu_sys::HPDF_GetFont(
                self.handle(),
                font_name.as_ptr(),
                encoding_name.as_ptr(),
            )
        };

        if font_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Font::new(self, font_handle))
        }
    }

    /// Type1フォントを読み込む。
    pub fn load_type1_font(&self, afm_filename: &CStr, pfm_filename: &CStr, encoding_name: &CStr) -> Result<Font> {
        let font_name = unsafe {
            let ret = libharu_sys::HPDF_LoadType1FontFromFile(
                self.handle(),
                afm_filename.as_ptr(),
                pfm_filename.as_ptr()
            );

            std::ffi::CStr::from_ptr(ret)
        };

        self.find_font(font_name, encoding_name)
    }

    /// TTFフォントを読み込む。
    pub fn load_ttf_font(&self, filename: &CStr, embedding: bool, index: usize, encoding_name: &CStr) -> Result<Font> {
        let index = index.try_into()?;

        let font_name = unsafe {
            let ret = libharu_sys::HPDF_LoadTTFontFromFile2(
                self.handle(),
                filename.as_ptr(),
                index,
                if embedding { 1 } else { 0 }
            );

            std::ffi::CStr::from_ptr(ret)
        };

        self.find_font(font_name, encoding_name)
    }

    /// 日本語フォントを有効にする。
    pub fn enable_jp_fonts(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseJPFonts(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 韓国語フォントを有効にする。
    pub fn enable_kr_fonts(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseKRFonts(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 簡体字中国語フォントを有効にする。
    pub fn enable_cns_fonts(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNSFonts(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    /// 繁体字中国語フォントを有効にする。
    pub fn enable_cnt_fonts(&self) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_UseCNTFonts(self.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }
    



    /* ******************************************************************** */
    /*  Outline API                                                         */
    /* ******************************************************************** */

    pub fn add_outline(&self, title: &CStr, parent: Option<&Outline>) -> Result<Outline>
    {
        let enc = self.current_encoder()?;

        let outline_handle = unsafe {
            libharu_sys::HPDF_CreateOutline(
                self.handle(),
                if let Some(outline) = parent { outline.handle() } else { ptr::null_mut() },
                title.as_ptr(),
                enc.handle()
            )
        };

        if outline_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Outline::new(self, outline_handle))
        }
    }




    /* ******************************************************************** */
    /*  Page API                                                            */
    /* ******************************************************************** */


    /// ドキュメントの末尾に新しいページを追加する。
    pub fn add_page(&self) -> Result<PageDescriptionMode> {
        let page_handle = unsafe {
            libharu_sys::HPDF_AddPage(self.handle())
        };

        if page_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(PageDescriptionMode::new(self, page_handle))
        }
    }

    /// 引数で指定したページの前に新しいページを追加する。
    pub fn insert_page(&self, page: &dyn Page) -> Result<PageDescriptionMode>
    {
        let page_handle = unsafe {
            libharu_sys::HPDF_InsertPage(self.handle(), page.handle())
        };

        if page_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(PageDescriptionMode::new(self, page_handle))
        }
    }

    /// ドキュメントの現在のページを取得する。(TODO: 現在のページとは？)
    /// TODO: 戻り値はDescriptionModeでいいか？
    pub fn current_page(&self) -> Result<PageDescriptionMode> {
        let page_handle = unsafe {
            libharu_sys::HPDF_GetCurrentPage(self.handle())
        };

        if page_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(PageDescriptionMode::new(self, page_handle))
        }
    }




    /* ******************************************************************** */
    /*  Pageview API                                                        */
    /* ******************************************************************** */

    pub fn pageview_layout(&self) -> Result<PageLayout> {
        let layout = unsafe {
            libharu_sys::HPDF_GetPageLayout(self.handle())
        };

        if let libharu_sys::HPDF_PageLayout::HPDF_PAGE_LAYOUT_EOF = layout {
            Err(self.last_error())
        }
        else {
            Ok(layout.into())
        }
    }

    pub fn set_pageview_layout(&self, layout: PageLayout) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetPageLayout(self.handle(), layout.into())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn pageview_mode(&self) -> Result<PageMode> {
        let mode = unsafe {
            libharu_sys::HPDF_GetPageMode(self.handle())
        };

        if let libharu_sys::HPDF_PageMode::HPDF_PAGE_MODE_EOF = mode {
            Err(self.last_error())
        }
        else {
            Ok(mode.into())
        }
    }

    pub fn set_pageview_mode(&self, mode: PageMode) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetPageMode(self.handle(), mode.into())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }


    pub fn set_welcome_page(&self, dst: &Destination) -> Result<()>
    {
        let status = unsafe {
            libharu_sys::HPDF_SetOpenAction(self.handle(), dst.handle())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }




    /* ******************************************************************** */
    /*  Image API                                                           */
    /* ******************************************************************** */
    
    pub fn load_png_image(&self, filename: &CStr) -> Result<Image> {
        let image_handle = unsafe {
            libharu_sys::HPDF_LoadPngImageFromFile2(self.handle(), filename.as_ptr())
        };

        if image_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Image::new(self, image_handle))
        }
    }

    pub fn load_raw_image(&self, filename: &CStr, width: usize, height: usize, color_space: ColorSpace) -> Result<Image> {
        let width = width.try_into()?;
        let height = height.try_into()?;
        let image_handle = unsafe {
            libharu_sys::HPDF_LoadRawImageFromFile(
                self.handle(),
                filename.as_ptr(),
                width,
                height,
                color_space.into(),
            )
        };

        if image_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Image::new(self, image_handle))
        }
    }

    pub fn load_raw_image_from_mem(&self, buf: &[u8], width: usize, height: usize, color_space: ColorSpace, bits_per_component: usize) -> Result<Image> {
        let width = width.try_into()?;
        let height = height.try_into()?;
        let bits_per_component = bits_per_component.try_into()?;

        let image_handle = unsafe {
            libharu_sys::HPDF_LoadRawImageFromMem(
                self.handle(),
                buf.as_ptr(),
                width,
                height,
                color_space.into(),
                bits_per_component,
            )
        };

        if image_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Image::new(self, image_handle))
        }
    }

    pub fn load_jpeg_image(&self, filename: &CStr) -> Result<Image> {
        let image_handle = unsafe {
            libharu_sys::HPDF_LoadJpegImageFromFile(
                self.handle(),
                filename.as_ptr(),
            )
        };

        if image_handle == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            Ok(Image::new(self, image_handle))
        }
    }




    /* ******************************************************************** */
    /*  Attribute API                                                       */
    /* ******************************************************************** */
    
    pub fn get_info_attribute(&self, info_type: InfoType) -> Result<&str> {
        let s = unsafe {
            HPDF_GetInfoAttr(self.handle(), info_type.into())
        };

        if s == ptr::null_mut() {
            Err(self.last_error())
        }
        else {
            let s = unsafe { CStr::from_ptr(mem::transmute(s)) };
            Ok(s.to_str()?)
        }
    }

    pub fn set_info_attribute(&self, info_type: InfoType, value: &CStr) -> Result<()> {
        let status = unsafe {
            HPDF_SetInfoAttr(
                self.handle(),
                info_type.into(),
                mem::transmute(value.as_ptr()),
            )
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_password(&self, owner_password: &CStr, user_password: &CStr) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetPassword(
                self.handle(),
                owner_password.as_ptr(),
                user_password.as_ptr(),
            )
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_encryption_mode(&self, enc_mode: EncryptMode, key_len: usize) -> Result<()> {
        let key_len = key_len.try_into()?;
        let status = unsafe {
            libharu_sys::HPDF_SetEncryptionMode(self.handle(), enc_mode.into(), key_len)
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }
    }

    pub fn set_compression_mode(&self, compression_mode: CompressionMode) -> Result<()> {
        let status = unsafe {
            libharu_sys::HPDF_SetCompressionMode(self.handle(), compression_mode.into())
        };

        if status != 0 {
            Err(status.into())
        }
        else {
            Ok(())
        }

    }

}

impl Drop for Document
{
    fn drop(&mut self) {
        unsafe {
            libharu_sys::HPDF_Free(self.handle());
        }
    }
}

pub enum PageLayout {
    Single,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

impl From<libharu_sys::HPDF_PageLayout> for PageLayout {
    fn from(v: libharu_sys::HPDF_PageLayout) -> Self {
        use libharu_sys::HPDF_PageLayout::*;
        match v {
            HPDF_PAGE_LAYOUT_SINGLE => Self::Single,
            HPDF_PAGE_LAYOUT_ONE_COLUMN => Self::OneColumn,
            HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => Self::TwoColumnLeft,
            HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT => Self::TwoColumnRight,
            HPDF_PAGE_LAYOUT_TWO_PAGE_LEFT => Self::TwoPageLeft,
            HPDF_PAGE_LAYOUT_TWO_PAGE_RIGHT => Self::TwoPageRight,
            HPDF_PAGE_LAYOUT_EOF => Self::Single,
        }
    }
}

impl From<PageLayout> for libharu_sys::HPDF_PageLayout {
    fn from(v: PageLayout) -> Self {
        use libharu_sys::HPDF_PageLayout::*;
        match v {
            PageLayout::Single => HPDF_PAGE_LAYOUT_SINGLE,
            PageLayout::OneColumn => HPDF_PAGE_LAYOUT_ONE_COLUMN,
            PageLayout::TwoColumnLeft => HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
            PageLayout::TwoColumnRight => HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
            PageLayout::TwoPageLeft => HPDF_PAGE_LAYOUT_TWO_PAGE_LEFT,
            PageLayout::TwoPageRight => HPDF_PAGE_LAYOUT_TWO_PAGE_RIGHT,
        }
    }
}

pub enum PageMode {
    None,
    Outline,
    Thumbs,
    FullScreen,
}

impl From<libharu_sys::HPDF_PageMode> for PageMode {
    fn from(v: libharu_sys::HPDF_PageMode) -> Self {
        use libharu_sys::HPDF_PageMode::*;
        match v {
            HPDF_PAGE_MODE_USE_NONE => Self::None,
            HPDF_PAGE_MODE_USE_OUTLINE => Self::Outline,
            HPDF_PAGE_MODE_USE_THUMBS => Self::Thumbs,
            HPDF_PAGE_MODE_FULL_SCREEN => Self::FullScreen,
            HPDF_PAGE_MODE_EOF => Self::None,
        }
    }
}
impl From<PageMode> for libharu_sys::HPDF_PageMode {
    fn from(v: PageMode) -> Self {
        use libharu_sys::HPDF_PageMode::*;
        match v {
             PageMode::None => HPDF_PAGE_MODE_USE_NONE,
             PageMode::Outline => HPDF_PAGE_MODE_USE_OUTLINE,
             PageMode::Thumbs => HPDF_PAGE_MODE_USE_THUMBS,
             PageMode::FullScreen => HPDF_PAGE_MODE_FULL_SCREEN,
        }
    }
}


pub enum ColorSpace {
    DeviceGray,
    DeviceRgb,
    DeviceCmyk,
    CalGray,
    CalRgb,
    Lab,
    IccBased,
    Separation,
    DeviceN,
    Indexed,
    Pattern,
}

impl From<libharu_sys::HPDF_ColorSpace> for ColorSpace {
    fn from(v: libharu_sys::HPDF_ColorSpace) -> Self {
        use libharu_sys::HPDF_ColorSpace::*;
        match v {
            HPDF_CS_DEVICE_GRAY => Self::DeviceGray,
            HPDF_CS_DEVICE_RGB => Self::DeviceRgb,
            HPDF_CS_DEVICE_CMYK => Self::DeviceCmyk,
            HPDF_CS_CAL_GRAY => Self::CalGray,
            HPDF_CS_CAL_RGB => Self::CalRgb,
            HPDF_CS_LAB => Self::Lab,
            HPDF_CS_ICC_BASED => Self::IccBased,
            HPDF_CS_SEPARATION => Self::Separation,
            HPDF_CS_DEVICE_N => Self::DeviceN,
            HPDF_CS_INDEXED => Self::Indexed,
            HPDF_CS_PATTERN => Self::Pattern,
            HPDF_CS_EOF => Self::DeviceGray,
        }
    }
}

impl From<ColorSpace> for libharu_sys::HPDF_ColorSpace {
    fn from(v: ColorSpace) -> Self {
        use libharu_sys::HPDF_ColorSpace::*;
        match v {
             ColorSpace::DeviceGray => HPDF_CS_DEVICE_GRAY,
             ColorSpace::DeviceRgb => HPDF_CS_DEVICE_RGB,
             ColorSpace::DeviceCmyk => HPDF_CS_DEVICE_CMYK,
             ColorSpace::CalGray => HPDF_CS_CAL_GRAY,
             ColorSpace::CalRgb => HPDF_CS_CAL_RGB,
             ColorSpace::Lab => HPDF_CS_LAB,
             ColorSpace::IccBased => HPDF_CS_ICC_BASED,
             ColorSpace::Separation => HPDF_CS_SEPARATION,
             ColorSpace::DeviceN => HPDF_CS_DEVICE_N,
             ColorSpace::Indexed => HPDF_CS_INDEXED,
             ColorSpace::Pattern => HPDF_CS_PATTERN,
        }
    }
}

pub enum InfoType {
    CreationDate,
    ModifiedDate,
    Author,
    Creator,
    Producer,
    Title,
    Subject,
    Keywords,
    Trapped,
    GtsPdfx,
}

impl From<libharu_sys::HPDF_InfoType> for InfoType {
    fn from(v: libharu_sys::HPDF_InfoType) -> Self {
        use libharu_sys::HPDF_InfoType::*;
        match v {
            HPDF_INFO_CREATION_DATE => Self::CreationDate,
            HPDF_INFO_MOD_DATE => Self::ModifiedDate,
            HPDF_INFO_AUTHOR => Self::Author,
            HPDF_INFO_CREATOR => Self::Creator,
            HPDF_INFO_PRODUCER => Self::Producer,
            HPDF_INFO_TITLE => Self::Title,
            HPDF_INFO_SUBJECT => Self::Subject,
            HPDF_INFO_KEYWORDS => Self::Keywords,
            HPDF_INFO_TRAPPED => Self::Trapped,
            HPDF_INFO_GTS_PDFX => Self::GtsPdfx,
            HPDF_INFO_EOF => Self::CreationDate,
        }
    }
}

impl From<InfoType> for libharu_sys::HPDF_InfoType {
    fn from(v: InfoType) -> Self {
        use libharu_sys::HPDF_InfoType::*;
        match v {
            InfoType::CreationDate => HPDF_INFO_CREATION_DATE,
            InfoType::ModifiedDate => HPDF_INFO_MOD_DATE,
            InfoType::Author => HPDF_INFO_AUTHOR,
            InfoType::Creator => HPDF_INFO_CREATOR,
            InfoType::Producer => HPDF_INFO_PRODUCER,
            InfoType::Title => HPDF_INFO_TITLE,
            InfoType::Subject => HPDF_INFO_SUBJECT,
            InfoType::Keywords => HPDF_INFO_KEYWORDS,
            InfoType::Trapped => HPDF_INFO_TRAPPED,
            InfoType::GtsPdfx => HPDF_INFO_GTS_PDFX,
        }
    }
}

pub enum EncryptMode {
    Rev2,
    Rev3,
}

impl From<libharu_sys::HPDF_EncryptMode> for EncryptMode {
    fn from(v: libharu_sys::HPDF_EncryptMode) -> Self {
        use libharu_sys::HPDF_EncryptMode::*;
        match v {
            HPDF_ENCRYPT_R2 => Self::Rev2,
            HPDF_ENCRYPT_R3 => Self::Rev3,
        }
    }
}

impl From<EncryptMode> for libharu_sys::HPDF_EncryptMode {
    fn from(v: EncryptMode) -> Self {
        use libharu_sys::HPDF_EncryptMode::*;
        match v {
            EncryptMode::Rev2 => HPDF_ENCRYPT_R2,
            EncryptMode::Rev3 => HPDF_ENCRYPT_R3,
        }
    }
}

pub enum CompressionMode {
    None,
    Text,
    Image,
    Metadata,
    All,
}

impl From<libharu_sys::HPDF_UINT> for CompressionMode {
    fn from(v: libharu_sys::HPDF_UINT) -> Self {
        match v {
            libharu_sys::HPDF_COMP_NONE => Self::None,
            libharu_sys::HPDF_COMP_TEXT => Self::Text,
            libharu_sys::HPDF_COMP_IMAGE => Self::Image,
            libharu_sys::HPDF_COMP_METADATA => Self::Metadata,
            libharu_sys::HPDF_COMP_ALL => Self::All,
            _ => Self::None,
        }
    }
}

impl From<CompressionMode> for libharu_sys::HPDF_UINT {
    fn from(v: CompressionMode) -> Self {
        match v {
            CompressionMode::None => libharu_sys::HPDF_COMP_NONE,
            CompressionMode::Text => libharu_sys::HPDF_COMP_TEXT,
            CompressionMode::Image => libharu_sys::HPDF_COMP_IMAGE,
            CompressionMode::Metadata => libharu_sys::HPDF_COMP_METADATA,
            CompressionMode::All => libharu_sys::HPDF_COMP_ALL,
        }
    }
}
