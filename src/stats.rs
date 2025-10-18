use bevy::prelude::*;

use crate::{
    BaseStorage, GameTextures, PlayerCash, SPRITE_SCALE, ShipStorage, WinSize,
    components::{BaseStorageUi, PlayerCashUi, ShipStorageUi, Stats},
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
}

fn update_stats(
    ship_storage: Res<ShipStorage>,
    base_storage: Res<BaseStorage>,
    player_cash: Res<PlayerCash>,
    mut ship_storage_ui: Query<&mut Text, (With<ShipStorageUi>, Without<BaseStorageUi>)>,
    mut base_storage_ui: Query<&mut Text, With<BaseStorageUi>>,
    mut player_cash_ui: Query<
        &mut Text,
        (
            With<PlayerCashUi>,
            Without<ShipStorageUi>,
            Without<BaseStorageUi>,
        ),
    >,
) {
    for mut text in &mut ship_storage_ui {
        **text = format!("{}/10", ship_storage.gold);
    }
    for mut text in &mut base_storage_ui {
        **text = format!(" {}/100", base_storage.gold);
    }
    for mut text in &mut player_cash_ui {
        **text = format!("${}", player_cash.0.separate_with_commas());
    }
}
