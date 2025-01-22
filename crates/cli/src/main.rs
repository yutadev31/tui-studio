use anyhow::Result;
use crossterm::event;
use editor::Editor;
use utils::term::init_term;

fn main() -> Result<()> {
    init_term()?;

    let mut editor = Editor::new()?;
    editor.draw()?;

    loop {
        editor.on_event(event::read()?)?;
        editor.draw()?;
    }
}
