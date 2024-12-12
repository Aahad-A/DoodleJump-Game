mod map;

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_rapier2d::prelude::*;
use map::spawn_map;
use map::modify_body_translation;

// Constants
pub const PLAYER_SIZE: f32 = 64.0; // player size
pub const PLAYER_SPEED: f32 = 500.0;
pub const TILE_SIZE: f32 = 0.1;
pub const FALL_SPEED: f32 = 1080.0;
pub const JUMP_FORCE: f32 = 500.0;
const ANIMATION_FRAME_TIME: f32 = 0.1;
const BACKGROUND_COLOR: Color = Color::rgb(0.5, 2.5, 5.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mega Jump Game".into(),
                // resolution: WindowPosition::Windowed { width: 800.0, height: 600.0 },
                present_mode: PresentMode::AutoVsync, // VSync with frame limiting
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -FALL_SPEED,
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 60.0,
                substeps: 1,
            },
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_map)
        .add_system(modify_body_translation)
        .add_system(camera_follow.after(move_player))
        .add_system(move_player)
        .add_system(confine_player_movement)
        .add_system(apply_jump_force)
        // Uncomment below to see physics colliders (for debug use)
        //.add_plugin(RapierDebugRenderPlugin::default())
        // Uncomment below to recreate the bug with music
        //.add_system(music_setup)
        .run();
}

pub fn hello_world() {
    println!("hello world")
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

// Update Player struct
#[derive(Component, Default)]
pub struct Player {
    state: PlayerState,
    animation_frame: bool, // alternates between true/false for walk1/walk2
}

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Left,
    Right,
    Jump,
}


// SPAWN PLAYER WITH SPRITE AND COLLIDER
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands
        .spawn(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(20.0, 49.0))
        .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 10.0,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.0))
        .insert(KinematicCharacterController::default())
        .insert(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 1.0 - 50.0, 0.0),
            texture: asset_server.load("sprites/Adventurer/Poses/adventurer_cheer1.png"),
            ..default()
        })
        .insert(Player::default())
        .insert(AnimationTimer(Timer::from_seconds(ANIMATION_FRAME_TIME, TimerMode::Repeating)));
}

#[derive(Component)]
pub struct BackGround {
    pos: Vec3,
}

// ADD TEXTURE OF BACKGROUND TO THE GAME WITH CLOUDS COVERING THE PLATFORM AND PLAYER FOR ADDED DIFFICULTY
pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
        ..default()
    });

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -5.0),
            texture: asset_server.load("backgrounds/BG_DesertMountains/background2.png"),
            ..default()
        })
        .insert(BackGround { pos: Vec3::ZERO });

    // Spawn clouds in a loop
    for i in (0..12000).step_by(500) {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0 + if i % 1000 == 0 { 20.0 } else { -20.0 }, window.height() / 2.0 + i as f32, 0.0),
            texture: asset_server.load("clouds.png"),
            ..default()
        });
    }
}

// FOLLOWS THE PLAYER CHARACTER WITH A TRANSLATION OF 150 PIXELS SO PLAYER IS ON BOTTOM OF SCREEN AND YOU CAN SEE PLATFORMS ABOVE
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y + 150.0;
}


#[derive(Component)]
pub struct Jump(pub f32);

pub fn move_player(
    mut controllers: Query<&mut KinematicCharacterController>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Player, &mut Handle<Image>, &mut AnimationTimer), With<Player>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut to_move: Vec2 = Vec2::ZERO;
    
    if let Ok((entity, mut player, mut sprite, mut timer)) = player_query.get_single_mut() {
        timer.0.tick(time.delta());

        // Handle movement and sprite changes
        if keyboard.pressed(KeyCode::Left) {
            to_move.x -= PLAYER_SPEED;
            if timer.0.just_finished() {
                player.animation_frame = !player.animation_frame;
                *sprite = asset_server.load(if player.animation_frame {
                    "sprites/Adventurer/Poses/adventurer_walk1.png"
                } else {
                    "sprites/Adventurer/Poses/adventurer_walk2.png"
                });
            }
            player.state = PlayerState::Left;
        } else if keyboard.pressed(KeyCode::Right) {
            to_move.x += PLAYER_SPEED;
            if timer.0.just_finished() {
                player.animation_frame = !player.animation_frame;
                *sprite = asset_server.load(if player.animation_frame {
                    "sprites/Adventurer/Poses/adventurer_walk1.png"
                } else {
                    "sprites/Adventurer/Poses/adventurer_walk2.png"
                });
            }
            player.state = PlayerState::Right;
        } else {
            *sprite = asset_server.load("sprites/Adventurer/Poses/adventurer_idle.png");
            player.state = PlayerState::Idle;
        }
        // Handle jumping
        if keyboard.just_pressed(KeyCode::Space) {
            commands.entity(entity).insert(Jump(JUMP_FORCE));
            *sprite = asset_server.load("sprites/Adventurer/Poses/adventurer_climb1.png");
            player.state = PlayerState::Jump;
        }
    }

    for mut controller in controllers.iter_mut() {
        controller.translation = Some(to_move * time.delta_seconds());
    }
}

// Update apply_jump_force system
pub fn apply_jump_force(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut Jump, &mut Handle<Image>), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((entity, mut transform, mut jump, mut sprite)) = player_query.get_single_mut() {
        let jump_power = (time.delta_seconds() * FALL_SPEED * 2.0).min(jump.0);
        jump.0 -= jump_power;
        transform.translation.y += jump_power;
        
        if jump.0 <= 0.0 {
            commands.entity(entity).remove::<Jump>();
            *sprite = asset_server.load("sprites/Adventurer/Poses/adventurer_idle.png");
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0 - 50.0; // 32.0
        let x_min = 25.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 50.0 + half_player_size;
        let y_max = window.height() + 100000.0;

        let mut translation: Vec3 = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn music_setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("Child's Nightmare.ogg");
    audio.play(music);
}