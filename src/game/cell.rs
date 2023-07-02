use std::{str::FromStr, fmt};

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

impl FromStr for CellSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "small" | "sm" => Ok(CellSize::Small),
            "medium" | "md" => Ok(CellSize::Medium),
            "large" | "lg" => Ok(CellSize::Large),
            _ => Err(()),
        }
    }
}

impl From<&str> for CellSize {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" | "sm" => CellSize::Small,
            "medium" | "md" => CellSize::Medium,
            "large" | "lg" => CellSize::Large,
            _ => panic!("Invalid value! Enter \"small\", \"medium\", or \"large\"."),
        }
    }
}

impl fmt::Display for CellSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellSize::Small => write!(f, "small"),
            CellSize::Medium => write!(f, "medium"),
            CellSize::Large => write!(f, "large"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub location: (usize, usize),
    pub state: CellState,
}

impl Cell {
    pub fn new(location: (usize, usize), state: CellState) -> Self {
        Self { location, state }
    }

    pub fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            CellState::Dead => false,
        }
    }
}