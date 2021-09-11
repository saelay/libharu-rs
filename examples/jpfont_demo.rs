extern crate libharu;
extern crate encoding_rs;

use libharu::{Document, PageMode, CompressionMode};

fn main() -> anyhow::Result<()> {
    // http://libharu.sourceforge.net/demo/jpfont_demo.c
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })?;

    /* configure pdf-document to be compressed */
    doc.set_compression_mode(CompressionMode::ALL)?;

    /* declaration for using Japanese font, encoding */
    doc.use_jpencodings()?;
    doc.use_jpfonts()?;
    
    let detail_font = [
        doc.font("MS-Mincho",             Some("90ms-RKSJ-H"))?,
        doc.font("MS-Mincho,Bold",        Some("90ms-RKSJ-H"))?,
        doc.font("MS-Mincho,Italic",      Some("90ms-RKSJ-H"))?,
        doc.font("MS-Mincho,BoldItalic",  Some("90ms-RKSJ-H"))?,
        doc.font("MS-PMincho",            Some("90msp-RKSJ-H"))?,
        doc.font("MS-PMincho,Bold",       Some("90msp-RKSJ-H"))?,
        doc.font("MS-PMincho,Italic",     Some("90msp-RKSJ-H"))?,
        doc.font("MS-PMincho,BoldItalic", Some("90msp-RKSJ-H"))?,
        doc.font("MS-Gothic",             Some("90ms-RKSJ-H"))?,
        doc.font("MS-Gothic,Bold",        Some("90ms-RKSJ-H"))?,
        doc.font("MS-Gothic,Italic",      Some("90ms-RKSJ-H"))?,
        doc.font("MS-Gothic,BoldItalic",  Some("90ms-RKSJ-H"))?,
        doc.font("MS-PGothic",            Some("90msp-RKSJ-H"))?,
        doc.font("MS-PGothic,Bold",       Some("90msp-RKSJ-H"))?,
        doc.font("MS-PGothic,Italic",     Some("90msp-RKSJ-H"))?,
        doc.font("MS-PGothic,BoldItalic", Some("90msp-RKSJ-H"))?,
    ];

    /* Set page mode to use outlines. */
    doc.set_page_mode(PageMode::Outline)?;

    /* create outline root. */
    let root = doc.create_outline("JP font demo", None)?;
    root.set_opened(true)?;

    for font in &detail_font {
        /* add a new page object */
        let page = doc.add_page()?;

        /* create outline entry */
        let outline = doc.create_outline(font.name()?, Some(&root))?;
        let dst = page.create_destination()?;
        outline.set_destination(&dst)?;

        let title_font = doc.font("Helvetica", None)?;
        page.set_font_and_size(&title_font, 18.0);

        page.begin_text();

        /* move the position of the text to top of the page */
        page.move_text_pos((10.0, 190.0))?;
        page.show_text(font.name()?);

        page.set_font_and_size(font, 15.0);
        page.move_text_pos((10.0, -20.0))?;
        page.show_text("abcdefghijklmnopqrstuvwxyz");
        page.move_text_pos((0.0, -20.0))?;
        page.show_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        page.move_text_pos((0.0, -20.0))?;
        page.show_text("1234567890");
        page.move_text_pos((0.0, -20.0))?;

        let jptext = "アメンボ赤いなあいうえお。浮き藻に小エビもおよいでる。";
        let (jptext, _, _) = encoding_rs::SHIFT_JIS.encode(jptext);

        page.set_font_and_size(font, 10.0);
        page.show_text_raw(&jptext);
        page.move_text_pos((0.0, -18.0))?;

        page.set_font_and_size(font, 16.0);
        page.show_text_raw(&jptext);
        page.move_text_pos((0.0, -27.0))?;

        page.set_font_and_size(font, 23.0);
        page.show_text_raw(&jptext);
        page.move_text_pos((0.0, -36.0))?;

        page.set_font_and_size(font, 30.0);
        page.show_text_raw(&jptext);

        let pos = page.current_text_pos()?;

        /* finish to print text */
        page.end_text();

        let mut x_pos = 20.0;
        for _ in 0..jptext.len()/2 {
            page.move_to((x_pos, pos.y - 10.0))?;
            page.line_to((x_pos, pos.y - 12.0))?;
            page.stroke();
            x_pos = x_pos + 30.0;
        }

        const PAGE_HEIGHT:libharu::Real = 210.0;
        page.set_width(pos.x + 20.0);
        page.set_height(PAGE_HEIGHT);

        page.move_to((10.0, PAGE_HEIGHT - 25.0))?;
        page.line_to((pos.x + 10.0, PAGE_HEIGHT - 25.0))?;
        page.stroke();

        page.move_to((10.0, PAGE_HEIGHT - 85.0))?;
        page.line_to((pos.x + 10.0, PAGE_HEIGHT - 85.0))?;
        page.stroke();

        page.move_to((10.0, pos.y - 12.0))?;
        page.line_to((pos.x + 10.0, pos.y - 12.0))?;
        page.stroke();
    }

    doc.save_to_file("jpfont_demo.pdf");

    Ok(())
}
