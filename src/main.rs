mod game;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{Game};
use std::{
    error::Error,
    io::{self, Stdout},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let args = game::Args::parse();
    
    let mut terminal = setup_terminal()?;

    // // create app and run it
    let mut game = Game::new(args.cell_size);
    let res = game.run(&mut terminal);

    // // restore terminal
    restore_terminal(terminal)?;

    if let Err(err) = res {
        println!("{:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

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