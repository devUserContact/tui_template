use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

enum InputMode {
    Normal,
    Editing,
}
struct App {
    input: Input,
    input_mode: InputMode,
    messages: Vec<String>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Test App")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title_alignment(Alignment::Left)
        .title("Test Box")
        .style(Style::default());
    f.render_widget(block, left_chunks[0]);

    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Test Box")
        .title_alignment(Alignment::Left);
    f.render_widget(block, left_chunks[1]);

    // Bottom two inner blocks
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let block = Block::default()
        .border_style(Style::default().fg(Color::Magenta))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Test Box")
        .title_alignment(Alignment::Left);
    f.render_widget(block, right_chunks[0]);
    // Bottom right block with styled left and right border
}
