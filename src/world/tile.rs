use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    Door,
    StairsDown,
    StairsUp,
    Chest,
    Exit,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Floor
            | TileType::Door
            | TileType::StairsDown
            | TileType::StairsUp
            | TileType::Chest
            | TileType::Exit => true,
            TileType::Wall => false,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            TileType::Wall => '#',
            TileType::Floor => '.',
            TileType::Door => '+',
            TileType::StairsDown => '>',
            TileType::StairsUp => '<',
            TileType::Chest => 'C',
            TileType::Exit => 'E',
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub explored: bool,
    pub visible: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            explored: false,
            visible: false,
        }
    }

    pub fn wall() -> Self {
        Tile::new(TileType::Wall)
    }

    pub fn floor() -> Self {
        Tile::new(TileType::Floor)
    }

    pub fn door() -> Self {
        Tile::new(TileType::Door)
    }

    pub fn stairs_down() -> Self {
        Tile::new(TileType::StairsDown)
    }

    pub fn stairs_up() -> Self {
        Tile::new(TileType::StairsUp)
    }

    pub fn chest() -> Self {
        Tile::new(TileType::Chest)
    }

    pub fn exit() -> Self {
        Tile::new(TileType::Exit)
    }

    pub fn render(&self) -> char {
        if !self.explored {
            return ' ';
        }

        if self.visible {
            self.tile_type.symbol()
        } else {
            match self.tile_type {
                TileType::Wall => '#',
                TileType::Floor => '.',
                TileType::Door => '+',
                TileType::StairsDown => '>',
                TileType::StairsUp => '<',
                TileType::Chest => 'C',
                TileType::Exit => 'E',
            }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
