use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, Stdout},
};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod cell;
mod ui;

type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

fn setup_terminal() -> io::Result<CrosstermTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: CrosstermTerminal) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    // create app and run it
    let mut app = app::App::default();
    let res = app.run(&mut terminal);

    // restore terminal
    restore_terminal(terminal)?;

    if let Err(err) = res {
        println!("{:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}
