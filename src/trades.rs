use bevy::prelude::*;

use crate::{
    BaseStorage, PlayerCash, ShipStorage,
    components::{
        BaseStorageUi, PlayerCashUi, ShipStorageUi, Trade1, Trade2, Trade3, Trade4, Trade5, Trade6,
        Trade7, Trade8, Trade9,
    },
};

use thousands::Separable;

#[derive(Resource)]
pub struct Prices {
    gold: u32,
    iron: u32,
    copper: u32,
    coal: u32,
}

pub struct TradesPlugin;
impl Plugin for TradesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Prices {
            gold: 4200,
            iron: 500,
            copper: 500,
            coal: 1200,
        })
        .add_systems(PostStartup, trades_spawn)
        .add_systems(Update, update_trades);
    }
}

fn trades_spawn(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                ..default()
            },
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    width: Val::Px(250.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.04, 0.04, 0.04)),
                children![
                    (
                        Trade1,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade2,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade3,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade4,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade5,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade6,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade7,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade8,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        Trade9,
                        Text::default(),
                        TextFont {
                            font_size: 13.0,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    )
                ]
            )],
        ))
        .observe(handle_click);
}

fn update_trades(
    prices: Res<Prices>,
    trade1_ui: Single<&mut Text, With<Trade1>>,
    trade2_ui: Single<&mut Text, (With<Trade2>, Without<Trade1>)>,
    trade3_ui: Single<&mut Text, (With<Trade3>, Without<Trade1>, Without<Trade2>)>,
    trade4_ui: Single<
        &mut Text,
        (
            With<Trade4>,
            Without<Trade1>,
            Without<Trade2>,
            Without<Trade3>,
        ),
    >,
) {
    let mut text1 = trade1_ui.into_inner();
    text1.0 = format!("trade 1 gold for ${}", prices.gold.separate_with_commas());

    let mut text2 = trade2_ui.into_inner();
    text2.0 = format!("trade 1 iron for ${}", prices.iron.separate_with_commas());

    let mut text3 = trade3_ui.into_inner();
    text3.0 = format!(
        "trade 1 copper for ${}",
        prices.copper.separate_with_commas()
    );

    let mut text4 = trade4_ui.into_inner();
    text4.0 = format!("trade 1 coal for ${}", prices.coal.separate_with_commas());
}

fn handle_click(event: On<Pointer<Click>>, trade1_ui: Single<&mut Text, With<Trade5>>) {
    println!("clicked");
    let mut text1 = trade1_ui.into_inner();
    text1.0 = format!("complete!");
}
