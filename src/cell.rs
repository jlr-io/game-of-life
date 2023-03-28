#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub enum CellSize {
    Small,
    Medium,
    Large,
}

impl CellSize {
    pub fn width(&self) -> u16 {
        match self {
            CellSize::Small => 2,
            CellSize::Medium => 3,
            CellSize::Large => 5,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            CellSize::Small => 1,
            CellSize::Medium => 2,
            CellSize::Large => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    location: (usize, usize),
    state: CellState,
}
