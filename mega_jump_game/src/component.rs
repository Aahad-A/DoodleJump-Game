use bevy::prelude::*;


#[derive(Component)]
pub struct MovingPlatform {
    direction: i32,
    max_x: f32,
    min_x: f32,
}