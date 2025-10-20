use bevy::prelude::*;

use crate::{
    BaseStorage, GameTextures, MAX_BASE_STORAGE, MAX_SHIP_STORAGE, PlayerCash, SPRITE_SCALE,
    ShipStorage, WinSize,
    components::{
        BaseStorageUi, CoalCount, CopperCount, GoldCount, IronCount, MaintenanceTimer,
        PlayerCashUi, ShipStorageUi, Stats,
    },
};

use thousands::Separable;

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, stats_spawn)
            .add_systems(Update, update_stats);
    }
}

fn stats_spawn(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>) {
    let bottom = -win_size.h / 2.0;
    let target_position = Vec3::new(0., bottom / 2. * SPRITE_SCALE + 20., 2.0);
    commands
        .spawn((
            Sprite::from_image(game_textures.stats.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Stats);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Start,
            ..default()
        },
        children![
            (
                GoldCount,
                Text::default(),
                BackgroundColor(Color::srgba(0.04, 0.04, 0.04, 0.8)),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(13.0)),
                    ..default()
                },
            ),
            (
                IronCount,
                Text::default(),
                BackgroundColor(Color::srgba(0.04, 0.04, 0.04, 0.8)),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(13.0)),
                    ..default()
                },
            ),
            (
                CopperCount,
                Text::default(),
                BackgroundColor(Color::srgba(0.04, 0.04, 0.04, 0.8)),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(13.0)),
                    ..default()
                },
            ),
            (
                CoalCount,
                Text::default(),
                BackgroundColor(Color::srgba(0.04, 0.04, 0.04, 0.8)),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(13.0)),
                    ..default()
                },
            )
        ],
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                width: Val::Px(250.0),
                ..default()
            },
            children![
                (
                    Text::default(),
                    TextFont {
                        font_size: 12.0,
                        ..Default::default()
                    },
                    ShipStorageUi,
                ),
                (
                    Text::default(),
                    TextFont {
                        font_size: 12.0,
                        ..Default::default()
                    },
                    Node {
                        margin: UiRect::all(Val::Px(13.0)),
                        ..default()
                    },
                    PlayerCashUi,
                ),
                (
                    Text::default(),
                    TextFont {
                        font_size: 12.0,
                        ..Default::default()
                    },
                    BaseStorageUi,
                )
            ]
        )],
    ));

    commands.spawn(MaintenanceTimer::default());
}

fn update_stats(
    time: Res<Time>,
    mut maintenance_timer: Single<&mut MaintenanceTimer>,
    ship_storage: Res<ShipStorage>,
    base_storage: Res<BaseStorage>,
    mut player_cash: ResMut<PlayerCash>,
    ship_storage_ui: Single<&mut Text, (With<ShipStorageUi>, Without<BaseStorageUi>)>,
    base_storage_ui: Single<&mut Text, With<BaseStorageUi>>,
    player_cash_ui: Single<
        &mut Text,
        (
            With<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
    gold_count: Single<
        &mut Text,
        (
            With<GoldCount>,
            Without<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
    iron_count: Single<
        &mut Text,
        (
            With<IronCount>,
            Without<GoldCount>,
            Without<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
    copper_count: Single<
        &mut Text,
        (
            With<CopperCount>,
            Without<IronCount>,
            Without<GoldCount>,
            Without<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
    coal_count: Single<
        &mut Text,
        (
            With<CoalCount>,
            Without<CopperCount>,
            Without<IronCount>,
            Without<GoldCount>,
            Without<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
) {
    // add ship maintenance costs: $100/30sec
    maintenance_timer.0.tick(time.delta());
    if maintenance_timer.0.is_finished() {
        player_cash.0 -= 100;
    }

    let mut ship_total = ship_storage.gold;
    ship_total += ship_storage.iron;
    ship_total += ship_storage.copper;
    ship_total += ship_storage.coal;

    let mut base_total = base_storage.gold;
    base_total += base_storage.iron;
    base_total += base_storage.copper;
    base_total += base_storage.coal;

    let mut gold = gold_count.into_inner();
    gold.0 = format!("Gold: {}", base_storage.gold);

    let mut iron = iron_count.into_inner();
    iron.0 = format!("Iron: {}", base_storage.iron);

    let mut copper = copper_count.into_inner();
    copper.0 = format!("Copper: {}", base_storage.copper);

    let mut coal = coal_count.into_inner();
    coal.0 = format!("Coal: {}", base_storage.coal);

    let mut ship_storage_text = ship_storage_ui.into_inner();
    ship_storage_text.0 = format!("{}/{}", ship_total, MAX_SHIP_STORAGE);

    let mut base_storage_text = base_storage_ui.into_inner();
    base_storage_text.0 = format!(" {}/{}", base_total, MAX_BASE_STORAGE);

    let mut player_cash_text = player_cash_ui.into_inner();
    player_cash_text.0 = format!("${}", player_cash.0.separate_with_commas());
}
