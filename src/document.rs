use std::ops::{Deref};
use std::ffi::{CString};

use super::core;

//use super::core::page::{Page};
use super::core::page::private::PageHandle;
use super::page::{PageDescriptionMode};
//use super::destination::Destination;
use super::encoder::Encoder;
use super::font::Font;
use super::outline::Outline;
use super::core::error::{Error, Result};
use super::image::Image;

/// PDFドキュメントオブジェクト
/// 
/// ### Font API
/// * [`Document::find_font`]
/// * [`Document::load_type1_font`]
/// * [`Document::load_ttf_font`]
/// * [`Document::enable_jp_fonts`]
/// * [`Document::enable_kr_fonts`]
/// * [`Document::enable_cns_fonts`]
/// * [`Document::enable_cnt_fonts`]
/// 
/// ### Encoding API
/// * [`Document::find_encoder`]
/// * [`Document::current_encoder`]
/// * [`Document::set_current_encoder`]
/// * [`Document::enable_jp_encodings`]
/// * [`Document::enable_kr_encodings`]
/// * [`Document::enable_cns_encodings`]
/// * [`Document::enable_cnt_encodings`]
/// 
/// ### Pageview API
/// * [`Document::pageview_layout`]
/// * [`Document::set_pageview_layout`]
/// * [`Document::pageview_mode`]
/// * [`Document::set_pageview_mode`]
/// * [`Document::set_welcome_page`]
/// 
/// ### Image API
/// * [`Document::load_png_image`]
/// * [`Document::load_raw_image`]
/// * [`Document::load_raw_image_from_mem`]
/// * [`Document::load_jpeg_image`]
/// 
/// ### Attribute API
/// * [`Document::get_info_attribute`]
/// * [`Document::set_info_attribute`]
pub struct Document<T>
where
    T: Deref<Target=core::Document>
{
    doc: T
}

impl Document<Box<core::Document>> {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self { doc: Box::new(core::Document::new()?) })
    }
}

impl<'a> Document<&'a core::Document> {
    #[inline]
    pub fn new_by_ref(doc: &'a core::Document) -> Result<Self> {
        Ok(Self { doc })
    }
}

impl<T> Document<T>
where
    T: Deref<Target=core::Document>
{
    #[inline]
    pub fn save(&mut self, filename: &str) -> Result<()> {
        let filename = CString::new(filename)?;
        self.doc.deref().save(filename.as_c_str())
    }

    /* ******************************************************************** */
    /*  Encoder API                                                         */
    /* ******************************************************************** */

    /// 引数'encoding_name'で与えられたエンコーディング名を持つエンコーダオブジェクトを生成する。
    #[inline]
    pub fn find_encoder(&self, encoding_name: &str) -> Result<Encoder<'_, Box<core::Encoder>>> {
        let encoding_name = CString::new(encoding_name)?;
        
        Ok(Encoder::new(self.doc.deref(), self.doc.deref().find_encoder(encoding_name.as_c_str())?.handle()))
    }

    /// このドキュメントに設定されているエンコーダを返す。
    #[inline]
    pub fn current_encoder(&self) -> Result<Encoder<'_, Box<core::Encoder>>> {
        Ok(Encoder::new(self.doc.deref(), self.doc.deref().current_encoder()?.handle()))
    }

    /// このドキュメントに新しいエンコーダを設定する。
    #[inline]
    pub fn set_current_encoder(&mut self, encoding_name: &str) -> Result<()> {
        let encoding_name = CString::new(encoding_name)?;
        self.doc.deref().set_current_encoder(encoding_name.as_c_str())
    }

    /// 日本語エンコーディングを有効にする。
    #[inline]
    pub fn enable_jp_encodings(&mut self) -> Result<()> {
        self.doc.deref().enable_jp_encodings()
    }

    /// 韓国語エンコーディングを有効にする。
    #[inline]
    pub fn enable_kr_encodings(&mut self) -> Result<()> {
        self.doc.deref().enable_kr_encodings()
    }

    /// 簡体字中国語エンコーディングを有効にする。
    #[inline]
    pub fn enable_cns_encodings(&mut self) -> Result<()> {
        self.doc.deref().enable_cns_encodings()
    }

    /// 繁体字中国語エンコーディングを有効にする。
    #[inline]
    pub fn enable_cnt_encodings(&mut self) -> Result<()> {
        self.doc.deref().enable_cnt_encodings()
    }
    



    /* ******************************************************************** */
    /*  Font API                                                            */
    /* ******************************************************************** */


    /// 引数で与えられたフォント名とエンコーディング名を持つフォントオブジェクトを生成する。
    #[inline]
    pub fn find_font(&self, font_name: &str, encoding_name: &str) -> Result<Font<'_, Box<core::Font>>> {
        let font_name = CString::new(font_name)?;
        let encoding_name = CString::new(encoding_name)?;

        Ok(Font::new(
            self.doc.deref(),
            self.doc.deref().find_font(
                font_name.as_c_str(),
                encoding_name.as_c_str()
            )?.handle()
        ))
    }

    /// Type1フォントを読み込む。
    #[inline]
    pub fn load_type1_font(&mut self, afm_filename: &str, pfm_filename: &str, encoding_name: &str) -> Result<Font<'_, Box<core::Font>>> {
        let afm_filename = CString::new(afm_filename)?;
        let pfm_filename = CString::new(pfm_filename)?;
        let encoding_name = CString::new(encoding_name)?;

        Ok(Font::new(
            self.doc.deref(),
            self.doc.deref().load_type1_font(
                afm_filename.as_c_str(),
                pfm_filename.as_c_str(),
                encoding_name.as_c_str(),
            )?.handle()
        ))
    }

    /// TTFフォントを読み込む。
    #[inline]
    pub fn load_ttf_font(&mut self, filename: &str, embedding: bool, index: usize, encoding_name: &str) -> Result<Font<'_, Box<core::Font>>> {
        let filename = CString::new(filename)?;
        let encoding_name = CString::new(encoding_name)?;

        Ok(Font::new(
            self.doc.deref(),
            self.doc.deref().load_ttf_font(
                filename.as_c_str(),
                embedding,
                index,
                encoding_name.as_c_str()
            )?.handle()
        ))
    }

    /// 日本語フォントを有効にする。
    #[inline]
    pub fn enable_jp_fonts(&mut self) -> Result<()> {
        self.doc.deref().enable_jp_fonts()
    }

    /// 韓国語フォントを有効にする。
    #[inline]
    pub fn enable_kr_fonts(&mut self) -> Result<()> {
        self.doc.deref().enable_kr_fonts()
    }

    /// 簡体字中国語フォントを有効にする。
    #[inline]
    pub fn enable_cns_fonts(&mut self) -> Result<()> {
        self.doc.deref().enable_cns_fonts()
    }

    /// 繁体字中国語フォントを有効にする。
    #[inline]
    pub fn enable_cnt_fonts(&mut self) -> Result<()> {
        self.doc.deref().enable_cnt_fonts()
    }
    



    /* ******************************************************************** */
    /*  Outline API                                                         */
    /* ******************************************************************** */

    #[inline]
    pub fn add_outline<'a, OLINE>(&mut self, title: &str, parent: Option<&OLINE>) -> Result<Outline<'_, Box<core::Outline>>>
    where
        OLINE: Deref<Target=core::Outline<'a>>
    {
        let title = CString::new(title)?;

        Ok(Outline::new(
            self.doc.deref(),
            self.doc.deref().add_outline(
                title.as_c_str(),
                parent.map(|op| op.deref())
            )?.handle()
        ))
    }




    /* ******************************************************************** */
    /*  Page API                                                            */
    /* ******************************************************************** */


    /// ドキュメントの末尾に新しいページを追加する。
    #[inline]
    pub fn add_page(&mut self) -> Result<PageDescriptionMode<'_, Box<core::page::PageDescriptionMode>>> {
        Ok(PageDescriptionMode::new(
            self.doc.deref(),
            self.doc.deref().add_page()?.handle()
        ))
    }

/* TODO: 後で検討
    /// 引数で指定したページの前に新しいページを追加する。
    #[inline]
    pub fn insert_page<'a, PAGE>(&mut self, page: &PAGE) -> Result<PageDescriptionMode<'_, Box<core::page::PageDescriptionMode>>>
    where
        PAGE: Deref<Target=core::Page<'a>>
    {
        Ok(PageDescriptionMode::new(
            self.doc.deref(),
            self.doc.deref().insert_page(page.deref())?.handle()
        ))
    }

    /// ドキュメントの現在のページを取得する。(TODO: 現在のページとは？)
    #[inline]
    pub fn current_page(&self) -> Result<PageDescriptionMode<'_, Box<core::page::PageDescriptionMode>>> {
        Ok(PageDescriptionMode::new(
            self.doc.deref(),
            self.doc.deref().current_page()?.handle()
        ))
    }
*/



    /* ******************************************************************** */
    /*  Pageview API                                                        */
    /* ******************************************************************** */
    #[inline]
    pub fn pageview_layout(&self) -> Result<PageLayout> {
        self.doc.deref().pageview_layout()
    }
    #[inline]
    pub fn set_pageview_layout(&mut self, layout: PageLayout) -> Result<()> {
        self.doc.deref().set_pageview_layout(layout)
    }
    #[inline]
    pub fn pageview_mode(&self) -> Result<PageMode> {
        self.doc.deref().pageview_mode()
    }
    #[inline]
    pub fn set_pageview_mode(&mut self, mode: PageMode) -> Result<()> {
        self.doc.deref().set_pageview_mode(mode)
    }

    #[inline]
    pub fn set_welcome_page<'a, DST>(&mut self, dst: &DST) -> Result<()>
    where
        DST: Deref<Target=core::Destination<'a>>
    {
        self.doc.deref().set_welcome_page(dst.deref())
    }




    /* ******************************************************************** */
    /*  Image API                                                           */
    /* ******************************************************************** */
    

    #[inline]
    pub fn load_png_image(&mut self, filename: &str) -> Result<Image<'_, Box<core::Image>>> {
        let filename = CString::new(filename)?;
        Ok(Image::new(
            self.doc.deref(),
            self.doc.deref().load_png_image(filename.as_c_str())?.handle()
        ))
    }

    #[inline]
    pub fn load_raw_image(&mut self, filename: &str, width: usize, height: usize, color_space: ColorSpace) -> Result<Image<'_, Box<core::Image>>> {
        let filename = CString::new(filename)?;
        Ok(Image::new(
            self.doc.deref(),
            self.doc.deref().load_raw_image(
                filename.as_c_str(),
                width,
                height,
                color_space
            )?.handle()
        ))
    }

    #[inline]
    pub fn load_raw_image_from_mem(&mut self, buf: &[u8], width: usize, height: usize, color_space: ColorSpace, bits_per_component: usize) -> Result<Image<'_, Box<core::Image>>> {
        Ok(Image::new(
            self.doc.deref(),
            self.doc.deref().load_raw_image_from_mem(
                buf,
                width,
                height,
                color_space,
                bits_per_component
            )?.handle()
        ))
    }

    #[inline]
    pub fn load_jpeg_image(&mut self, filename: &str) -> Result<Image<'_, Box<core::Image>>> {
        let filename = CString::new(filename)?;
        Ok(Image::new(
            self.doc.deref(),
            self.doc.deref().load_jpeg_image(filename.as_c_str())?.handle()
        ))
    }




    /* ******************************************************************** */
    /*  Attribute API                                                       */
    /* ******************************************************************** */
    
    #[inline]
    pub fn get_info_attribute(&self, info_type: InfoType) -> Result<&str> {
        self.doc.deref().get_info_attribute(info_type)
    }

    #[inline]
    pub fn set_info_attribute(&mut self, info_type: InfoType, value: &str) -> Result<()> {
        let value = CString::new(value)?;
        self.doc.deref().set_info_attribute(info_type, value.as_c_str())
    }

    #[inline]
    pub fn set_password(&mut self, owner_password: &str, user_password: &str) -> Result<()> {
        let owner_password = CString::new(owner_password)?;
        let user_password = CString::new(user_password)?;
        self.doc.deref().set_password(owner_password.as_c_str(), user_password.as_c_str())
    }

    #[inline]
    pub fn set_encryption_mode(&mut self, enc_mode: EncryptMode, key_len: usize) -> Result<()> {
        self.doc.deref().set_encryption_mode(enc_mode, key_len)
    }

    #[inline]
    pub fn set_compression_mode(&mut self, compression_mode: CompressionMode) -> Result<()> {
        self.doc.deref().set_compression_mode(compression_mode)
    }
}

pub type PageLayout = super::core::document::PageLayout;
pub type PageMode = super::core::document::PageMode;
pub type ColorSpace = super::core::document::ColorSpace;
pub type InfoType = super::core::document::InfoType;
pub type EncryptMode = super::core::document::EncryptMode;
pub type CompressionMode = super::core::document::CompressionMode;
