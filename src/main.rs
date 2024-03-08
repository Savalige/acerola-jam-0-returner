use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_xpbd_2d::prelude::*;
use actor::ActorPlugin;
use map::MapPlugin;

mod actor;
mod map;
mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            ScreenDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
            ScreenFrameDiagnosticsPlugin,
            ActorPlugin,
            MapPlugin,
        ))
        .run();
}
