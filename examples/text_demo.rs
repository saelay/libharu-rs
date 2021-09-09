use libharu::{Real, Document, Page, TextRenderingMode, CompressionMode};

mod util;

fn show_stripe_pattern(page: &Page, x: Real, y: Real) {
    let mut iy = 0.0;
    while iy < 50.0 {
        page.set_rgb_stroke(0.0, 0.0, 0.5);
        page.set_line_width(1.0);
        page.move_to(x, y + iy);
        page.line_to(x + page.text_width("ABCabc123"), y + iy);
        page.stroke();
        iy = iy + 3.0;
    }
}

fn show_description(page: &Page, x: Real, y: Real, text: &str) {
    let fsize = page.current_font_size();
    let font = page.current_font();
    let color = page.rgb_fill();

    page.begin_text();
    page.set_rgb_fill(0.0, 0.0, 0.0);
    page.set_text_rendering_mode(TextRenderingMode::Fill);
    page.set_font_and_size(&font, 10.0);
    page.text_out(x, y - 12.0, text);
    page.end_text();

    page.set_font_and_size(&font, fsize);
    page.set_rgb_fill(color.red, color.green, color.blue);
}

fn main() {
    // http://libharu.sourceforge.net/demo/text_demo.c
    let samp_text = "abcdefgABCDEFG123!#$%&+-@?";
    let samp_text2 = "The quick brown fox jumps over the lazy dog.";

    let doc = Document::new(|err| {
        println!("err={:?}", err);
    }).unwrap();

    /* set compression mode */
    doc.set_compression_mode(CompressionMode::ALL);

    /* create default-font */
    let font = doc.font("Helvetica", None).unwrap();

    /* add a new page object */
    let page = doc.add_page().unwrap();

    /* draw grid to the page */
    util::print_grid(&doc, &page);

    /* print the title of the page (with positioning center) */
    let page_title = "Text Demo";
    page.set_font_and_size(&font, 24.0);
    let tw = page.text_width(page_title);
    page.begin_text();
    page.text_out((page.width() - tw)/2.0, page.height() - 50.0, page_title);
    page.end_text();

    page.begin_text();
    page.move_text_pos(60.0, page.height() - 60.0);

    /* font size */
    let mut fsize = 8.0;
    while fsize < 60.0 {
        /* set style and size of font */
        page.set_font_and_size(&font, fsize);

        /* set the position of the text */
        page.move_text_pos(0.0, -5.0 - fsize);

        /* measure the number of characters which included in the page. */
        let (len, _real_width) = page.measure_text(samp_text, page.width() - 120.0, false);

        /* truncate the text */
        let samp_text_truncated = &samp_text[..len];

        page.show_text(samp_text_truncated);

        /* print the description */
        page.move_text_pos(0.0, -10.0);
        page.set_font_and_size(&font, 8.0);
        page.show_text(&format!("Fontsize={}", fsize));

        fsize = fsize * 1.5;
    }
    
    /* font color */
    page.set_font_and_size(&font, 8.0);
    page.move_text_pos(0.0, -30.0);
    page.show_text("Font color");

    page.set_font_and_size(&font, 18.0);
    page.move_text_pos(0.0, -20.0);
    for (i, ch) in samp_text.chars().enumerate() {
        let r = (i as f32) / (samp_text.len() as f32);
        let g = 1.0 - ((i as f32) / (samp_text.len() as f32));

        page.set_rgb_fill(r, g, 0.0);
        page.show_text(&format!("{}", ch));
    }
    page.move_text_pos(0.0, -25.0);

    for (i, ch) in samp_text.chars().enumerate() {
        let r = (i as f32) / (samp_text.len() as f32);
        let b = 1.0 - ((i as f32) / (samp_text.len() as f32));

        page.set_rgb_fill(r, 0.0, b);
        page.show_text(&format!("{}", ch));
    }
    page.move_text_pos(0.0, -25.0);

    for (i, ch) in samp_text.chars().enumerate() {
        let b = (i as f32) / (samp_text.len() as f32);
        let g = 1.0 - ((i as f32) / (samp_text.len() as f32));

        page.set_rgb_fill(0.0, g, b);
        page.show_text(&format!("{}", ch));
    }

    page.end_text();

    let ypos = 450.0;

    /* Font rendering mode */
    page.set_font_and_size(&font, 32.0);
    page.set_rgb_fill(0.5, 0.5, 0.0);
    page.set_line_width(1.5);

    /* PDF_FILL */
    show_description(&page, 60.0, ypos, "RenderingMode=PDF_FILL");
    page.set_text_rendering_mode(TextRenderingMode::Fill);
    page.begin_text();
    page.text_out(60.0, ypos, "ABCabc123");
    page.end_text();

    /* PDF_STROKE */
    show_description(&page, 60.0, ypos - 50.0, "RenderingMode=PDF_STROKE");
    page.set_text_rendering_mode(TextRenderingMode::Stroke);
    page.begin_text();
    page.text_out(60.0, ypos - 50.0, "ABCabc123");
    page.end_text();

    /* PDF_FILL_THEN_STROKE */
    show_description(&page, 60.0, ypos - 100.0, "RenderingMode=PDF_FILL_THEN_STROKE");
    page.set_text_rendering_mode(TextRenderingMode::FillThenStroke);
    page.begin_text();
    page.text_out(60.0, ypos - 100.0, "ABCabc123");
    page.end_text();

    /* PDF_FILL_CLIPPING */
    show_description(&page, 60.0, ypos - 150.0, "RenderingMode=PDF_FILL_CLIPPING");
    page.gsave();
    page.set_text_rendering_mode(TextRenderingMode::FillClipping);
    page.begin_text();
    page.text_out(60.0, ypos - 150.0, "ABCabc123");
    page.end_text();
    show_stripe_pattern(&page, 60.0, ypos - 150.0);
    page.grestore();

    /* PDF_STROKE_CLIPPING */
    show_description(&page, 60.0, ypos - 200.0, "RenderingMode=PDF_STROKE_CLIPPING");
    page.gsave();
    page.set_text_rendering_mode(TextRenderingMode::StrokeClipping);
    page.begin_text();
    page.text_out(60.0, ypos - 200.0, "ABCabc123");
    page.end_text();
    show_stripe_pattern(&page, 60.0, ypos - 200.0);
    page.grestore();

    /* PDF_FILL_STROKE_CLIPPING */
    show_description(&page, 60.0, ypos - 250.0, "RenderingMode=PDF_FILL_STROKE_CLIPPING");
    page.gsave();
    page.set_text_rendering_mode(TextRenderingMode::FillStrokeClipping);
    page.begin_text();
    page.text_out(60.0, ypos - 250.0, "ABCabc123");
    page.end_text();
    show_stripe_pattern(&page, 60.0, ypos - 250.0);
    page.grestore();

    /* Reset text attributes */
    page.set_text_rendering_mode(TextRenderingMode::Fill);
    page.set_rgb_fill(0.0, 0.0, 0.0);
    page.set_font_and_size(&font, 30.0);

    /* Rotating text */
    let angle1 = 30.0; /* A rotation of 30 degrees. */
    let rad1: Real = angle1 / 180.0 * 3.141592; /* Calcurate the radian value. */

    show_description(&page, 320.0, ypos - 60.0, "Rotating text");
    page.begin_text();
    page.set_text_matrix(rad1.cos(), rad1.sin(), -rad1.sin(), rad1.cos(), 330.0, ypos - 60.0);
    page.show_text("ABCabc123");
    page.end_text();

    /* skewing text */
    let angle1 = 10.0;
    let angle2 = 20.0;
    let rad1: Real = angle1 / 180.0 * 3.141592;
    let rad2: Real = angle2 / 180.0 * 3.141592;
    show_description(&page, 320.0, ypos - 120.0, "Skewing text");
    page.begin_text();
    page.set_text_matrix(1.0, rad1.tan(), rad2.tan(), 1.0, 320.0, ypos - 120.0);
    page.show_text("ABCabc123");
    page.end_text();

    /* scaling text (X direction) */
    show_description(&page, 320.0, ypos - 175.0, "Scaling text (X direction)");
    page.begin_text();
    page.set_text_matrix(1.5, 0.0, 0.0, 1.0, 320.0, ypos - 175.0);
    page.show_text("ABCabc123");
    page.end_text();

    /* scaling text (Y direction) */
    show_description(&page, 320.0, ypos - 250.0, "Scaling text (Y direction)");
    page.begin_text();
    page.set_text_matrix(1.0, 0.0, 0.0, 2.0, 320.0, ypos - 250.0);
    page.show_text("ABCabc123");
    page.end_text();

    /* char spacing, word spacing */
    show_description(&page, 60.0, 140.0, "char-spacing 0");
    show_description(&page, 60.0, 100.0, "char-spacing 1.5");
    show_description(&page, 60.0, 60.0, "char-spacing 1.5, word-spacing 2.5");

    /* char-spacing 0 */
    page.begin_text();
    page.text_out(60.9, 140.0, samp_text2);
    page.end_text();

    /* char-spacing 1.5 */
    page.set_char_space(1.5);
    page.begin_text();
    page.text_out(60.0, 100.0, samp_text2);
    page.end_text();

    /* char-spacing 1.5, word-spacing 2.5 */
    page.set_word_space(2.5);
    page.begin_text();
    page.text_out(60.0, 60.0, samp_text2);
    page.end_text();

    /* save the document to a file */
    doc.save_to_file("text_demo.pdf");
}
