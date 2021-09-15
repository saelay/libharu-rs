extern crate libharu;
extern crate anyhow;

use libharu::prelude::*;//{Document};

fn main() -> anyhow::Result<()> {
    // http://libharu.sourceforge.net/demo/font_demo.c
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    }).unwrap();

    let page = doc.add_page()?;
    let page = PageDescriptionMode::new(&page);

    let height = page.height()?;
    let width = page.width()?;
    
    /* Print the lines of the page. */
    page.set_line_width(1.0)?;
    page.rectangle(50.0, 50.0, width - 100.0, height - 110.0)?;
    page.stroke()?;

    /* Print the title of the page (with positioning center). */
    let def_font = doc.font("Helvetica", None)?;
    page.set_font_and_size(&def_font, 24.0)?;

    let page_title = "Font Demo";
    let tw = page.text_width(page_title)?;
    page.run_text_mode(|page|{
        page.text_out(((width-tw)/2.0, height -50.0), page_title)?;
        Ok(())
    })?;

    /* output subtitle. */
    page.run_text_mode(|page|{
        page.set_font_and_size(&def_font, 16.0)?;
        page.text_out((60.0, height-80.0), "<Standard Type1 fonts samples>")?;

        Ok(())
    })?;

    page.run_text_mode(|page|{
        page.move_text_pos((60.0, height - 105.0))?;
    
        let font_list = [
            "Courier",
            "Courier-Bold",
            "Courier-Oblique",
            "Courier-BoldOblique",
            "Helvetica",
            "Helvetica-Bold",
            "Helvetica-Oblique",
            "Helvetica-BoldOblique",
            "Times-Roman",
            "Times-Bold",
            "Times-Italic",
            "Times-BoldItalic",
            "Symbol",
            "ZapfDingbats",
        ];
    
        for font_name in font_list {
            let samp_text = "abcdefgABCDEFG12345!#$%&+-@?";
            let font = doc.font(font_name, None).unwrap();
    
            /* print a label of text */
            page.set_font_and_size(&def_font, 9.0)?;
            page.show_text(font_name)?;
            page.move_text_pos((0.0, -18.0))?;
    
            /* print a sample text. */
            page.set_font_and_size(&font, 20.0)?;
            page.show_text(samp_text)?;
            page.move_text_pos((0.0, -20.0))?;
        }
        
        Ok(())
    })?;

    doc.save_to_file("font_demo.pdf")?;

    Ok(())
}
