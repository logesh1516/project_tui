use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoList>,
}

#[derive(Debug, Default)]
struct TodoList {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let state = AppState::default();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    Paragraph::new("hello from the applications ").render(frame.area(), frame.buffer_mut());
    Block::bordered().render(frame.area(), frame.buffer_mut());
}
