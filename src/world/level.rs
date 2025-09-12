use crate::item::Item;
use crate::world::{DungeonType, Enemy, Tile, TileType};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use web_sys;

// WASM-optimized map sizes to prevent freezing
#[cfg(target_arch = "wasm32")]
const MAP_WIDTH: usize = 40;
#[cfg(target_arch = "wasm32")]
const MAP_HEIGHT: usize = 30;

#[cfg(not(target_arch = "wasm32"))]
const MAP_WIDTH: usize = 80;
#[cfg(not(target_arch = "wasm32"))]
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

        // Generate rooms with WASM-specific limits and early termination
        let (max_rooms, max_attempts) = if cfg!(target_arch = "wasm32") {
            // Very conservative limits for WASM
            (2.min(difficulty / 2 + 1) as i32, 10) // Maximum 2-3 rooms, only 10 attempts
        } else {
            (10 + (difficulty / 2).min(15) as i32, 100)
        };

        let min_size = if cfg!(target_arch = "wasm32") { 5 } else { 5 };
        let max_size = if cfg!(target_arch = "wasm32") { 6 } else { 12 };

        let mut rng = rand::thread_rng();
        let mut rooms_created = 0;
        let mut attempts = 0;

        // WASM: Start with a guaranteed room to ensure we have at least one
        if cfg!(target_arch = "wasm32") {
            let starter_room = Room::new(3, 3, 8, 8);
            level.create_room(&starter_room);
            level.rooms.push(starter_room);
            rooms_created = 1;
        }

        while rooms_created < max_rooms && attempts < max_attempts {
            attempts += 1;
            let w = rng.gen_range(min_size..=max_size);
            let h = rng.gen_range(min_size..=max_size);
            let x = rng.gen_range(1..(MAP_WIDTH as i32 - w - 1));
            let y = rng.gen_range(1..(MAP_HEIGHT as i32 - h - 1));

            let new_room = Room::new(x, y, w, h);

            // WASM: Skip overlap checking for faster generation
            let overlap = if cfg!(target_arch = "wasm32") {
                false // Accept overlaps to speed up generation
            } else {
                let mut has_overlap = false;
                for other_room in &level.rooms {
                    if new_room.intersects(other_room) {
                        has_overlap = true;
                        break;
                    }
                }
                has_overlap
            };

            if !overlap {
                // Create room
                level.create_room(&new_room);

                // Connect to previous room (simplified for WASM)
                if level.rooms.len() > 0 {
                    let (new_x, new_y) = {
                        let center = new_room.center();
                        (center.x, center.y)
                    };

                    let (prev_x, prev_y) = {
                        let center = level.rooms[level.rooms.len() - 1].center();
                        (center.x, center.y)
                    };

                    // WASM: Always use horizontal first to simplify
                    if cfg!(target_arch = "wasm32") {
                        level.create_horizontal_tunnel(prev_x, new_x, prev_y);
                        level.create_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        // Original random logic for desktop
                        if rng.gen_bool(0.5) {
                            level.create_horizontal_tunnel(prev_x, new_x, prev_y);
                            level.create_vertical_tunnel(prev_y, new_y, new_x);
                        } else {
                            level.create_vertical_tunnel(prev_y, new_y, prev_x);
                            level.create_horizontal_tunnel(prev_x, new_x, new_y);
                        }
                    }
                }

                // WASM: Skip door placement for speed
                if !cfg!(target_arch = "wasm32") {
                    level.place_doors(&new_room);
                }

                // Store the room
                level.rooms.push(new_room);
                rooms_created += 1;
            }
        }

        // Ensure we have at least one room (more aggressive for WASM)
        if level.rooms.is_empty() {
            let fallback_room = if cfg!(target_arch = "wasm32") {
                Room::new(2, 2, 12, 8) // Larger fallback room for WASM
            } else {
                Room::new(5, 5, 6, 6)
            };
            level.create_room(&fallback_room);
            level.rooms.push(fallback_room);
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

        // WASM: Skip enemy and item placement during initialization to speed up
        if cfg!(target_arch = "wasm32") {
            // Place minimal entities for WASM
            if level.rooms.len() > 1 {
                // Place one enemy in the last room only
                let last_room = &level.rooms[level.rooms.len() - 1];
                let enemy_pos = last_room.center();
                let enemy = Enemy::generate_random(level_num, 1); // Minimal difficulty
                level.enemies.insert(enemy_pos, enemy);

                // Place one chest
                if level.rooms.len() > 1 {
                    let mut chest_pos = level.rooms[1].center();
                    chest_pos.x += 1; // Offset from center
                    level.tiles[chest_pos.y as usize][chest_pos.x as usize] = Tile::chest();
                    let item = Item::generate_for_chest(level_num);
                    level.items.insert(chest_pos, item);
                }
            }
        } else {
            // Full generation for desktop
            level.place_enemies(difficulty);
            level.place_items(difficulty);
        }

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
            let max_enemies = if cfg!(target_arch = "wasm32") { 2 } else { 5 };
            let num_enemies =
                ((room_area as f32 * 0.01 * difficulty as f32).round() as u32).min(max_enemies);

            let mut enemies_placed = 0;
            let max_attempts = 20; // Prevent infinite attempts

            for _ in 0..num_enemies {
                let mut attempts = 0;
                let mut enemy_placed = false;

                while attempts < max_attempts && !enemy_placed {
                    let x = rng.gen_range((room.x1 + 1)..room.x2);
                    let y = rng.gen_range((room.y1 + 1)..room.y2);
                    let pos = Position::new(x, y);

                    // Don't place enemies on stairs, other enemies, or player position
                    if (Some(pos) != self.stairs_down)
                        && (Some(pos) != self.stairs_up)
                        && (Some(pos) != self.stairs_up)
                        && (!self.enemies.contains_key(&pos))
                        && (pos != self.player_position)
                    {
                        // Generate enemy based on difficulty and level number
                        let enemy = Enemy::generate_random(self.level_num, difficulty);
                        self.enemies.insert(pos, enemy);
                        enemy_placed = true;
                        enemies_placed += 1;
                    }
                    attempts += 1;
                }

                // If we couldn't place an enemy after max attempts, skip remaining enemies for this room
                if !enemy_placed {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(
                        &format!(
                            "Warning: Could not place enemy {} in room {}",
                            enemies_placed + 1,
                            i
                        )
                        .into(),
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    eprintln!(
                        "Warning: Could not place enemy {} in room {}",
                        enemies_placed + 1,
                        i
                    );
                    break;
                }
            }
        }
    }

    fn place_items(&mut self, _difficulty: u32) {
        let mut rng = rand::thread_rng();

        // Place chests and items in random rooms (but not the first)
        for i in 1..self.rooms.len() {
            let room = &self.rooms[i];

            // 50% chance of chest (increased from 30% to ensure more chests spawn for testing)
            // This makes it easier to verify the fix works
            if rng.gen_bool(0.5) {
                // Find a spot for the chest with bounded attempts to prevent infinite loops
                let mut chest_placed = false;
                let max_attempts = 20; // Prevent infinite loops

                for _ in 0..max_attempts {
                    let chest_x = rng.gen_range((room.x1 + 1)..room.x2);
                    let chest_y = rng.gen_range((room.y1 + 1)..room.y2);
                    let chest_pos = Position::new(chest_x, chest_y);

                    // Make sure we're not placing on top of stairs, enemies, or player
                    if (Some(chest_pos) != self.stairs_down)
                        && (Some(chest_pos) != self.stairs_up)
                        && (!self.enemies.contains_key(&chest_pos))
                        && (chest_pos != self.player_position)
                    {
                        // Place chest
                        self.tiles[chest_y as usize][chest_x as usize] = Tile::chest();

                        // Generate a guaranteed quality item specifically for chests
                        let item = Item::generate_for_chest(self.level_num);
                        self.items.insert(chest_pos, item);

                        // Debug validation - confirm item was added at this position
                        #[cfg(debug_assertions)]
                        assert!(
                            self.items.contains_key(&chest_pos),
                            "Failed to insert item at chest position: {chest_pos:?}"
                        );

                        chest_placed = true;
                        break;
                    }
                }

                // If we couldn't place a chest after max attempts, skip this room
                if !chest_placed {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(
                        &format!("Warning: Could not place chest in room {}", i).into(),
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    eprintln!("Warning: Could not place chest in room {}", i);
                }
            }

            // Maybe place some loose items too (20% chance)
            if rng.gen_bool(0.2) {
                let mut item_placed = false;
                let max_attempts = 15; // Prevent infinite attempts for loose items

                for _ in 0..max_attempts {
                    let x = rng.gen_range((room.x1 + 1)..room.x2);
                    let y = rng.gen_range((room.y1 + 1)..room.y2);
                    let pos = Position::new(x, y);

                    // Don't place on stairs, enemies, chests, or player
                    if (Some(pos) != self.stairs_down)
                        && (Some(pos) != self.stairs_up)
                        && (!self.enemies.contains_key(&pos))
                        && (self.tiles[y as usize][x as usize].tile_type != TileType::Chest)
                        && (pos != self.player_position)
                        && (!self.items.contains_key(&pos))
                    {
                        let item = Item::generate_random(self.level_num);
                        self.items.insert(pos, item);
                        item_placed = true;
                        break;
                    }
                }

                // Log if we couldn't place a loose item (less critical than chests/enemies)
                if !item_placed {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(
                        &format!("Info: Could not place loose item in room {}", i).into(),
                    );
                }
            }
        }
    }

    pub fn is_position_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
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
        let item = self.items.get(pos);

        // Additional debug validation for chest items
        if cfg!(debug_assertions) {
            if let Some(tile) = self.get_tile(pos.x, pos.y) {
                if tile.tile_type == TileType::Chest && item.is_none() {
                    // This would indicate a bug - chest exists but has no item
                    eprintln!("WARNING: Found chest at {pos:?} but no item associated with it");
                }
            }
        }

        item
    }

    pub fn remove_item_at(&mut self, pos: &Position) -> Option<Item> {
        self.items.remove(pos)
    }

    // More methods for field of view calculations would be added here
}
