use bevy::prelude::{Bundle, SpriteBundle};

use game_types::{
    components::{
        grid_components::definitions::CellIndex,
        movement_components::{MoveTag, SurfaceCoordinate},
    }
};

#[derive(Bundle, Clone, Default)]
pub struct SurfaceWalkerBundle {
    pub surface_coordinate: SurfaceCoordinate,
    pub occupied_cell_index: CellIndex,
    pub sprite_bundle: SpriteBundle,
    pub move_tag: MoveTag,
}