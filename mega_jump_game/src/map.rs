use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MovingPlatform {
    direction: i32,
    max_x: f32,
    min_x: f32,
}

// MOVES PLATFORMS SIDE TO SIDE WITH RANGE OF X DIRECTION PARAMETERS
pub fn modify_body_translation(
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
    mut direction: Local<i32>,
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
            material: materials.add(ColorMaterial::from(Color::BLACK)),
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
