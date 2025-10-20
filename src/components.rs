use bevy::{
    math::Vec2,
    prelude::Component,
    time::{Timer, TimerMode},
};

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
pub struct GoldCount;

#[derive(Component)]
pub struct IronCount;

#[derive(Component)]
pub struct CopperCount;

#[derive(Component)]
pub struct CoalCount;

#[derive(Component)]
pub struct ShipStorageUi;

#[derive(Component)]
pub struct BaseStorageUi;

#[derive(Component)]
pub struct PlayerCashUi;

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

#[derive(Component)]
pub struct MaintenanceTimer(pub Timer);

impl Default for MaintenanceTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(30.0, TimerMode::Repeating))
    }
}
