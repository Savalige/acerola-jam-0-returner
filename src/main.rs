use bevy::{
    prelude::*, render::{
        render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin,
    }
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_xpbd_2d::prelude::*;
use actor::ActorPlugin;
use map::MapPlugin;
use util::GameState;

mod actor;
mod map;
mod util;
mod menu;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            ScreenDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
            ScreenFrameDiagnosticsPlugin,
            ActorPlugin,
            MapPlugin,
        ))
        .init_state::<GameState>()
        .run();
}
