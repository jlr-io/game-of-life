mod ui;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use ratatui::{backend::Backend, layout::Rect, Terminal};

pub mod cell;
use cell::{Cell, CellSize, CellState};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = cell::CellSize::Large)]
    pub cell_size: cell::CellSize
}

pub struct Game {
    pub tick_rate: Duration,
    pub cell_size: CellSize,
    pub cells: Vec<Vec<Cell>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
            cell_size: CellSize::Small,
            cells: vec![],
        }
    }
}

impl Game {
    pub fn new(cell_size: CellSize) -> Game {
        Game {
            tick_rate: Duration::from_millis(250),
            cell_size,
            cells: vec![]
        }
    }
    pub fn get_cell_rows(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell_columns(&self) -> usize {
        self.cells[0].len()
    }

    pub fn set_cells_dimensions(&mut self, game: &Rect) {
        let rows = game.height / self.cell_size.height();
        let columns = game.width / self.cell_size.width();
        self.cells.resize(rows.into(), vec![]);
        for row in self.cells.iter_mut() {
            row.resize(columns.into(), Cell::new((0, 0), CellState::Dead));
        }
    }

    pub fn initialize_cells(&mut self) {
        let mut cells: Vec<Vec<Cell>> = vec![];
        for x in 0..self.get_cell_rows() {
            let mut row: Vec<Cell> = vec![];
            for y in 0..self.get_cell_columns() {
                let rand = rand::random::<u8>();
                let state = if rand % 2 == 0 {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
                let cell = Cell::new((x, y), state);
                row.push(cell);
            }
            cells.push(row);
        }
        self.cells = cells;
    }

    pub fn get_neighbors(&self, cell: &Cell) -> Vec<Cell> {
        let max_row = self.get_cell_rows();
        let max_column = self.get_cell_columns();

        let left = match cell.location {
            (_, 0) => None,
            (_, _) => self
                .cells
                .get(cell.location.0)
                .and_then(|row| row.get(cell.location.1 - 1)),
        };

        let right = match cell.location {
            _ if cell.location.1 == max_column => None,
            (_, _) => self
                .cells
                .get(cell.location.0)
                .and_then(|row| row.get(cell.location.1 + 1)),
        };

        let up = match cell.location {
            (0, _) => None,
            (_, _) => self
                .cells
                .get(cell.location.0 - 1)
                .and_then(|row| row.get(cell.location.1)),
        };

        let down = match cell.location {
            _ if cell.location.0 == max_row => None,
            (_, _) => self
                .cells
                .get(cell.location.0 + 1)
                .and_then(|row| row.get(cell.location.1)),
        };

        vec![left, right, up, down]
            .into_iter()
            .filter(|c| c.is_some())
            .map(|c| *c.unwrap())
            .collect()
    }

    // Any live cell with two or three live neighbours survives.
    // Any dead cell with three live neighbours becomes a live cell.
    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    pub fn update_cells(&mut self) -> Vec<Vec<Cell>> {
        let mut new_cells: Vec<Vec<Cell>> = vec![];
        for row in self.cells.iter() {
            let mut new_row: Vec<Cell> = vec![];
            for cell in row.iter() {
                let neighbors = self.get_neighbors(cell);
                let alive_neighbors = neighbors.iter().filter(|c| c.is_alive()).count();
                let new_cell = match cell.state {
                    CellState::Alive => match alive_neighbors {
                        2 | 3 => *cell,
                        _ => Cell::new(cell.location, CellState::Dead),
                    },
                    CellState::Dead => match alive_neighbors {
                        3 => Cell::new(cell.location, CellState::Alive),
                        _ => *cell,
                    },
                };
                new_row.push(new_cell);
            }
            new_cells.push(new_row);
        }
        new_cells
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut tick = Instant::now();

        loop {
            terminal.draw(|f| {
                let game = ui::setup_game(f, self);
                // TODO: support resizing
                if self.cells.len() == 0 {
                    self.set_cells_dimensions(&game);
                    self.initialize_cells();
                }
                ui::draw_cells(self, f);
                self.cells = self.update_cells();
            })?;

            if crossterm::event::poll(self.tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }

            if tick.elapsed() >= self.tick_rate {
                tick = Instant::now();
            }
        }
    }
}
