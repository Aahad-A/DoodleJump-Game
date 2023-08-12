use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_rapier2d::prelude::*;

// used :
// https://www.pngegg.com/en/png-fztwu/download
// https://patrickdearteaga.com/royalty-free-music/childs-nightmare/
// https://ohnoponogames.itch.io/retro-cloud-tileset
// https://www.freepik.com/free-vector/flat-design-pixel-art-cloud-illustration_38680485.htm#query=pixel%20art%20cloud&position=0&from_view=keyword&track=ais

pub const PLAYER_SIZE: f32 = 64.0; //player size
pub const PLAYER_SPEED: f32 = 360.0;
pub const TILE_SIZE: f32 = 0.1;
pub const FALL_SPEED: f32 = 1080.0;
pub const JUMP_FORCE: f32 = 500.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 2.5, 5.0);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mega Jump Game".into(),
                resolution: (600., 700.).into(),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -FALL_SPEED,
            ..default()
        })

        // UNCOMMENT below to see physics colliders (for debug use)
        //.add_plugin(RapierDebugRenderPlugin::default())

        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_map)
        .add_system(modify_body_translation)
        .add_system(camera_follow.after(move_player))
        .add_system(move_player)
        .add_system(confine_player_movement)

        // For some reason, when music is being played, it lags the entire physics engine and the music starts stuttering
        // UNCOMMENT below to recreate the bug
        //.add_system(music_setup)
        
        
        .run();
}

pub fn hello_world() {
    println!("hello world")
}

#[derive(Component)]
pub struct Player {
    
}

// SPAWN PLAYER WITH SPRITE AND COLLIDER
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap(); //result type

    //rigid-bodies are only responsible for the dynamics and kinematics of the solid.

    //Colliders represent the geometric shapes that generate contacts and collision
    commands
        .spawn(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(20.0, 49.0))
        .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 10.0
        })
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.0))
        .insert(KinematicCharacterController::default())
        .insert(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 1.0 - 50.0, 0.0),
            texture: asset_server.load("sprites/Adventurer/Poses/adventurer_cheer1.png"),
            ..default()
        })
        .insert(Player {});
}

#[derive(Component)]
pub struct MovingPlatform {
    direction: i32,
    max_x: f32,
    min_x: f32,
}

// ATTEMPT AT TRYING TO MOVE BACKGROUND WTIH PLAYER (unsuccessful)
#[derive(Component)]
pub struct BackGround {
    pos: Vec3,
}

// ADD TEXTURE OF BACKGROUND TO THE GAME WITH CLOUDS COVERING THE PLATFORM AND PLAYER FOR ADDED DIFFICULTY
pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -5.0),
        texture: asset_server.load("backgrounds/BG_DesertMountains/background2.png"),
        ..default()
    })
    .insert(BackGround {
        pos: Vec3::ZERO,
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 100.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 300.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 600.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 1000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 1500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 2000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 2500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 3000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 3500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 4000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 4500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 5000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 5500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 6000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 6500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 7000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 7500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 8000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 8500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 20.0, window.height() / 2.0 + 9000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 20.0, window.height() / 2.0 + 9500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 10.0, window.height() / 2.0 + 10000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 50.0, window.height() / 2.0 + 10500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 50.0, window.height() / 2.0 + 11000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 + 50.0, window.height() / 2.0 + 11500.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });

    commands
    .spawn( 
        SpriteBundle {    
        transform: Transform::from_xyz(window.width() / 2.0 - 50.0, window.height() / 2.0 + 12000.0, 0.0),
        texture: asset_server.load("clouds.png"),
        ..default()
    });
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

// CREATES AND SPAWNS PLATFORMS MANUALLY...WITH ITERATIONS IN SIZE OF PLATFORMS, RANGE OF MOVEMENT, AND COLOR
pub fn spawn_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>,
) {
    let width = 300.0;
    let height = 30.0;

    let window: &Window = window_query.get_single().unwrap();

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(width/2.0, height/2.0)) 
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(width, height)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
    )));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 20.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0 + 12000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 100.0,
            max_x: 150.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 20.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0 + 11500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 200.0,
            max_x: 250.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 20.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0 + 11000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 250.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 20.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0 + 10500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 300.0,
            max_x: 350.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 20.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0 + 10000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 200.0,
            max_x: 250.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(20.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 40.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 9500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 200.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(20.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 40.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 9000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(20.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 40.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 8500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(20.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 40.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 8000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(30.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 60.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 7500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(30.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 60.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 7000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(40.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 80.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 6500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(40.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 80.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 6000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));
    

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(40.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 80.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 5500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 5000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 4500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 4000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 3500.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 3000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));





    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 2750.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 200.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 2350.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 50.0,
            max_x: 300.0,
        }
    ));


    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 200.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 2000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 5.0,
            max_x: 550.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(150.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 300.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 1750.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 5.0,
            max_x: 550.0,
        }
    ));



    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 100.0,
                y: 20.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 30.0,
                window.height() / 2.0 + 1450.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 5.0,
            max_x: 550.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x:200.0,
                y: 20.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 50.0,
                window.height() / 2.0 + 1150.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 5.0,
            max_x: 550.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(90.0, 5.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 180.0,
                y: 10.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 150.0,
                window.height() / 2.0 + 880.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 5.0,
            max_x: 100.0,
        }
    ));

    commands
    .spawn(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(width/2.0, height/2.0)) 
    .insert(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::new(width, height)).into()).into(),
        material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
        ..default()
    })
    .insert(TransformBundle::from(Transform::from_xyz(
        window.width() / 2.0 - 100.0,
        window.height() / 2.0 + 800.0,
        0.0,
    )))
    .insert(MovingPlatform {
        direction: 1,
        min_x: 50.0,
        max_x: 300.0
    });

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x:200.0,
                y: 20.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 75.0,
                window.height() / 2.0 + 650.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 30.0,
            max_x: 500.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x:200.0,
                y: 20.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 100.0,
                window.height() / 2.0 + 450.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: -1,
            min_x: 0.0,
            max_x: 450.0,
        }
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 15.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x:200.0,
                y: 30.0,
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 + 200.0,
                window.height() / 2.0 + 1000.0,
                0.0)),
            ..default()
        },
        MovingPlatform {
            direction: 1,
            min_x: 100.0,
            max_x: 300.0,
        }
    ));

    commands
    .spawn(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(width/2.0, height/2.0)) 
    .insert(MovingPlatform {
        direction: 1,
        min_x: 0.0,
        max_x: 200.0
    })
    .insert(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::new(width, height)).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
        ..default()
    })
    .insert(TransformBundle::from(Transform::from_xyz(
        window.width() / 2.0 - 200.0,
        window.height() / 2.0 + 300.0,
        0.0,
    )));

    //ball
    /* 
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            (window.width() / 2.0) - 50.0,
            window.height() / 1.0,
            0.0,
        )));
        */
}


// MOVES PLATFORMS SIDE TO SIDE WITH RANGE OF X DIRECTION PARAMETERS
fn modify_body_translation(
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
    mut direction: Local<i32>
) {
    if *direction != 1 && *direction != -1 {
        *direction = 1;
    }

    for (mut position, mut moving_platform) in query.iter_mut() {
       
        if moving_platform.direction == 1 {
            position.translation.x += 1.0;
        } else {
            position.translation.x -= 1.0;
        }
        
        if position.translation.x >= moving_platform.max_x {
            moving_platform.direction = -1;
        } else if position.translation.x <= moving_platform.min_x {
            moving_platform.direction = 1;
        }
        
    }
}

/*  PREVIOUS ITERATION OF PLAYER MOVEMENT (UNUSED)
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-0.5, 0.0, 0.0);

        }

        if keyboard_input.pressed(KeyCode::Right)  {
            direction += Vec3::new(0.5, 0.0, 0.0);
        }

        /* 
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        */

        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
*/

fn move_player (
    mut controllers: Query< &mut KinematicCharacterController>,
    controller_output: Query<&KinematicCharacterControllerOutput>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut background: Query<&mut BackGround>,
) {
    let mut to_move: Vec2 = Vec2::ZERO;
    if let Ok(_output) = controller_output.get_single() {
        //info!("{:?}", output);
    }

    
    if keyboard.pressed(KeyCode::Up) {
        to_move.y += 550.0;
        for mut bg in background.iter_mut() {
            bg.pos.y += 550.0 * time.delta_seconds();
        }
    }
    
    
    // Physics Debug Use
    /* 
    if keyboard.pressed(KeyCode::Down) {
        to_move.y -= 100.0 
    }
    */
    if keyboard.pressed(KeyCode::Left) {
        to_move.x -= 250.0 
    }
    if keyboard.pressed(KeyCode::Right) {
        to_move.x += 250.0 
    }
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(to_move * time.delta_seconds())
    }
}

#[derive(Component)]
pub struct Jump(pub f32);

pub fn player_jumping(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands : Commands,
    mut _player: Query<(Entity, &mut Transform), With<Player>>,
    _time: Res<Time>,
) {
    let (entity, _player) = _player.single_mut();

    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Up) {
        commands.entity(entity).insert(Jump(10.0));
    }
}

pub fn player_jump(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform,mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 1.5).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}


pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0 - 50.0; //32.0
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

fn music_setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    //let music = asset_server.load("sounds/Windless Slopes.ogg");
    //let music = asset_server.load("Child's Nightmare.ogg");
    let music = asset_server.load("Child's Nightmare.ogg");
    
    audio.play(music);
}












