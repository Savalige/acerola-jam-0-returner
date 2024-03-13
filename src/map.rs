use bevy::{
    prelude::*, render::render_resource::FilterMode, utils::HashMap
};
use bevy_xpbd_2d::prelude::*;
use bevy_entitiles::{
    ldtk::{
        app_ext::LdtkApp,
        events::LdtkEvent,
        layer::physics::LdtkPhysicsLayer,
        resources::{LdtkAdditionalLayers, LdtkAssets, LdtkLevelManager, LdtkLoadConfig},
        sprite::LdtkEntityMaterial,
    },
    tilemap::physics::PhysicsTile,
    EntiTilesPlugin,
};
use crate::util::*;
use crate::actor::{Enemy, Player};


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
                EntiTilesPlugin,
            )
            .add_systems(
                Update,
                (
                    events,
                    hot_reload,
                    load.run_if(resource_added::<LdtkLevelManager>),
                ),
            )
            .register_type::<Player>()
            .register_type::<Enemy>()
            .register_type::<Item>()
            .register_type::<Chest>()
            .register_type::<Door>()
            .insert_resource(Msaa::Off)
            .insert_resource(Gravity(Vec2::new(0., 0.)))
            .insert_resource(LdtkLoadConfig {
                file_path: "assets/ldtk/test.ldtk".to_string(),
                asset_path_prefix: "ldtk/".to_string(),
                filter_mode: FilterMode::Nearest,
                ignore_unregistered_entities: true,
                ..Default::default()
            })
            .insert_resource(LdtkAdditionalLayers {
                physics_layer: Some(LdtkPhysicsLayer {
                    identifier: "PhysicsColliders".to_string(),
                    air: 0,
                    parent: "Collisions".to_string(),
                    tiles: Some(HashMap::from([(
                        1,
                        PhysicsTile {
                            rigid_body: true,
                            friction: Some(0.9),
                        },
                    )])),
                }),
                ..Default::default()
            })
            .insert_gizmo_group(PhysicsGizmos::all(), GizmoConfig::default())
            .register_ldtk_entity::<Item>("Item")
            .register_ldtk_entity::<Chest>("Chest")
            .register_ldtk_entity::<Door>("Door")
            .register_ldtk_entity::<Player>("Player")
            .register_ldtk_entity::<Enemy>("Enemy")
            .register_ldtk_entity_tag::<Actor>("actor")
            .register_ldtk_entity_tag::<Loot>("loot")
            .register_ldtk_entity_tag::<Object>("object");
   } 
}



pub fn load(
    mut commands: Commands,
    mut manager: ResMut<LdtkLevelManager>,
) {
    manager.load(&mut commands, "Start".to_string(), None);
}

pub fn hot_reload(
    input: Res<ButtonInput<KeyCode>>,
    mut manager: ResMut<LdtkLevelManager>,
    config: Res<LdtkLoadConfig>,
    mut assets: ResMut<LdtkAssets>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut entity_material_assets: ResMut<Assets<LdtkEntityMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        manager.reload_json(&config);
        assets.initialize(
            &config,
            &manager,
            &asset_server,
            &mut atlas_layouts,
            &mut entity_material_assets,
            &mut mesh_assets,
        );
        println!("Hot reloaded!")
    }
}

pub fn events(mut ldtk_events: EventReader<LdtkEvent>) {
    for event in ldtk_events.read() {
        match event {
            LdtkEvent::LevelLoaded(level) => {
                println!("Level loaded: {}", level.identifier);
            }
            LdtkEvent::LevelUnloaded(level) => {
                println!("Level unloaded: {}", level.identifier);
            }
        }
    }
}

