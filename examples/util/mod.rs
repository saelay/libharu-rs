extern crate libharu;
extern crate anyhow;

use libharu::prelude::*;//{Document, PageDescriptionMode, PageDescTextCommon};

pub fn print_grid(doc: &Document, page: &PageDescriptionMode) -> anyhow::Result<()> {
    let height = page.height()?;
    let width = page.width()?;
    let font = doc.font("Helvetica", None)?;

    page.set_font_and_size(&font, 5.0)?;
    page.set_gray_fill(0.5)?;
    page.set_gray_stroke(0.8)?;

    /* draw horizontal lines */
    let mut y = 0.0;
    while y < height {
        if (y as u32) % 10 == 0 {
            page.set_line_width(0.5)?;
        }
        else {
            if page.line_width() != 0.25 {
                page.set_line_width(0.25)?;
            }
        }

        page.run_path_mode(|page|{
            page.move_to((0.0, y))?;
            page.line_to((width, y))?;
            page.stroke()?;
            Ok(())
        })?;

        if ((y as u32) % 10 == 0) && (y > 0.0) {
            page.set_gray_stroke(0.5)?;

            page.run_path_mode(|page|{
                page.move_to((0.0, y))?;
                page.line_to((5.0, y))?;
                page.stroke()?;
                Ok(())
            })?;

            page.set_gray_stroke(0.8)?;
        }
        y = y + 5.0;
    }

    /* draw vertical lines */
    let mut x = 0.0;
    while x < width {
        if ((x as u32) % 10) == 0 {
            page.set_line_width(0.5)?;
        }
        else {
            if page.line_width() != 0.25 {
                page.set_line_width(0.25)?;
            }
        }

        page.run_path_mode(|page|{
            page.move_to((x, 0.0))?;
            page.line_to((x, height))?;
            page.stroke()?;
            Ok(())
        })?;

        if ((x as u32) == 0) && (x > 0.0) {
            page.set_gray_stroke(0.5)?;

            page.run_path_mode(|page|{
                page.move_to((x, 0.0))?;
                page.line_to((x, 5.0))?;
                page.stroke()?;
    
                page.move_to((x, height))?;
                page.line_to((x, height - 5.0))?;
                page.stroke()?;
    
                Ok(())
            })?;
            
            page.set_gray_stroke(0.8)?;
        }

        x = x + 5.0;
    }

    /* Draw horizontal text */
    let mut y = 0.0;
    while y < height {
        if ((y as u32) % 10 == 0) && (y > 0.0) {
            page.run_text_mode(|page|{
                page.move_text_pos((5.0, y - 2.0))?;
                let text = format!("{}", y as u32);
                page.show_text(&text)?;
                Ok(())
            })?;
        }

        y = y + 5.0;
    }

    /* Draw virtical text */
    let mut x = 0.0;
    while x < width {
        if ((x as u32) %10 == 0) && (x > 0.0) {
            let text = format!("{}", x as u32);
            page.run_text_mode(|page|{
                page.move_text_pos((x, 5.0))?;
                page.show_text(&text)?;
                Ok(())
            })?;

            page.run_text_mode(|page|{
                page.move_text_pos((x, height - 10.0))?;
                page.show_text(&text)?;
                Ok(())
            })?;
        }

        x = x + 5.0;
    }

    page.set_gray_fill(0.0)?;
    page.set_gray_stroke(0.0)?;

    Ok(())
}
