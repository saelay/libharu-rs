extern crate libharu;
extern crate anyhow;
use std::path::PathBuf;
use libharu::prelude::*;

fn draw_image(doc: &Document, filename: &str, x: Real, y: Real, text: &str) -> anyhow::Result<()> {

    let page = doc.current_page()?;
    let page = PageDescriptionMode::new(&page);

    let mut path = PathBuf::from("./examples/pngsuite");
    path.push(filename);

    println!("{}", path.as_path().to_str().unwrap());
    let img = doc.load_png_image(path.as_path().to_str().unwrap())?;

    /* Draw image to the canvas. */
    page.draw_image(&img, (x, y), img.width()?, img.height()?)?;

    /* Print the text. */
    page.run_text_mode(|page|{
        page.set_text_leading(16.0)?;
        page.move_text_pos((x, y))?;
        page.show_text_next_line(filename)?;
        page.show_text_next_line(text)?;
        Ok(())
    })?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // http://libharu.sourceforge.net/demo/png_demo.c
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })?;

    doc.set_compression_mode(CompressionMode::ALL)?;

    /* create default-font */
    let font = doc.font("Helvetica", None)?;

    /* add a new page object */
    let page = doc.add_page()?;
    let page = PageDescriptionMode::new(&page);

    page.set_width(550.0)?;
    page.set_height(650.0)?;

    let dst = page.create_destination()?;
    dst.set_xyz(0.0, page.height()?, 1.0)?;
    doc.set_open_action(&dst)?;

    page.run_text_mode(|page|{
        page.set_font_and_size(&font, 20.0)?;
        page.move_text_pos((220.0, page.height()? - 70.0))?;
        page.show_text("PngDemo")?;
        Ok(())
    })?;

    page.set_font_and_size(&font, 12.0)?;

    draw_image(&doc, "basn0g01.png", 100.0, page.height()? - 150.0, "1bit grayscale.")?;
    draw_image(&doc, "basn0g02.png", 200.0, page.height()? - 150.0, "2bit grayscale.")?;
    draw_image(&doc, "basn0g04.png", 300.0, page.height()? - 150.0, "4bit grayscale.")?;
    draw_image(&doc, "basn0g08.png", 400.0, page.height()? - 150.0, "8bit grayscale.")?;
    
    draw_image(&doc, "basn2c08.png", 100.0, page.height()? - 250.0, "8bit color.")?;
    draw_image(&doc, "basn2c16.png", 200.0, page.height()? - 250.0, "16bit color.")?;

    draw_image(&doc, "basn3p01.png", 100.0, page.height()? - 350.0, "1bit pallet.")?;
    draw_image(&doc, "basn3p02.png", 200.0, page.height()? - 350.0, "2bit pallet.")?;
    draw_image(&doc, "basn3p04.png", 300.0, page.height()? - 350.0, "4bit pallet.")?;
    draw_image(&doc, "basn3p08.png", 400.0, page.height()? - 350.0, "8bit pallet.")?;

    draw_image(&doc, "basn4a08.png", 100.0, page.height()? - 450.0, "8bit alpha.")?;
    draw_image(&doc, "basn4a16.png", 200.0, page.height()? - 450.0, "16bit alpha.")?;

    draw_image(&doc, "basn6a08.png", 100.0, page.height()? - 550.0, "8bit alpha.")?;
    draw_image(&doc, "basn6a16.png", 200.0, page.height()? - 550.0, "8bit alpha.")?;

    /* save the document to a file */
    doc.save_to_file("line_demo.pdf")?;

    Ok(())
}
