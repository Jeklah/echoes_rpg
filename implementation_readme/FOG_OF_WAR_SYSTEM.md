# Fog of War System Documentation

## Overview

The Echoes RPG implements a comprehensive fog of war system that creates an immersive exploration experience by limiting the player's visibility to areas they have explored and are currently within their line of sight.

## System Components

### 1. Tile-Level Visibility (`src/world/tile.rs`)

Each tile in the game world has two visibility states:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub explored: bool,    // Has the player ever seen this tile?
    pub visible: bool,     // Can the player currently see this tile?
}
```

#### Tile States:
- **Unexplored** (`explored: false`): Areas the player has never visited - rendered as empty space
- **Explored but not visible** (`explored: true, visible: false`): Areas the player has seen before but are not currently in view - rendered in muted colors
- **Currently visible** (`explored: true, visible: true`): Areas within the player's current line of sight - rendered in full color

### 2. Level-Wide Visibility Arrays (`src/world/level.rs`)

The Level structure maintains two 2D boolean arrays for efficient visibility checking:

```rust
pub struct Level {
    // ... other fields
    pub revealed_tiles: Vec<Vec<bool>>,  // Tiles that have been explored
    pub visible_tiles: Vec<Vec<bool>>,   // Tiles currently visible
}
```

### 3. Visibility Update System (`src/game/mod.rs`)

The `update_visibility()` method runs every game frame and implements the core fog of war logic:

```rust
pub fn update_visibility(&mut self) {
    let level = self.current_level_mut();
    let player_pos = level.player_position;

    // Reset all tiles to not visible
    for row in &mut level.visible_tiles {
        for tile in row {
            *tile = false;
        }
    }

    // Reveal circular area around player
    let view_radius = 10;
    for dy in -view_radius..=view_radius {
        for dx in -view_radius..=view_radius {
            let x = player_pos.x + dx;
            let y = player_pos.y + dy;
            
            if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
                if dx * dx + dy * dy <= view_radius * view_radius {
                    // Mark as both visible and explored
                    level.visible_tiles[y as usize][x as usize] = true;
                    level.revealed_tiles[y as usize][x as usize] = true;
                    
                    if let Some(tile) = level.get_tile_mut(x, y) {
                        tile.explored = true;
                        tile.visible = true;
                    }
                }
            }
        }
    }
}
```

#### Key Features:
- **Circular Vision**: Player sees in a realistic circular pattern, not a square
- **Persistent Exploration**: Once explored, areas remain in the `revealed_tiles` array
- **Dynamic Visibility**: Only areas within the current view radius are marked as visible

### 4. Rendering System (`src/ui/mod.rs`)

The UI rendering respects fog of war by checking tile visibility before drawing entities:

```rust
let char_to_draw = if pos == level.player_position {
    '@'
} else if !tile.explored {
    ' '  // Unexplored areas show as space
} else if tile.visible && level.enemies.contains_key(&pos) {
    'E'  // Only show enemies if tile is currently visible
} else if tile.visible && level.items.contains_key(&pos) {
    '!'  // Only show items if tile is currently visible
} else {
    tile.render()  // Use tile's own fog of war rendering
};
```

#### Rendering Rules:
1. **Player**: Always visible (yellow `@`)
2. **Unexplored areas**: Empty space (black)
3. **Enemies**: Only visible if in current line of sight (red `E`)
4. **Items**: Only visible if in current line of sight (green `!`)
5. **Terrain**: Uses tile's `render()` method for fog of war coloring

### 5. Color-Based Fog of War

Different visibility states use different colors to provide visual feedback:

```rust
let color = match char_to_draw {
    '#' => {
        if tile.visible {
            Color::White        // Bright walls in view
        } else {
            Color::DarkGrey     // Dim walls in memory
        }
    }
    '.' => {
        if tile.visible {
            Color::DarkGrey     // Visible floor
        } else {
            Color::Black        // Remembered floor
        }
    }
    // ... other tile types follow similar pattern
};
```

## Implementation Details

### Initialization

Fog of war is initialized when creating a new game:

```rust
impl Game {
    pub fn new(player: Player) -> Self {
        let mut game = Game {
            // ... initialization
        };
        
        // Initialize visibility for starting level
        game.update_visibility();
        game
    }
}
```

### Performance Optimizations

1. **Efficient Updates**: Only recalculates visibility each frame, doesn't store complex state
2. **Bounded Calculations**: Vision radius limits the area that needs to be checked
3. **Simple Math**: Uses basic distance calculations for circular vision
4. **Memory Efficient**: Boolean arrays are compact and cache-friendly

### Integration Points

The fog of war system integrates with several game systems:

- **Movement**: Updates visibility when player moves
- **Level Changes**: Automatically resets and recalculates for new levels
- **Save/Load**: Visibility state is preserved through serialization
- **Combat**: Enemies only appear when visible, affecting tactical decisions

## Troubleshooting

### Common Issues and Solutions

#### Problem: All areas visible / No fog of war
**Cause**: Visibility update not being called or rendering bypassing checks
**Solution**: Ensure `update_visibility()` is called in main game loop and UI checks tile visibility

#### Problem: Areas remain unexplored after visiting
**Cause**: `explored` flag not being set properly
**Solution**: Verify that `tile.explored = true` is set in `update_visibility()`

#### Problem: Enemies/items visible through walls
**Cause**: Entity rendering not checking tile visibility
**Solution**: Ensure entity rendering checks `tile.visible` before drawing

#### Problem: Performance issues with large maps
**Cause**: Inefficient visibility calculations
**Solution**: Consider implementing line-of-sight algorithms for larger view distances

## Configuration

### Adjustable Parameters

- **View Radius**: `view_radius = 10` in `update_visibility()` method
- **Screen Radius**: Extended area for smooth scrolling (currently 30x10)
- **Color Schemes**: Easily modified in the color matching section

### Future Enhancements

Potential improvements to the fog of war system:

1. **Line-of-Sight**: Implement raycasting for realistic vision blocking
2. **Light Sources**: Dynamic lighting from torches, spells, etc.
3. **Weather Effects**: Fog, rain reducing visibility
4. **Stealth System**: Different visibility rules for sneaking
5. **Minimap**: Show explored areas in a compact view

## Technical Notes

### Thread Safety
The fog of war system is single-threaded and doesn't require synchronization.

### Memory Usage
Visibility arrays scale with level size: `width × height × 2 bytes` for boolean arrays.

### Serialization
All fog of war state (`explored` and `visible` flags) is preserved in save files through Serde serialization.

This fog of war implementation provides a solid foundation for exploration-based gameplay while maintaining good performance and visual clarity.