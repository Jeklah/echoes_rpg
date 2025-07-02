use crate::item::Item;
use crate::world::{DungeonType, Enemy, Tile, TileType};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 45;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn distance(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    pub fn center(&self) -> Position {
        Position::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
    pub width: usize,
    pub height: usize,
    pub enemies: HashMap<Position, Enemy>,
    pub items: HashMap<Position, Item>,
    pub stairs_down: Option<Position>,
    pub stairs_up: Option<Position>,
    pub level_num: u32,
    pub player_position: Position,
    pub revealed_tiles: Vec<Vec<bool>>,
    pub visible_tiles: Vec<Vec<bool>>,
    pub exit_position: Option<Position>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::wall(); width]; height];
        let revealed_tiles = vec![vec![false; width]; height];
        let visible_tiles = vec![vec![false; width]; height];

        Level {
            tiles,
            rooms: Vec::new(),
            width,
            height,
            enemies: HashMap::new(),
            items: HashMap::new(),
            stairs_down: None,
            stairs_up: None,
            level_num: 1,
            player_position: Position::new(0, 0),
            revealed_tiles,
            visible_tiles,
            exit_position: None,
        }
    }

    pub fn generate(
        difficulty: u32,
        level_num: u32,
        _dungeon_type: DungeonType,
        is_final: bool,
    ) -> Self {
        let mut level = Level::new(MAP_WIDTH, MAP_HEIGHT);
        level.level_num = level_num;

        // Generate rooms
        let max_rooms = 10 + (difficulty / 2).min(15) as i32;
        let min_size = 5;
        let max_size = 12;

        let mut rng = rand::thread_rng();

        for _ in 0..max_rooms {
            let w = rng.gen_range(min_size..=max_size);
            let h = rng.gen_range(min_size..=max_size);
            let x = rng.gen_range(1..(MAP_WIDTH as i32 - w - 1));
            let y = rng.gen_range(1..(MAP_HEIGHT as i32 - h - 1));

            let new_room = Room::new(x, y, w, h);

            let mut overlap = false;
            for other_room in &level.rooms {
                if new_room.intersects(other_room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // Create room
                level.create_room(&new_room);

                // Connect to previous room
                if !level.rooms.is_empty() {
                    let (new_x, new_y) = {
                        let center = new_room.center();
                        (center.x, center.y)
                    };

                    let (prev_x, prev_y) = {
                        let center = level.rooms[level.rooms.len() - 1].center();
                        (center.x, center.y)
                    };

                    // Randomly decide if we go horizontal first or vertical first
                    if rng.gen_bool(0.5) {
                        level.create_horizontal_tunnel(prev_x, new_x, prev_y);
                        level.create_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        level.create_vertical_tunnel(prev_y, new_y, prev_x);
                        level.create_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                // Place doors
                level.place_doors(&new_room);

                // Store the room
                level.rooms.push(new_room);
            }
        }

        // Place player in the first room
        let player_pos = level.rooms[0].center();
        level.player_position = player_pos;

        // Place stairs
        if !is_final {
            let stairs_pos = level.rooms[level.rooms.len() - 1].center();
            level.tiles[stairs_pos.y as usize][stairs_pos.x as usize] = Tile::stairs_down();
            level.stairs_down = Some(stairs_pos);
        } else {
            // Final level has an exit instead of stairs
            let exit_pos = level.rooms[level.rooms.len() - 1].center();
            level.tiles[exit_pos.y as usize][exit_pos.x as usize] = Tile::exit();
            level.exit_position = Some(exit_pos);
        }

        if level_num > 1 {
            // Add stairs up in the first room, but not at the player's position
            let mut stairs_up_pos = level.rooms[0].center();
            stairs_up_pos.x += 1; // Place it next to the player
            level.tiles[stairs_up_pos.y as usize][stairs_up_pos.x as usize] = Tile::stairs_up();
            level.stairs_up = Some(stairs_up_pos);
        }

        // Place enemies
        level.place_enemies(difficulty);

        // Place items and chests
        level.place_items(difficulty);

        level
    }

    fn create_room(&mut self, room: &Room) {
        for y in (room.y1 + 1)..room.y2 {
            for x in (room.x1 + 1)..room.x2 {
                if (y as usize) > 0
                    && (y as usize) < self.height
                    && (x as usize) > 0
                    && (x as usize) < self.width
                {
                    self.tiles[y as usize][x as usize] = Tile::floor();
                }
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
            if (y as usize) > 0
                && (y as usize) < self.height
                && (x as usize) > 0
                && (x as usize) < self.width
            {
                self.tiles[y as usize][x as usize] = Tile::floor();
            }
        }
    }

    fn create_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
            if (y as usize) > 0
                && (y as usize) < self.height
                && (x as usize) > 0
                && (x as usize) < self.width
            {
                self.tiles[y as usize][x as usize] = Tile::floor();
            }
        }
    }

    fn place_doors(&mut self, room: &Room) {
        let mut rng = rand::thread_rng();

        // Try to place a door on each side of the room with some randomness
        if rng.gen_bool(0.7) {
            let x = rng.gen_range((room.x1 + 1)..room.x2);
            if self.is_valid_door_position(x, room.y1) {
                self.tiles[room.y1 as usize][x as usize] = Tile::door();
            }
        }

        if rng.gen_bool(0.7) {
            let x = rng.gen_range((room.x1 + 1)..room.x2);
            if self.is_valid_door_position(x, room.y2) {
                self.tiles[room.y2 as usize][x as usize] = Tile::door();
            }
        }

        if rng.gen_bool(0.7) {
            let y = rng.gen_range((room.y1 + 1)..room.y2);
            if self.is_valid_door_position(room.x1, y) {
                self.tiles[y as usize][room.x1 as usize] = Tile::door();
            }
        }

        if rng.gen_bool(0.7) {
            let y = rng.gen_range((room.y1 + 1)..room.y2);
            if self.is_valid_door_position(room.x2, y) {
                self.tiles[y as usize][room.x2 as usize] = Tile::door();
            }
        }
    }

    fn is_valid_door_position(&self, x: i32, y: i32) -> bool {
        if y as usize == 0
            || y as usize >= self.height - 1
            || x as usize == 0
            || x as usize >= self.width - 1
        {
            return false;
        }

        let has_floor_adjacent = (self.tiles[y as usize - 1][x as usize].tile_type
            == TileType::Floor)
            || (self.tiles[y as usize + 1][x as usize].tile_type == TileType::Floor)
            || (self.tiles[y as usize][x as usize - 1].tile_type == TileType::Floor)
            || (self.tiles[y as usize][x as usize + 1].tile_type == TileType::Floor);

        let has_wall_adjacent = (self.tiles[y as usize - 1][x as usize].tile_type
            == TileType::Wall)
            || (self.tiles[y as usize + 1][x as usize].tile_type == TileType::Wall)
            || (self.tiles[y as usize][x as usize - 1].tile_type == TileType::Wall)
            || (self.tiles[y as usize][x as usize + 1].tile_type == TileType::Wall);

        has_floor_adjacent && has_wall_adjacent
    }

    fn place_enemies(&mut self, difficulty: u32) {
        let mut rng = rand::thread_rng();

        // Skip the first room (player's starting position)
        for i in 1..self.rooms.len() {
            let room = &self.rooms[i];

            // Number of enemies increases with difficulty and room size
            let room_area = room.width() * room.height();
            let num_enemies = ((room_area as f32 * 0.01 * difficulty as f32).round() as u32).min(5);

            for _ in 0..num_enemies {
                let x = rng.gen_range((room.x1 + 1)..room.x2);
                let y = rng.gen_range((room.y1 + 1)..room.y2);

                let pos = Position::new(x, y);

                // Don't place enemies on stairs or other enemies
                if (Some(pos) != self.stairs_down)
                    && (Some(pos) != self.stairs_up)
                    && (!self.enemies.contains_key(&pos))
                {
                    // Generate enemy based on difficulty and level number
                    let enemy = Enemy::generate_random(self.level_num, difficulty);

                    self.enemies.insert(pos, enemy);
                }
            }
        }
    }

    fn place_items(&mut self, _difficulty: u32) {
        let mut rng = rand::thread_rng();

        // Place chests and items in random rooms (but not the first)
        for i in 1..self.rooms.len() {
            let room = &self.rooms[i];

            // 30% chance of chest
            if rng.gen_bool(0.3) {
                // Find a spot for the chest
                let mut chest_x = rng.gen_range((room.x1 + 1)..room.x2);
                let mut chest_y = rng.gen_range((room.y1 + 1)..room.y2);
                let mut chest_pos = Position::new(chest_x, chest_y);

                // Make sure we're not placing on top of stairs, enemies, or player
                while (Some(chest_pos) == self.stairs_down)
                    || (Some(chest_pos) == self.stairs_up)
                    || (self.enemies.contains_key(&chest_pos))
                    || (chest_pos == self.player_position)
                {
                    chest_x = rng.gen_range((room.x1 + 1)..room.x2);
                    chest_y = rng.gen_range((room.y1 + 1)..room.y2);
                    chest_pos = Position::new(chest_x, chest_y);
                }

                // Place chest
                self.tiles[chest_y as usize][chest_x as usize] = Tile::chest();

                // Also place an item in the chest location that will be collected when chest is opened
                let item = Item::generate_random(self.level_num);
                self.items.insert(chest_pos, item);
            }

            // Maybe place some loose items too (20% chance)
            if rng.gen_bool(0.2) {
                let x = rng.gen_range((room.x1 + 1)..room.x2);
                let y = rng.gen_range((room.y1 + 1)..room.y2);
                let pos = Position::new(x, y);

                // Don't place on stairs, enemies, chests, or player
                if (Some(pos) != self.stairs_down)
                    && (Some(pos) != self.stairs_up)
                    && (!self.enemies.contains_key(&pos))
                    && (self.tiles[y as usize][x as usize].tile_type != TileType::Chest)
                    && (pos != self.player_position)
                {
                    let item = Item::generate_random(self.level_num);
                    self.items.insert(pos, item);
                }
            }
        }
    }

    pub fn is_position_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn is_position_walkable(&self, pos: Position) -> bool {
        // Check if the position is valid and the tile type is walkable
        self.is_position_valid(pos.x, pos.y)
            && self.tiles[pos.y as usize][pos.x as usize]
                .tile_type
                .is_walkable()
            // Don't consider positions with enemies as walkable
            && !self.enemies.contains_key(&pos)
    }

    // This method only checks if the tile is walkable, ignoring enemies
    pub fn is_tile_walkable(&self, pos: Position) -> bool {
        self.is_position_valid(pos.x, pos.y)
            && self.tiles[pos.y as usize][pos.x as usize]
                .tile_type
                .is_walkable()
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if self.is_position_valid(x, y) {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Gets a tile at the specified position
    pub fn get_tile_at(&self, pos: &Position) -> Option<&Tile> {
        self.get_tile(pos.x, pos.y)
    }

    // We already have is_tile_walkable method defined above
    // This is just a helper that uses get_tile_at
    pub fn is_position_walkable_by_ref(&self, pos: &Position) -> bool {
        if let Some(tile) = self.get_tile_at(pos) {
            tile.tile_type.is_walkable()
        } else {
            false
        }
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if self.is_position_valid(x, y) {
            Some(&mut self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_enemy_at(&self, pos: &Position) -> Option<&Enemy> {
        self.enemies.get(pos)
    }

    pub fn get_enemy_at_mut(&mut self, pos: &Position) -> Option<&mut Enemy> {
        self.enemies.get_mut(pos)
    }

    pub fn remove_enemy_at(&mut self, pos: &Position) -> Option<Enemy> {
        self.enemies.remove(pos)
    }

    pub fn get_item_at(&self, pos: &Position) -> Option<&Item> {
        self.items.get(pos)
    }

    pub fn remove_item_at(&mut self, pos: &Position) -> Option<Item> {
        self.items.remove(pos)
    }

    // More methods for field of view calculations would be added here
}
