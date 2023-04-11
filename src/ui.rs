use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app;

pub fn setup_game<B: Backend>(frame: &mut Frame<B>, app: &app::App) -> Rect {
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
        .constraints([Constraint::Percentage(90)])
        .split(terminal);
    let screen = screen_layout[0];

    // figure out how many cells can fit on the screen
    let cell_rows = screen.height / app.cell_size.height();
    let cell_columns = screen.width / app.cell_size.width();

    // height and width of the game
    let game_height = cell_rows * app.cell_size.height();
    let game_width = cell_columns * app.cell_size.width();

    // let game_summary = screen_layout[1];

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
    game
}

pub fn draw_game_summary<B: Backend>(frame: &mut Frame<B>, game_summary: Rect, summary: String) {
    let summary_paragraph = Paragraph::new(summary);
    frame.render_widget(summary_paragraph, game_summary);
}

pub fn draw_cells<B: Backend>(app: &app::App, frame: &mut Frame<B>) {
    for y in 0..app.get_cell_rows() as u16 {
        for x in 0..app.get_cell_columns() as u16 {
            let cell = match app.cells[y as usize][x as usize].is_alive() {
                true => Block::default().style(Style::default().bg(Color::Green).fg(Color::White)),
                false => Block::default().style(Style::default().bg(Color::Black).fg(Color::Black)),
            };
            frame.render_widget(
                cell,
                Rect::new(
                    x as u16 * app.cell_size.width() + 2, // +2 is to offset the border
                    y as u16 * app.cell_size.height() + 2,
                    app.cell_size.width(),
                    app.cell_size.height(),
                ),
            )
        }
    }
}
