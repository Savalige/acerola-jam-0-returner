use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use actor::ActorPlugin;
use map::MapPlugin;

mod actor;
mod map;
mod util;
mod menu;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            ActorPlugin,
            MapPlugin,
        ))
        .run();
}
