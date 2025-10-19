use bevy::{math::Vec2, prelude::Component};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Base;

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct Gold;

#[derive(Component)]
pub struct Iron;

#[derive(Component)]
pub struct Copper;

#[derive(Component)]
pub struct Coal;

#[derive(Component)]
pub struct Stats;

#[derive(Component)]
pub struct ShipStorageUi;

#[derive(Component)]
pub struct BaseStorageUi;

#[derive(Component)]
pub struct PlayerCashUi;

#[derive(Component)]
pub struct Trade1;

#[derive(Component)]
pub struct Trade2;

#[derive(Component)]
pub struct Trade3;

#[derive(Component)]
pub struct Trade4;

#[derive(Component)]
pub struct Trade5;

#[derive(Component)]
pub struct Trade6;

#[derive(Component)]
pub struct Trade7;

#[derive(Component)]
pub struct Trade8;

#[derive(Component)]
pub struct Trade9;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1))
    }
}
