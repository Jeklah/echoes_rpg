//! Shared fog-of-war factory to eliminate duplication between UI backends
//!
//! This module provides a centralized way to create fog-of-war instances
//! with consistent configuration across both GUI and terminal interfaces.

use crate::world::fog_of_war::{FogColor, FogOfWar, FogOfWarConfig};

/// Creates a standardized fog-of-war instance with consistent configuration
/// for use across both GUI and terminal backends.
///
/// This eliminates the duplication of fog-of-war configuration and creation
/// logic that was previously scattered across gui.rs and ui/mod.rs.
pub fn create_standard_fog_of_war() -> FogOfWar {
    let config = FogOfWarConfig {
        hide_unexplored: true,
        show_explored_dimmed: true,
        dimming_factor: 0.5,
        unexplored_color: FogColor::BLACK,
    };
    FogOfWar::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_fog_of_war_creation() {
        let _fog = create_standard_fog_of_war();
        // Test that the fog instance is created successfully
        // This is mainly to ensure the factory function doesn't panic
        assert!(true); // Placeholder - actual fog testing would depend on FogOfWar's public interface
    }
}
