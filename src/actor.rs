use bevy::{
    ecs::system::EntityCommands, prelude::*, utils::HashMap, transform::components::Transform, reflect::Reflect,
};
use bevy_entitiles::
    ldtk::{
        json::{field::FieldInstance, level::EntityInstance},
        resources::LdtkAssets,
};
use bevy_xpbd_2d::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_entitiles_derive::LdtkEntity;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use crate::util::*;

const PLAYER_SPEED: f32 = 100.;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
   fn build(&self, app: &mut App) {
       app.add_plugins((
                AsepritesheetPlugin::new(&["sprite.json"]).in_schedule(Update),
                YarnSpinnerPlugin::new(),
                ExampleYarnSpinnerDialogueViewPlugin::new(),
            ))
            .init_state::<DialougeState>()
            .add_systems(Startup, (
                setup,
            ))
            .add_systems(
                Update,
                (
                    player_control,
                    pick_up_items,
                    open_inventory,
                    extra_player_setup,
                    player_rotation,
                    attack_collisions,
                    follow,
                    enemy_add_sprites,
                    idle,
                    follow,
                    flee,
                    spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
                    enemy_hit,
                    death,
                ),
            )
            .add_event::<EnemyHit>();
    } 
}

fn spawn_dialogue_runner(
    mut commands: Commands, 
    project: Res<YarnProject>,
    state: Res<State<DialougeState>>,
) {
    // Create a dialogue runner from the project.
    // Immediately start showing the dialogue to the player
    match state.get() {
        DialougeState::Why => {
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node("WhyLevel0");
            commands.spawn(dialogue_runner);
        },
        DialougeState::Fear => {
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node("FearLevel0");
            commands.spawn(dialogue_runner);
        },
        DialougeState::Despair => {
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node("DespaiLevel0");
            commands.spawn(dialogue_runner);
        },
        DialougeState::Hello => {
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node("HelloLevel0");
            commands.spawn(dialogue_runner);
        },
        DialougeState::None => {},
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;

    let sheet_handle = load_spritesheet_then(
        &mut commands,
        &assets,
        "sprite.json",
        bevy::sprite::Anchor::Center,
        |sheet| {
            println!("Spritesheet finished loading!");
            format_anims_attack(sheet);
        },
    );

    let anim_attack = commands
        .spawn(AnimatedSpriteBundle {
            spritesheet: sheet_handle,
            ..Default::default()
        })
        .id();

    let sheet_handle = load_spritesheet_then(
        &mut commands,
        &assets,
        "character.json",
        bevy::sprite::Anchor::Center,
        |sheet| {
            println!("Spritesheet finished loading!");
            format_anims_player(sheet);
        },
    );

    let anim_player = commands
        .spawn((
            AnimatedSpriteBundle {
                spritesheet: sheet_handle,
                ..Default::default()
            },
            PlayerSprite,
        )).id();

    let attack_hitbox = commands
        .spawn((
            Collider::rectangle(15., 27.),
            Transform::from_xyz(8., 0., 0.),
            PlayerAttackBox,
            Sensor,
        ))
        .id();

    commands.entity(anim_player).add_child(anim_attack);
    commands.entity(anim_player).add_child(attack_hitbox);

    commands
        .spawn((
            camera,
            Collider::circle(5.0),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Friction {
                dynamic_coefficient: 0.5,
                static_coefficient: 0.5,
                ..Default::default()
            },
            Mass(100.),
            LinearDamping(10.),
            PlayerHitBox,
        ))
        .add_child(anim_player);
}

fn format_anims_attack(sheet: &mut Spritesheet) {
    let handle_attack = sheet.get_anim_handle("attack");

    if let Ok(attack) = sheet.get_anim_mut(&handle_attack) {
        attack.end_action = AnimEndAction::Stop;
    }
}

fn format_anims_player(sheet: &mut Spritesheet) {
    let handle_idle = sheet.get_anim_handle("idle");
    let handle_hit = sheet.get_anim_handle("hit");
    let _handle_walk = sheet.get_anim_handle("walk");

    if let Ok(anim) = sheet.get_anim_mut(&handle_idle) {
        anim.end_action = AnimEndAction::Pause;
    }

    if let Ok(anim) = sheet.get_anim_mut(&handle_hit) {
        anim.end_action = AnimEndAction::Next(handle_idle);
    }
}

fn extra_player_setup(
    mut commands: Commands,
    mut camera_q: Query<
        (Entity, &mut Transform),
        (With<Camera2d>, Without<PlayerMover>, Without<Player>),
    >,
    mut player_q: Query<(Entity, &mut Transform), With<Player>>, // TODO fix it does not have a
    // transform without the sprite
) {
    for (camera, mut camera_t) in camera_q.iter_mut() {
        let Ok((player, mut player_t)) = player_q.get_single_mut() else {
            return;
        };

        camera_t.translation = player_t.translation.clone();
        player_t.translation = Vec3::ZERO;

        commands.entity(camera).add_child(player);
        commands.entity(camera).insert(PlayerMover);
    }
}

fn enemy_add_sprites(
    mut commands: Commands,
    enemy_q: Query<Entity, (With<Enemy>, With<AddSprite>)>,
    assets: Res<AssetServer>
) {
    for enemy in enemy_q.iter() {
        let sheet_handle = load_spritesheet_then(
            &mut commands,
            &assets,
            "enemy.json",
            bevy::sprite::Anchor::Center,
            |sheet| {
                println!("Spritesheet finished loading!");
                format_anims_player(sheet);
            },
        );

        let anim = commands
            .spawn((
                AnimatedSpriteBundle {
                    spritesheet: sheet_handle,
                    ..Default::default()
                },
                EnemySprite,
            ))
            .id();

        commands.entity(enemy).add_child(anim);
        commands.entity(enemy).remove::<AddSprite>();
    }
}

fn attack_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut events: EventWriter<EnemyHit>,
    enemies_q: Query<Entity, With<EnemyHitBox>>,
    input: Res<ButtonInput<MouseButton>>,
    player_entity_q: Query<Entity, With<PlayerAttackBox>>,
) {
    if input.just_pressed(MouseButton::Left) {
        let Ok(player_e) = player_entity_q.get_single() else {
            return;
        };

        for Collision(contacts) in collision_event_reader.read() {
            for enemy_e in enemies_q.iter() {
                if contacts.entity1 == player_e && contacts.entity2 == enemy_e {
                    events.send(EnemyHit { enemy:enemy_e, player: player_e });
                }
            }
        }
    }
}

fn enemy_hit(
    mut enemies: Query<(&mut Enemy, &Children)>,
    mut players: Query<&mut Player>,
    mut anims: Query<&mut SpriteAnimator, With<EnemySprite>>,
    mut events: EventReader<EnemyHit>,
) {
    for event in events.read() {
        let (mut enemy, children) = enemies.get_mut(event.enemy).unwrap();
        let player = players.single_mut();

        enemy.hp -= player.sword_skill;
        enemy.fear += player.sword_skill * 10.;
        //player.sword_skill += 0.1;
        
        println!("PLAYER SWORD SKILL INCREASED: {}", player.sword_skill);

        for child in children {
            anims.get_mut(*child).unwrap().set_anim_index(1);
        }
    }
}

fn open_inventory(input: Res<ButtonInput<KeyCode>>, player_q: Query<&Player>) {
    if input.just_released(KeyCode::KeyI) {
        let Ok(player) = player_q.get_single() else {
            return;
        };
        println!("INVENTORY: {:?}", player.inventory.0);
    }
}

fn pick_up_items(
    input: Res<ButtonInput<KeyCode>>,
    items: Query<(Entity, &Transform, &Item)>,
    mut inventory_q: Query<&mut Player>,
    player_q: Query<&Transform, With<PlayerMover>>,
    mut commands: Commands,
) {
    if input.just_released(KeyCode::KeyE) {
        let Ok(transform) = player_q.get_single() else {
            return;
        };
        let Ok(mut player) = inventory_q.get_single_mut() else {
            return;
        };
        for (entity, position, item) in items.iter() {
            if position.translation.distance(transform.translation) < 20.0 {
                for _ in 0..item.count {
                    player.inventory.0.push(item.itype);
                }

                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_rotation(
    mut player_q: Query<&mut Transform, With<PlayerSprite>>,
    window_q: Query<&Window>,
) {
    let Ok(window) = window_q.get_single() else {
        return;
    };

    let pos = Vec2::new(window.width() / 2., window.height() / 2.);

    match window.cursor_position() {
        Some(cursor) => {
            let angle = (cursor - pos).angle_between(pos);

            let Ok(mut transform) = player_q.get_single_mut() else {
                return;
            };

            transform.rotation = Quat::from_rotation_z(angle - 0.5);
        }
        None => {}
    }
}

fn player_control(
    mut player_q: Query<&mut LinearVelocity, With<PlayerMover>>,
    mut anim_q: Query<&mut SpriteAnimator>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok(mut velocity) = player_q.get_single_mut() else {
        return;
    };
    // wasd is taken up by the camera controller.
    if input.pressed(KeyCode::KeyA) {
        velocity.x = -PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        velocity.x = PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyW) {
        velocity.y = PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyS) {
        velocity.y = -PLAYER_SPEED;
    }

    if buttons.just_pressed(MouseButton::Left) {
        for mut sprite_animator in anim_q.iter_mut() {
            if sprite_animator.cur_anim().is_none() {
                sprite_animator.set_anim_index(0);
            } else {
                sprite_animator.restart_anim();
            }
        }
    }

    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}







// Entities in the `Follow` task move toward the given entity at the given speed
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Follow {
    target: Entity,
    speed: f32,
}

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Idle;

// Entities in the `Follow` task move toward the given entity at the given speed
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Flee {
    target: Entity,
    speed: f32,
}

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Dead;

// Let's define some real behavior for entities in the follow task.
fn follow(
    mut transforms: Query<&mut Transform>,
    mut next_state: ResMut<NextState<DialougeState>>,
    mut anims: Query<&mut SpriteAnimator>,
    follows: Query<(Entity, &Follow, &Children), Without<Flee>>,
    time: Res<Time>,
) {
    for (entity, follow, children) in &follows {
        // Get the positions of the follower and target
        let mut target_translation = transforms.get(follow.target).unwrap().translation;
        target_translation.z = 0.;
        let follow_transform = &mut transforms.get_mut(entity).unwrap();
        let mut follow_translation = follow_transform.translation;
        follow_translation.z = 0.;

        // Find the direction from the follower to the target and go that way
        follow_transform.translation += (target_translation - follow_translation)
            .normalize_or_zero()
            * follow.speed
            * time.delta_seconds();
        
        for child in children {
            anims.get_mut(*child).unwrap().set_anim_index(2);
        }

        next_state.set(DialougeState::Hello);
    }
}

fn idle(
    mut anims: Query<&mut SpriteAnimator>,
    idles: Query<&Children, (With<Enemy>, With<Idle>)>
) {
    for children in &idles {
        for child in children {
            anims.get_mut(*child).unwrap().set_anim_index(0);
        }
    }
}

fn death(
    mut anims: Query<&mut SpriteAnimator>,
    idles: Query<&Children, (With<Enemy>, With<Dead>)>
) {
    for children in &idles {
        for child in children {
            anims.get_mut(*child).unwrap().set_anim_index(3);
        }
    }
}

// Let's define some real behavior for entities in the follow task.
fn flee(
    mut transforms: Query<&mut Transform>,
    mut anims: Query<&mut SpriteAnimator>,
    follows: Query<(Entity, &Flee, &Children)>,
    time: Res<Time>,
) {
    for (entity, follow, children) in &follows {
        // Get the positions of the follower and target
        let mut target_translation = transforms.get(follow.target).unwrap().translation;
        target_translation.z = 0.;
        let follow_transform = &mut transforms.get_mut(entity).unwrap();
        let mut follow_translation = follow_transform.translation;
        follow_translation.z = 0.;

        // Find the direction from the follower to the target and go that way
        follow_transform.translation -= (target_translation - follow_translation)
            .normalize_or_zero()
            * follow.speed
            * time.delta_seconds();
        
        for child in children {
            anims.get_mut(*child).unwrap().set_anim_index(2);
        }
    }
}




fn player_spawn(
    commands: &mut EntityCommands,
    entity_instance: &EntityInstance,
    _fields: &HashMap<String, FieldInstance>,
    _asset_server: &AssetServer,
    _ldtk_assets: &LdtkAssets,
) {
    let pos = Vec3::new(entity_instance.local_pos[0] as f32, -entity_instance.local_pos[1] as f32, 0.);

    commands.insert((
        Name::new("Player"),
        Transform::from_translation(pos),
    ));
}

fn enemy_spawn(
    commands: &mut EntityCommands,
    _entity_instance: &EntityInstance,
    _fields: &HashMap<String, FieldInstance>,
    _asset_server: &AssetServer,
    _ldtk_assets: &LdtkAssets,
) {
    commands.insert((
        EnemyHitBox,
        Collider::rectangle(10., 10.),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.5,
            ..Default::default()
        },
        Mass(100.),
        LinearDamping(10.),
        AddSprite,
        Name::new("Enemy"),
    ));
}

#[derive(Component, LdtkEntity, Default, Reflect)]
// this means the entity will be spawned with a sprite
#[spawn_sprite]
// this means the entity will not disappear when the level is unloaded
#[global_entity]
#[callback(player_spawn)]
pub struct Player {
    // this is a wrapper which will be generated
    // when you derive LdtkEnum for your custom enums.
    // There are also another two wrappers:
    // ItemTypeOption and Item TypeOptionVec

    // As impl a foreign trait for a foreign type is not allowed in rust,
    // we have to define these two wrappers.

    // You can impl the LdtkEntity trait yourself so these wrappers
    // can be avoided.
    pub inventory: ItemTypeVec,
    #[ldtk_name = "HP"]
    pub hp: f32,
    #[ldtk_name = "MP"]
    pub mp: f32,
    #[ldtk_name = "SwordSkill"]
    pub sword_skill: f32,
    #[ldtk_name = "RunSkill"]
    pub run_skill: f32,
}

#[derive(Component, LdtkEntity, Default, Reflect)]
#[spawn_sprite]
#[global_entity]
#[callback(enemy_spawn)]
pub struct Enemy {
    pub inventory: ItemTypeVec,
    #[ldtk_name = "HP"]
    pub hp: f32,
    pub attack: f32,
    pub fear: f32,
}

#[derive(Resource)]
struct IdleTimer(Timer);

#[derive(Component)]
struct AddSprite;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum DialougeState {
    #[default]
    None,
    Fear,
    Hello,
    Despair,
    Why,
}

#[derive(Event)]
struct EnemyHit {
    enemy: Entity,
    player: Entity,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum EnemyState {
    #[default]
    Idle,
    Follow,
    Flee,
    Dead,
}
