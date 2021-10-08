extern crate libharu;

use libharu::prelude::*;

fn main() -> anyhow::Result<()> {
    // Same as jpfont_demo, but different from ttf font, and UTF-8 encoding
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })?;

    /* configure pdf-document to be compressed */
    doc.set_compression_mode(CompressionMode::ALL)?;

    /* declaration for using Japanese font, encoding */
    doc.use_jpencodings()?;
    doc.use_jpfonts()?;
    doc.use_utfencodings()?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("argument error");
        println!("Usage:");
        println!("    utf_demo <ttf_font_path>");
        anyhow::bail!("argument error");
    }
    println!("{}", std::env::current_dir().unwrap().as_path().to_str().unwrap());
    println!("{}", args[1]);
    let ttfont = doc.load_ttf_font(&args[1], true)?;

    let detail_font = [
        doc.font(ttfont,        Some("UTF-8"))?,
    ];

    /* set UTF-8 */
    doc.set_current_encoder("UTF-8")?;

    /* Set page mode to use outlines. */
    doc.set_page_mode(PageMode::Outline)?;

    /* create outline root. */
    let root = doc.create_outline("JP font demo", None, None)?;
    root.set_opened(true)?;

    let jptext = "アメンボ赤いなあいうえお。浮き藻に小エビもおよいでる。";
    let mut pos = (0.0, 0.0).into();

    for font in &detail_font {
        /* add a new page object */
        let page = doc.add_page()?;
        let page = PageDescriptionMode::new(&page);

        /* create outline entry */
        let outline = doc.create_outline(font.name()?, Some(&root), None)?;
        let dst = page.create_destination()?;
        outline.set_destination(&dst)?;

        let title_font = doc.font("Helvetica", None)?;
        page.set_font_and_size(&title_font, 18.0)?;

        page.run_text_mode(|page|{
            /* move the position of the text to top of the page */
            page.move_text_pos((10.0, 190.0))?;
            page.show_text(font.name()?)?;
    
            page.set_font_and_size(font, 15.0)?;
            page.move_text_pos((10.0, -20.0))?;
            page.show_text("abcdefghijklmnopqrstuvwxyz")?;

            page.move_text_pos((0.0, -20.0))?;
            page.show_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ")?;
            page.move_text_pos((0.0, -20.0))?;
            page.show_text("1234567890")?;
            page.move_text_pos((0.0, -20.0))?;
    
    
            page.set_font_and_size(font, 10.0)?;
            page.show_text(jptext)?;
            page.move_text_pos((0.0, -18.0))?;
    
            page.set_font_and_size(font, 16.0)?;
            page.show_text(jptext)?;
            page.move_text_pos((0.0, -27.0))?;
    
            page.set_font_and_size(font, 23.0)?;
            page.show_text(jptext)?;
            page.move_text_pos((0.0, -36.0))?;
    
            page.set_font_and_size(font, 30.0)?;
            page.show_text(jptext)?;
    
            pos = page.current_text_pos()?;
            Ok(())
        })?;

        let mut x_pos = 20.0;
        for _ in 0..jptext.len()/2 {
            page.run_path_mode(|page|{
                page.move_to((x_pos, pos.y - 10.0))?;
                page.line_to((x_pos, pos.y - 12.0))?;
                page.stroke()?;
                Ok(())
            })?;
            x_pos = x_pos + 30.0;
        }

        const PAGE_HEIGHT:libharu::Real = 210.0;
        page.run_path_mode(|page|{
            page.set_width(pos.x + 20.0)?;
            page.set_height(PAGE_HEIGHT)?;
    
            page.move_to((10.0, PAGE_HEIGHT - 25.0))?;
            page.line_to((pos.x + 10.0, PAGE_HEIGHT - 25.0))?;
            page.stroke()?;
    
            page.move_to((10.0, PAGE_HEIGHT - 85.0))?;
            page.line_to((pos.x + 10.0, PAGE_HEIGHT - 85.0))?;
            page.stroke()?;
    
            page.move_to((10.0, pos.y - 12.0))?;
            page.line_to((pos.x + 10.0, pos.y - 12.0))?;
            page.stroke()?;
            Ok(())
        })?;
    }

    doc.save_to_file("utf_demo.pdf")?;

    Ok(())
}
