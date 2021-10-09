# libharu-rs

Rust binding for libharu(http://libharu.org/) PDF library.


## API implementaion status

### Document Handling Functions

| C API                       | Rust API |
|-----------------------------|-|
| HPDF_New                    | Document::new |
| HPDF_NewEx                  | |
| HPDF_Free                   | Document::drop (impl Drop trait) |
| HPDF_NewDoc                 | |
| HPDF_FreeDoc                | |
| HPDF_FreeDocAll             | |
| HPDF_SaveToFile             | Document::save_to_file |
| HPDF_SaveToStream           | |
| HPDF_GetStreamSize          | |
| HPDF_ReadFromStream         | |
| HPDF_ResetStream            | |
| HPDF_HasDoc                 | |
| HPDF_SetErrorHandler        | |
| HPDF_GetError               | |
| HPDF_ResetError             | |
| HPDF_SetPagesConfiguration  | |
| HPDF_SetPageLayout          | Document::set_page_layout |
| HPDF_GetPageLayout          | Document::page_layout |
| HPDF_SetPageMode            | Document::set_page_mode |
| HPDF_GetPageMode            | Document::page_mode |
| HPDF_SetOpenAction          | Document::set_open_action |
| HPDF_GetCurrentPage         | Document::current_page |
| HPDF_AddPage                | Document::add_page |
| HPDF_InsertPage             | Document::insert_page |
| HPDF_GetFont                | Document::font |
| HPDF_LoadType1FontFromFile  | |
| HPDF_LoadTTFontFromFile     | Document::load_ttf_font |
| HPDF_LoadTTFontFromFile2    | |
| HPDF_AddPageLabel           | Document::add_page_label |
| HPDF_UseJPFonts             | Document::use_jpfonts |
| HPDF_UseKRFonts             | Document::use_krfonts |
| HPDF_UseCNSFonts            | Document::use_cnsfonts |
| HPDF_UseCNTFonts            | Document::use_cntfonts |
| HPDF_CreateOutline          | Document::craete_outline / Document::create_outline_bytes |
| HPDF_GetEncoder             | Document::find_encoder |
| HPDF_GetCurrentEncoder      | Document::current_encoder |
| HPDF_SetCurrentEncoder      | Document::set_current_endoer |
| HPDF_UseJPEncodings         | Document::use_jpencodings |
| HPDF_UseKREncodings         | Document::use_krencodings |
| HPDF_UseCNSEncodings        | Document::use_cnsencodings |
| HPDF_UseCNTEncodings        | Document::use_cntencodings |
| HPDF_LoadPngImageFromFile   | Document::load_png_image |
| HPDF_LoadPngImageFromFile2  | |
| HPDF_LoadRawImageFromFile   | |
| HPDF_LoadRawImageFromFile2  | |
| HPDF_LoadRawImageFromMem    | |
| HPDF_LoadJpegImageFromFile  | |
| HPDF_SetInfoAttr            | |
| HPDF_GetInfoAttr            | |
| HPDF_SetInfoDateAttr        | |
| HPDF_SetPassword            | |
| HPDF_SetPermission          | |
| HPDF_SetEncryptionMode      | |
| HPDF_SetCompressionMode     | Document::set_compression_mode |

### Page Handling Functions

| C API                           | Rust API |
|---------------------------------|-|
| HPDF_Page_SetWidth              | Page::set_width |
| HPDF_Page_SetHeight             | Page::set_height |
| HPDF_Page_SetSize               | Page::set_size |
| HPDF_Page_SetRotate             | Page::set_rotate |
| HPDF_Page_GetWidth              | Page::width |
| HPDF_Page_GetHeight             | Page::height |
| HPDF_Page_CreateDestination     | Page::create_destination |
| HPDF_Page_CreateTextAnnot       | |
| HPDF_Page_CreateLinkAnnot       | |
| HPDF_Page_CreateURILinkAnnot    | |
| HPDF_Page_TextWidth             | Page::text_width |
| HPDF_Page_MeasureText           | Page::measure_text / Page::measure_text_bytes |
| HPDF_Page_GetGMode              | |
| HPDF_Page_GetCurrentPos         | |
| HPDF_Page_GetCurrentTextPos     | Page::current_text_pos |
| HPDF_Page_GetCurrentFont        | Page::current_font |
| HPDF_Page_GetCurrentFontSize    | Page::current_font_size |
| HPDF_Page_GetTransMatrix        | |
| HPDF_Page_GetLineWidth          | Page::line_width |
| HPDF_Page_GetLineCap            | |
| HPDF_Page_GetLineJoin           | |
| HPDF_Page_GetMiterLimit         | |
| HPDF_Page_GetDash               | |
| HPDF_Page_GetFlat               | |
| HPDF_Page_GetCharSpace          | |
| HPDF_Page_GetWordSpace          | |
| HPDF_Page_GetHorizontalScaling  | |
| HPDF_Page_GetTextLeading        | Page::text_leading |
| HPDF_Page_GetTextRenderingMode  | |
| HPDF_Page_GetTextRise           | |
| HPDF_Page_GetRGBFill            | Page::rgb_fill |
| HPDF_Page_GetRGBStroke          | |
| HPDF_Page_GetCMYKFill           | |
| HPDF_Page_GetCMYKStroke         | |
| HPDF_Page_GetGrayFill           | |
| HPDF_Page_GetGrayStroke         | |
| HPDF_Page_GetStrokingColorSpace | |
| HPDF_Page_GetFillingColorSpace  | |
| HPDF_Page_GetTextMatrix         | |
| HPDF_Page_GetGStateDepth        | |
| HPDF_Page_SetSlideShow          | |

### Graphics

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Page_SetLineWidth             | Page::set_line_width |
| HPDF_Page_SetLineCap               | Page::set_line_cap |
| HPDF_Page_SetLineJoin              | Page::set_line_join |
| HPDF_Page_SetMiterLimit            | |
| HPDF_Page_SetDash                  | Page::set_dash / Page::clear_dash |
| HPDF_Page_SetExtGState             | |
| HPDF_Page_GSave                    | Page::gsave |
| HPDF_Page_GRestore                 | Page::grestore |
| HPDF_Page_Concat                   | |
| HPDF_Page_MoveTo                   | Page::move_to |
| HPDF_Page_LineTo                   | Page::line_to |
| HPDF_Page_CurveTo                  | Page::curve_to |
| HPDF_Page_CurveTo2                 | Page::curve_to_2 |
| HPDF_Page_CurveTo3                 | Page::curve_to_3 |
| HPDF_Page_ClosePath                | |
| HPDF_Page_Rectangle                | Page::rectangle |
| HPDF_Page_Stroke                   | Page::stroke |
| HPDF_Page_ClosePathStroke          | |
| HPDF_Page_Fill                     | Page::fill |
| HPDF_Page_Eofill                   | |
| HPDF_Page_FillStroke               | Page::fill_stroke |
| HPDF_Page_EofillStroke             | |
| HPDF_Page_ClosePathFillStroke      | |
| HPDF_Page_ClosePathEofillStroke    | |
| HPDF_Page_EndPath                  | |
| HPDF_Page_Clip                     | Page::clip |
| HPDF_Page_Eoclip                   | |
| HPDF_Page_BeginText                | Page::begin_text |
| HPDF_Page_EndText                  | Page::end_text |
| HPDF_Page_SetCharSpace             | Page::set_char_space |
| HPDF_Page_SetWordSpace             | Page::set_word_space |
| HPDF_Page_SetHorizontalScalling    | |
| HPDF_Page_SetTextLeading           | Page::set_text_leading |
| HPDF_Page_SetFontAndSize           | Page::set_font_and_size |
| HPDF_Page_SetTextRenderingMode     | Page::set_text_rendering_mode |
| HPDF_Page_SetTextRise              | |
| HPDF_Page_MoveTextPos              | Page::move_text_pos |
| HPDF_Page_MoveTextPos2             | |
| HPDF_Page_SetTextMatrix            | Page::set_text_matrix |
| HPDF_Page_MoveToNextLine           | |
| HPDF_Page_ShowText                 | Page::show_text / Page::show_text_bytes |
| HPDF_Page_ShowTextNextLine         | Page::show_text_next_line / Page::show_text_next_line_bytes |
| HPDF_Page_ShowTextNextLineEx       | |
| HPDF_Page_SetGrayFill              | Page::set_gray_fill |
| HPDF_Page_SetGrayStroke            | Page::set_gray_stroke |
| HPDF_Page_SetRGBFill               | Page::set_rgb_fill |
| HPDF_Page_SetRGBStroke             | Page::set_rgb_stroke |
| HPDF_Page_SetCMYKFill              | |
| HPDF_Page_SetCMYKStroke            | |
| HPDF_Page_ExecuteXObject           | |
| HPDF_Page_DrawImage                | Page::draw_image |
| HPDF_Page_Circle                   | |
| HPDF_Page_Arc                      | |
| HPDF_Page_TextOut                  | Page::text_out / Page::text_out_bytes |
| HPDF_Page_TextRect                 | |

### Font Handling

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Font_GetFontName              | |
| HPDF_Font_GetBBox                  | |
| HPDF_Font_GetAscent                | |
| HPDF_Font_GetDescent               | |
| HPDF_Font_GetXHeight               | |
| HPDF_Font_GetCapHeight             | |
| HPDF_Font_TextWidth                | |
| HPDF_Font_MeasureText              | |

### Encoder

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Encoder_GetType               | Encoder::encoder_type |
| HPDF_Encoder_GetByteType           | Encoder::byte_type |
| HPDF_Encoder_GetUnicode            | |
| HPDF_Encoder_GetWritingMode        | |

### Annotation

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_LinkAnnot_SetHighlightMode    | |
| HPDF_LinkAnnot_SetBorderStyle      | |
| HPDF_LinkAnnot_SetIcon             | |
| HPDF_LinkAnnot_SetOpened           | |

### Outline

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Outline_SetOpened             | Outline::set_opened |
| HPDF_Outline_SetDestination        | Outline::set_destination |

### Destination

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Destination_SetXYZ            | Destination::set_syz |
| HPDF_Destination_SetFit            | |
| HPDF_Destination_SetFitH           | |
| HPDF_Destination_SetFitV           | |
| HPDF_Destination_SetFitR           | |
| HPDF_Destination_SetFitB           | |
| HPDF_Destination_SetFitBH          | |
| HPDF_Destination_SetFitBV          | |

### Image

| C API                              | Rust API |
|------------------------------------|-|
| HPDF_Image_GetSize                 | |
| HPDF_Image_GetWidth                | Image::width |
| HPDF_Image_GetHeight               | Image::height |
| HPDF_Image_GetBitsPerComponent     | |
| HPDF_Image_GetColorSpace           | |
| HPDF_Image_SetColorMask            | |
| HPDF_Image_SetMaskImage            | |
