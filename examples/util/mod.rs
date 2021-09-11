extern crate libharu;
extern crate anyhow;

use libharu::{Document, Page};

pub fn print_grid(doc: &Document, page: &Page) -> anyhow::Result<()> {
    let height = page.height();
    let width = page.width();
    let font = doc.font("Helvetica", None).unwrap();

    page.set_font_and_size(&font, 5.0);
    page.set_gray_fill(0.5);
    page.set_gray_stroke(0.8);

    /* draw horizontal lines */
    let mut y = 0.0;
    while y < height {
        if (y as u32) % 10 == 0 {
            page.set_line_width(0.5);
        }
        else {
            if page.line_width() != 0.25 {
                page.set_line_width(0.25);
            }
        }

        page.move_to((0.0, y))?;
        page.line_to((width, y))?;
        page.stroke();

        if ((y as u32) % 10 == 0) && (y > 0.0) {
            page.set_gray_stroke(0.5);

            page.move_to((0.0, y))?;
            page.line_to((5.0, y))?;
            page.stroke();

            page.set_gray_stroke(0.8);
        }
        y = y + 5.0;
    }

    /* draw vertical lines */
    let mut x = 0.0;
    while x < width {
        if ((x as u32) % 10) == 0 {
            page.set_line_width(0.5);
        }
        else {
            if page.line_width() != 0.25 {
                page.set_line_width(0.25);
            }
        }

        page.move_to((x, 0.0))?;
        page.line_to((x, height))?;
        page.stroke();

        if ((x as u32) == 0) && (x > 0.0) {
            page.set_gray_stroke(0.5);

            page.move_to((x, 0.0))?;
            page.line_to((x, 5.0))?;
            page.stroke();

            page.move_to((x, height))?;
            page.line_to((x, height - 5.0))?;
            page.stroke();

            page.set_gray_stroke(0.8);
        }

        x = x + 5.0;
    }

    /* Draw horizontal text */
    let mut y = 0.0;
    while y < height {
        if ((y as u32) % 10 == 0) && (y > 0.0) {
            page.begin_text();
            page.move_text_pos((5.0, y - 2.0))?;
            let text = format!("{}", y as u32);
            page.show_text(&text);
            page.end_text();
        }

        y = y + 5.0;
    }

    /* Draw virtical text */
    let mut x = 0.0;
    while x < width {
        if ((x as u32) %10 == 0) && (x > 0.0) {
            page.begin_text();
            page.move_text_pos((x, 5.0))?;
            let text = format!("{}", x as u32);
            page.show_text(&text);
            page.end_text();

            page.begin_text();
            page.move_text_pos((x, height - 10.0))?;
            page.show_text(&text);
            page.end_text();
        }

        x = x + 5.0;
    }

    page.set_gray_fill(0.0);
    page.set_gray_stroke(0.0);

    Ok(())
}
