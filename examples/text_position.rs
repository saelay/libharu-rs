use libharu::prelude::*;//{Document, PageDescTextCommon};

fn a<T:FnOnce()>(t: T) {

}
fn main() -> anyhow::Result<()> {
    
    /* craete new pdf document */
    let doc = Document::new()?;

    let mut page = doc.add_page()?;

    let pos = page.current_text_pos()?;
    println!("current text pos: ({}, {})", pos.x, pos.y);
    //a(page);
    page.show_text("unko morimori")?;

    page.run_text_mode(|page|{
        page.move_text_pos((100.0, 100.0))?;
        Ok(())
    })?;

    Ok(())
}
