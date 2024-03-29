#![allow(clippy::type_complexity)]

use bevy::{
    app::App
    ,
    prelude::*,
};

use crate::actions::ActionsPlugin;
// use crate::menu::MenuPlugin;

mod actions;

// use bevy::core::update_frame_count;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default()).add_plugins((
            // MenuPlugin,
            ActionsPlugin,
        ))
            .add_systems(Startup, setup);

        #[cfg(debug_assertions)]
        {
            // app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
