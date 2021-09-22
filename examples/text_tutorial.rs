extern crate libharu;
extern crate anyhow;

use libharu::{Point, Real, Document, Page, TextRenderingMode, CompressionMode};

mod util;

fn show_stripe_pattern<T:Into<Point>>(page: &Page, p: T) -> anyhow::Result<()> {
    let p = p.into();

    let mut iy = 0.0;
    while iy < 50.0 {
        page.set_rgb_stroke((0.0, 0.0, 0.5))?;
        page.set_line_width(1.0)?;
        page.move_to((p.x, p.y + iy))?;
        page.line_to((p.x + page.text_width("ABCabc123")?, p.y + iy))?;
        page.stroke()?;
        iy = iy + 3.0;
    }

    Ok(())
}

fn show_description(page: &Page, x: Real, y: Real, text: &str) -> anyhow::Result<()> {
    let fsize = page.current_font_size()?;
    let font = page.current_font()?;
    let color = page.rgb_fill()?;

    page.begin_text()?;
    page.set_rgb_fill((0.0, 0.0, 0.0))?;
    page.set_text_rendering_mode(TextRenderingMode::Fill)?;
    page.set_font_and_size(&font, 10.0)?;
    page.text_out(x, y - 12.0, text)?;
    page.end_text()?;

    page.set_font_and_size(&font, fsize)?;
    page.set_rgb_fill(color)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let samp_text = "abcdefgABCDEFG123!#$%&+-@?";
    let samp_text2 = "The quick brown fox jumps over the lazy dog.";

    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })?;

    /* set compression mode */
    doc.set_compression_mode(CompressionMode::ALL)?;

    /* create default-font */
    let font = doc.font("Helvetica", None)?;

    /* add a new page object */
    let page = doc.add_page()?;


    page.set_font_and_size(&font, 12.0)?;

    page.move_to((40.0, page.height()? - 30.0))?;
    page.line_to((240.0, page.height()? - 30.0))?;
    page.stroke()?;

    // set text pos (0, 0)
    page.begin_text()?;

    // move text pos
    page.move_text_pos((50.0, page.height()? - 30.0))?;

    let pos = page.current_text_pos()?;
    println!("1: {} {}", pos.x, pos.y);
    // write text from curren pos (0, 0)
    page.show_text("Quick brown lazy dogs")?;

    let pos = page.current_text_pos()?;
    println!("2: {} {}", pos.x, pos.y);
    page.set_text_leading(12.0)?;
    page.show_text_next_line("lazy brown quick cats.")?;
    let pos = page.current_text_pos()?;
    println!("3: {} {}", pos.x, pos.y);
    // end text
    page.end_text()?;

    /* save the document to a file */
    doc.save_to_file("text_tutorial.pdf")?;

    Ok(())
}
