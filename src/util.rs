use bevy::prelude::*;
use bevy_entitiles_derive::{LdtkEntity, LdtkEntityTag, LdtkEnum};


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    LoadingScreen,
    MainMenu,
    InGame,
    Paused,
}

#[derive(Component)]
pub struct PlayerMover;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct EnemySprite;

#[derive(LdtkEnum, Reflect, Clone, Copy, Debug)]
#[wrapper_derive(Reflect, Default)]
pub enum ItemType {
    Key,
    Coins,
    Scroll,
    Bandage,
    Sword,
    Soul,
}

#[derive(LdtkEnum, Reflect, Clone, Copy, Debug)]
#[wrapper_derive(Reflect, Default)]
pub enum ChestState {
    Open,
    Closed,
}

#[derive(LdtkEnum, Reflect, Clone, Copy, Debug)]
#[wrapper_derive(Reflect, Default)]
pub enum DoorState {
    Open,
    Closed,
}

#[derive(Component, LdtkEntity, Reflect)]
#[spawn_sprite]
pub struct Item {
    #[ldtk_name = "type"]
    pub itype: ItemType,
    pub count: i32,
}

#[derive(Component, LdtkEntity, Reflect)]
#[spawn_sprite]
pub struct Chest {
    #[ldtk_name = "state"]
    pub state: ChestState,
}

#[derive(Component, LdtkEntity, Reflect)]
#[spawn_sprite]
pub struct Door {
    #[ldtk_name = "state"]
    pub state: DoorState,
}

#[derive(Component, LdtkEntityTag)]
pub struct Actor;

#[derive(Component, LdtkEntityTag)]
pub struct Loot;

#[derive(Component, LdtkEntityTag)]
pub struct Object;

#[derive(Component)]
pub struct PlayerAttackBox;

#[derive(Component)]
pub struct PlayerHitBox;

#[derive(Component)]
pub struct EnemyAttackBox;

#[derive(Component)]
pub struct EnemyHitBox;
