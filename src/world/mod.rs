pub mod enemy;
pub mod fog_factory;
pub mod fog_of_war;
pub mod level;
pub mod tile;

// Re-exports
pub use enemy::Enemy;
pub use fog_factory::create_standard_fog_of_war;
pub use fog_of_war::FogOfWar;
pub use level::{Level, Position};
pub use tile::{Tile, TileType};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonType {
    Ruins,
    Forest,
    Mountain,
    Cavern,
}

impl DungeonType {
    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            DungeonType::Ruins => {
                "Ancient ruins filled with crumbling architecture and forgotten treasures."
            }
            DungeonType::Forest => {
                "A dense forest with overgrown ancient structures hidden among the trees."
            }
            DungeonType::Mountain => {
                "Treacherous mountain paths leading to hidden temples and lookout points."
            }
            DungeonType::Cavern => {
                "Dark underground caves with magical crystals providing eerie illumination."
            }
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => DungeonType::Ruins,
            1 => DungeonType::Forest,
            2 => DungeonType::Mountain,
            _ => DungeonType::Cavern,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub name: String,
    pub dungeon_type: DungeonType,
    pub levels: Vec<Level>,
    pub current_level: usize,
    pub difficulty: u32,
}

impl Dungeon {
    pub fn new(
        name: String,
        dungeon_type: DungeonType,
        difficulty: u32,
        num_levels: usize,
    ) -> Self {
        let mut levels = Vec::new();

        for i in 0..num_levels {
            let is_final = i == num_levels - 1;
            levels.push(Level::generate(
                difficulty,
                i as u32 + 1,
                dungeon_type,
                is_final,
            ));
        }

        Dungeon {
            name,
            dungeon_type,
            levels,
            current_level: 0,
            difficulty,
        }
    }

    pub fn generate_random(player_level: u32) -> Self {
        let mut rng = rand::thread_rng();

        let dungeon_type = DungeonType::random();
        let difficulty = player_level.max(1);

        // Generate a thematic name
        let prefix = match rng.gen_range(0..4) {
            0 => "Forgotten",
            1 => "Ancient",
            2 => "Mysterious",
            _ => "Haunted",
        };

        let location = match dungeon_type {
            DungeonType::Ruins => match rng.gen_range(0..3) {
                0 => "Temple",
                1 => "Citadel",
                _ => "Palace",
            },
            DungeonType::Forest => match rng.gen_range(0..3) {
                0 => "Grove",
                1 => "Thicket",
                _ => "Woods",
            },
            DungeonType::Mountain => match rng.gen_range(0..3) {
                0 => "Peaks",
                1 => "Summit",
                _ => "Cliffs",
            },
            DungeonType::Cavern => match rng.gen_range(0..3) {
                0 => "Caverns",
                1 => "Depths",
                _ => "Grotto",
            },
        };

        let name = format!("{} {}", prefix, location);

        // Number of levels increases with difficulty
        let num_levels = 3 + (difficulty / 5).min(5) as usize;

        Dungeon::new(name, dungeon_type, difficulty, num_levels)
    }

    pub fn current_level(&self) -> &Level {
        &self.levels[self.current_level]
    }

    pub fn current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.current_level]
    }

    pub fn go_to_next_level(&mut self) -> Result<(), String> {
        if self.current_level + 1 >= self.levels.len() {
            return Err("You are already at the final level".to_string());
        }

        self.current_level += 1;
        Ok(())
    }

    pub fn is_final_level(&self) -> bool {
        self.current_level == self.levels.len() - 1
    }
}
