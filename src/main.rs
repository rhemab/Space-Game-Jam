#![windows_subsystem = "windows"]

use bevy::{prelude::*, window::PrimaryWindow};
use components::*;

mod base;
mod coal;
mod components;
mod copper;
mod gold;
mod iron;
mod player;
mod rock;
mod stats;
mod trades;

const SOUND_TRACK: &str = "sound-track.mp3";

const PLAYER_SPRITE: &str = "player.png";
const GOLD_SPRITE: &str = "gold.png";
const IRON_SPRITE: &str = "iron.png";
const COPPER_SPRITE: &str = "copper.png";
const COAL_SPRITE: &str = "coal.png";
const STATS_SPRITE: &str = "stats.png";
const BASE_SPRITE: &str = "base.png";
const ROCK1_SPRITE: &str = "rock1.png";
const ROCK2_SPRITE: &str = "rock2.png";
const ROCK3_SPRITE: &str = "rock3.png";
const ROCK4_SPRITE: &str = "rock4.png";

const PLAYER_SIZE: (f32, f32) = (14.0, 13.0);
const BASE_SIZE: (f32, f32) = (30.0, 20.0);
const GOLD_SIZE: (f32, f32) = (6.0, 6.0);
const ROCK1_SIZE: (f32, f32) = (10.0, 10.0);
const ROCK2_SIZE: (f32, f32) = (16.0, 8.0);
const ROCK3_SIZE: (f32, f32) = (11.0, 9.0);
const ROCK4_SIZE: (f32, f32) = (6.0, 5.0);

const SPRITE_SCALE: f32 = 2.0;
const BASE_SPEED: f32 = 100.0;

const MAX_ROCKS: usize = 70;
const MAX_GOLD: usize = 1;
const MAX_IRON: usize = 3;
const MAX_COPPER: usize = 3;
const MAX_COAL: usize = 3;

const MAX_SHIP_STORAGE: u32 = 10;
const MAX_BASE_STORAGE: u32 = 100;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    base: Handle<Image>,
    gold: Handle<Image>,
    iron: Handle<Image>,
    copper: Handle<Image>,
    coal: Handle<Image>,
    stats: Handle<Image>,
    rock1: Handle<Image>,
    rock2: Handle<Image>,
    rock3: Handle<Image>,
    rock4: Handle<Image>,
}

#[derive(Resource)]
struct ShipStorage {
    gold: u32,
    iron: u32,
    copper: u32,
    coal: u32,
}

#[derive(Resource)]
struct BaseStorage {
    gold: u32,
    iron: u32,
    copper: u32,
    coal: u32,
}

#[derive(Resource)]
struct GameOver(bool);

#[derive(Resource)]
struct PlayerCash(u32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .insert_resource(GameOver(false))
        .insert_resource(PlayerCash(500))
        .insert_resource(ShipStorage {
            gold: 0,
            iron: 0,
            copper: 0,
            coal: 0,
        })
        .insert_resource(BaseStorage {
            gold: 0,
            iron: 0,
            copper: 0,
            coal: 0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spacy Trade".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(base::BasePlugin)
        .add_plugins(rock::RockPlugin)
        .add_plugins(stats::StatsPlugin)
        .add_plugins(gold::GoldPlugin)
        .add_plugins(iron::IronPlugin)
        .add_plugins(copper::CopperPlugin)
        .add_plugins(coal::CoalPlugin)
        .add_plugins(trades::TradesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        AudioPlayer::new(asset_server.load(SOUND_TRACK)),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
    ));
    commands.spawn(MarketTimer::default());

    // capture window size
    let Ok(primary) = query.single() else {
        return;
    };
    let (win_w, win_h) = (primary.width(), primary.height());
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        base: asset_server.load(BASE_SPRITE),
        gold: asset_server.load(GOLD_SPRITE),
        iron: asset_server.load(IRON_SPRITE),
        copper: asset_server.load(COPPER_SPRITE),
        coal: asset_server.load(COAL_SPRITE),
        stats: asset_server.load(STATS_SPRITE),
        rock1: asset_server.load(ROCK1_SPRITE),
        rock2: asset_server.load(ROCK2_SPRITE),
        rock3: asset_server.load(ROCK3_SPRITE),
        rock4: asset_server.load(ROCK4_SPRITE),
    };

    commands.insert_resource(game_textures);
}

fn movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
    time: Res<Time>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        // reduce speed when moving diagnolly
        let mut speed_offset = 1.0;
        if velocity.x != 0.0 && velocity.y != 0.0 {
            speed_offset = 0.75;
        }

        let translation = &mut transform.translation;
        let delta = time.delta_secs();
        let x = translation.x + velocity.x * delta * BASE_SPEED * speed_offset;
        let y = translation.y + velocity.y * delta * BASE_SPEED * speed_offset;

        translation.x = x;
        translation.y = y;

        if movable.auto_despawn {
            let margin = 30.0;
            if translation.y > win_size.h / 2. + margin
                || translation.y < -win_size.h / 2. - margin
                || translation.x > win_size.w / 2. + margin
                || translation.x < -win_size.w / 2. - margin
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
