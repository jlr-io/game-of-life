use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

struct App {
    tick_rate: Duration,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App {
        tick_rate: Duration::from_millis(250),
    };
    let res = run_app(&mut terminal, &app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
    let mut tick = Instant::now();
    // used for testing
    let mut i = 0;
    loop {
        terminal.draw(|f| ui(f, i))?;

        if crossterm::event::poll(app.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if tick.elapsed() > app.tick_rate {
            tick = Instant::now();
            i += 1;
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, i: i32) {
    let terminal = frame.size();

    let game_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Game of Life",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    frame.render_widget(game_block, terminal);

    let screen_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(terminal);
    let screen = screen_layout[0];

    let cell_height = 1;
    let cell_width = 2;

    // figure out how many cells can fit on the screen
    let num_cells_y = screen.height / cell_height;
    let num_cells_x = screen.width / cell_width;

    // height and width of the game
    let game_height = num_cells_y * cell_height;
    let game_width = num_cells_x * cell_width;

    let game_summary = Paragraph::new(format!(
        "Game Summary
				 game height: {}, game width: {},
				 i: {}
				",
        game_height, game_width, i
    ));
    frame.render_widget(game_summary, screen_layout[1]);

    // padding needed to center the game so none of the cells are cut off or elongated
    let vertical_padding = f32::from((screen.height - game_height) / 2);

    // floor and ceil are used to add more padding to the bottom if the padding is not even
    let top_padding_height = vertical_padding.floor() as u16;
    let bottom_padding_height = vertical_padding.ceil() as u16;

    // padding needed to center the game so none of the cells are cut off or elongated
    let horizontal_padding = f32::from((screen.width - game_width) / 2);

    // floor and ceil are used to add more padding to the right if the padding is not even
    let left_padding_width = horizontal_padding.floor() as u16;
    let right_padding_width = horizontal_padding.ceil() as u16;

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top_padding_height),
            Constraint::Length(game_height), //screen height
            Constraint::Length(bottom_padding_height),
        ])
        .split(screen_layout[0]);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left_padding_width),
            Constraint::Length(game_width), // screen width
            Constraint::Length(right_padding_width),
        ])
        .split(vertical_layout[1]);

    let game = horizontal_layout[1];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(cell_height); num_cells_y.into()])
        .split(game);

    for (x, row) in rows.into_iter().enumerate() {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(cell_width); num_cells_x.into()])
            .split(*row);
        for (y, column) in columns.into_iter().enumerate() {
            let cell = match (x + y + i as usize) % 2 {
                0 => Block::default().style(Style::default().bg(Color::White).fg(Color::Black)),
                _ => Block::default().style(Style::default().bg(Color::Black).fg(Color::White)),
            };

            frame.render_widget(cell, *column)
        }
    }
}
