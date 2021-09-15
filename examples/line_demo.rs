extern crate libharu;
extern crate anyhow;

use libharu::prelude::*;//{Point, Document, LineCap, LineJoin, PageDescriptionMode, PageDescTextCommon};

fn draw_line<T: Into<Point>>(page: &PageDescriptionMode, pos: T, label: &str) -> anyhow::Result<()> {
    let pos = pos.into();

    page.run_text_mode(|page|{
        page.move_text_pos((pos.x, pos.y - 10.0))?;
        page.show_text(label)?;
        Ok(())
    })?;

    page.move_to((pos.x, pos.y - 15.0))?;
    page.line_to((pos.x + 220.0, pos.y - 15.0))?;
    page.stroke()?;

    Ok(())
}

fn draw_line2<T: Into<Point>>(page: &PageDescriptionMode, pos: T, label: &str) -> anyhow::Result<()> {
    let pos = pos.into();

    page.run_text_mode(|page|{
        page.move_text_pos(pos)?;
        page.show_text(label)?;
        Ok(())
    })?;
    
    page.move_to((pos.x + 30.0, pos.y - 25.0))?;
    page.line_to((pos.x + 100.0, pos.y - 25.0))?;
    page.stroke()?;

    Ok(())
}

fn draw_rect<T: Into<Point>>(page: &PageDescriptionMode, pos: T, label: &str) -> anyhow::Result<()> {
    let pos = pos.into();

    page.run_text_mode(|page|{
        page.move_text_pos((pos.x, pos.y -10.0))?;
        page.show_text(label)?;
        Ok(())
    })?;

    page.rectangle(pos.x, pos.y - 40.0, 220.0, 25.0)?;

    Ok(())
}
fn main() -> anyhow::Result<()> {
    // http://libharu.sourceforge.net/demo/line_demo.c
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })?;

    /* create default-font */
    let font = doc.font("Helvetica", None)?;

    /* add a new page object */
    let page = doc.add_page()?;
    let page = PageDescriptionMode::new(&page);

    /* print the lines of the page */
    page.set_line_width(1.0)?;
    page.rectangle(50.0, 50.0, page.width()? - 100.0, page.height()? - 110.0)?;
    page.stroke()?;

    /* print the title of the page (with positioning center) */
    let page_title = "Line Demo";
    page.set_font_and_size(&font, 24.0)?;
    let tw = page.text_width(page_title)?;
    page.run_text_mode(|page|{
        page.move_text_pos(((page.width()? - tw) / 2.0, page.height()? - 50.0))?;
        page.show_text(page_title)?;
        Ok(())
    })?;

    page.set_font_and_size(&font, 10.0)?;

    /* Draw verious widths of lines */
    page.set_line_width(0.0)?;
    draw_line(&page, (60.0, 770.0), "line width = 0")?;

    page.set_line_width(1.0)?;
    draw_line(&page, (60.0, 740.0), "line width = 1.0")?;

    page.set_line_width(2.0)?;
    draw_line(&page, (60.0, 710.0), "line width = 2.0")?;

    /* Line dash pattern */
    page.set_line_width(1.0)?;
    page.set_dash(&[3], 1)?;
    draw_line(&page, (60.0, 680.0), "dash_ptn=[3], phase=1 -- 2 on, 3 off, 3 on...")?;
    page.set_dash(&[3,7], 2)?;
    draw_line(&page, (60.0, 650.0), "dash_ptn=[7,3], phase=2 -- 5 on, 3 off, 7 on...")?;
    page.set_dash(&[8,7,2,7], 0)?;
    draw_line(&page, (60.0, 620.0), "dash_ptn=[8,7,2,7], phase-0")?;

    page.clear_dash()?;

    page.set_line_width(30.0)?;
    AsRef::<PageDescTextCommon>::as_ref(&page).set_rgb_stroke((0.0, 0.5, 0.0))?;

    /* Line Cap Style */
    page.set_line_cap(LineCap::Butt)?;
    draw_line2(&page, (60.0, 570.0), "PDF_BUTT_END")?;

    page.set_line_cap(LineCap::Round)?;
    draw_line2(&page, (60.0, 505.0), "PDF_ROUND_END")?;
    page.set_line_cap(LineCap::ProjectingSquare)?;
    draw_line2(&page, (60.0, 440.0), "PDF_PROJECTING_SQUARE_END")?;

    /* Line Join Style */
    page.set_line_width(30.0)?;
    AsRef::<PageDescTextCommon>::as_ref(&page).set_rgb_stroke((0.0, 0.0, 0.5))?;

    page.set_line_join(LineJoin::Miter)?;
    page.move_to((120.0, 300.0))?;
    page.line_to((160.0, 340.0))?;
    page.line_to((200.0, 300.0))?;
    page.stroke()?;

    page.run_text_mode(|page|{
        page.move_text_pos((60.0, 360.0))?;
        page.show_text("PDF_MITER_JOIN")?;
        Ok(())
    })?;

    page.set_line_join(LineJoin::Round)?;
    page.move_to((120.0, 195.0))?;
    page.line_to((160.0, 235.0))?;
    page.line_to((200.0, 195.0))?;
    page.stroke()?;

    page.run_text_mode(|page|{
        page.move_text_pos((60.0, 255.0))?;
        page.show_text("PDF_ROUND_JOIN")?;
        Ok(())
    })?;

    page.set_line_join(LineJoin::Bevel)?;
    page.move_to((120.0, 90.0))?;
    page.line_to((160.0, 130.0))?;
    page.line_to((200.0, 90.0))?;
    page.stroke()?;

    page.run_text_mode(|page|{
        page.move_text_pos((60.0, 150.0))?;
        page.show_text("PDF_BEVEL_JOIN")?;
        Ok(())
    })?;

    /* Draw Rectangle */
    page.set_line_width(2.0)?;
    AsRef::<PageDescTextCommon>::as_ref(&page).set_rgb_stroke((0.0, 0.0, 0.0))?;
    AsRef::<PageDescTextCommon>::as_ref(&page).set_rgb_fill((0.75, 0.0, 0.0))?;

    draw_rect(&page, (300.0, 770.0), "Stroke")?;
    page.stroke()?;

    draw_rect(&page, (300.0, 720.0), "Fill")?;
    page.fill()?;

    draw_rect(&page, (300.0, 670.0), "Fill then Stroke")?;
    page.fill_stroke()?;
    
    /* Clip Rect */
    page.gsave()?; /* Save the current graphic state */
    draw_rect(&page, (300.0, 620.0), "Clip Rectangle")?;
    page.clip()?;
    page.stroke()?;
    page.set_font_and_size(&font, 13.0)?;

    page.run_text_mode(|page|{
        page.move_text_pos((290.0, 600.0))?;
        page.set_text_leading(12.0)?;
        page.show_text("Clip Clip Clip Clip Clip Clip Clip Clip Clip")?;
        page.show_text_next_line("Clip Clip Clip Clip Clip Clip Clip Clip Clip")?;
        page.show_text_next_line("Clip Clip Clip Clip Clip Clip Clip Clip Clip")?;
        Ok(())
    })?;

    page.grestore()?;

    /* Curve Example(CurveTo2) */
    let x = 330.0;
    let y = 440.0;
    let x1 = 430.0;
    let y1 = 530.0;
    let x2 = 480.0;
    let y2 = 470.0;
    let x3 = 480.0;
    let y3 = 90.0;

    AsRef::<PageDescTextCommon>::as_ref(&page).set_rgb_fill((0.0, 0.0, 0.0))?;

    page.run_text_mode(|page|{
        page.move_text_pos((300.0, 540.0))?;
        page.show_text("CurveTo2(x1, y1, x2. y2)")?;
        Ok(())
    })?;

    page.run_text_mode(|page|{
        page.move_text_pos((x + 5.0, y - 5.0))?;
        page.show_text("Current point")?;
        page.move_text_pos((x1 - x, y1 - y))?;
        page.show_text("(x1, y1)")?;
        page.move_text_pos((x2 - x1, y2 - y1))?;
        page.show_text("(x2, y2)")?;
        Ok(())
    })?;

    page.set_dash(&[3], 0)?;

    page.set_line_width(0.5)?;
    page.move_to((x1, y1))?;
    page.line_to((x2, y2))?;
    page.stroke()?;

    page.clear_dash()?;

    page.set_line_width(1.5)?;

    page.move_to((x, y))?;
    page.curve_to_2((x1, y1), (x2, y2))?;
    page.stroke()?;

    /* Curve Example(CurveTo3) */
    let y = y - 150.0;
    let y1 = y1 - 150.0;
    let y2 = y2 - 150.0;

    page.run_text_mode(|page|{
        page.move_text_pos((300.0, 390.0))?;
        page.show_text("CurveTo3(x1, y1, x2, y2)")?;
        Ok(())
    })?;

    page.run_text_mode(|page|{
        page.move_text_pos((x + 5.0, y - 5.0))?;
        page.show_text("Current point")?;
        page.move_text_pos((x1 - x, y1 - y))?;
        page.show_text("(x1, y1)")?;
        page.move_text_pos((x2 - x1, y2 - y1))?;
        page.show_text("(x2, y2)")?;
        Ok(())
    })?;

    page.set_dash(&[3], 0)?;

    page.set_line_width(0.5)?;
    page.move_to((x, y))?;
    page.line_to((x1, y1))?;
    page.stroke()?;

    page.clear_dash()?;
    
    page.set_line_width(1.5)?;
    page.move_to((x, y))?;
    page.curve_to_3((x1, y1), (x2, y2))?;
    page.stroke()?;

    /* Curve Example(CurveTo) */
    let y = y - 150.0;
    let y1 = y1 - 160.0;
    let y2 = y2 - 130.0;
    let x2 = x2 + 10.0;

    page.run_text_mode(|page|{
        page.move_text_pos((300.0, 240.0))?;
        page.show_text("CurveTo(x1, y1, x2, y2, x3, y3)")?;
        Ok(())
    })?;

    page.run_text_mode(|page|{
        page.move_text_pos((x + 5.0, y - 5.0))?;
        page.show_text("Current point")?;
        page.move_text_pos((x1 - x, y1 - y))?;
        page.show_text("(x1, y1)")?;
        page.move_text_pos((x2 - x1, y2 - y1))?;
        page.show_text("(x2, y2)")?;
        page.move_text_pos((x3 - x2, y3 - y2))?;
        page.show_text("(x3, y3)")?;
        Ok(())
    })?;

    page.set_dash(&[3], 1)?;

    page.set_line_width(0.5)?;
    page.move_to((x, y))?;
    page.line_to((x1, y1))?;
    page.stroke()?;
    page.move_to((x2, y2))?;
    page.line_to((x3, y3))?;
    page.stroke()?;

    page.clear_dash()?;

    page.set_line_width(1.5)?;
    page.move_to((x, y))?;
    page.curve_to((x1, y1), (x2, y2), (x3, y3))?;
    page.stroke()?;

    /* save the document to a file */
    doc.save_to_file("line_demo.pdf")?;

    Ok(())
}
