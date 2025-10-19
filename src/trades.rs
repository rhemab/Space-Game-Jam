use bevy::prelude::*;

use crate::{
    BaseStorage, PlayerCash, ShipStorage,
    components::{BaseStorageUi, PlayerCashUi, ShipStorageUi, Trades},
};

use thousands::Separable;

const RESOURCE_LIST: [(&str, f32); 5] = [
    ("Gold", 4200.0),
    ("Iron", 500.0),
    ("Copper", 500.0),
    ("Coal", 1200.0),
    ("Galactic Credits", 1.0),
];

#[derive(Resource)]
pub struct CurrentOffers(u32);

pub struct TradesPlugin;
impl Plugin for TradesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentOffers(0))
            .add_systems(PostStartup, trades_spawn)
            .add_systems(Update, update_trades);
    }
}

fn trades_spawn(mut commands: Commands) {
    commands.spawn((
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
            BackgroundColor(Color::srgba(0.04, 0.04, 0.04, 0.8)),
            Trades,
        )],
    ));
}

fn update_trades(
    mut commands: Commands,
    mut offers: ResMut<CurrentOffers>,
    trades_ui: Single<Entity, With<Trades>>,
) {
    // only show 6 offers at a time
    if offers.0 < 4 {
        // generate a random offer
        use rand::Rng;
        let mut rng = rand::rng();

        let x = RESOURCE_LIST[rng.random_range(0..4)];
        let x_qty = rng.random_range(1..10) as f32;

        // y is cash
        let y = RESOURCE_LIST[4];
        let y_qty = x.1 * x_qty;

        let child = commands
            .spawn((
                Text::new(format!(
                    "Trade {} {} for {} {}",
                    x_qty.separate_with_commas(),
                    x.0,
                    y_qty.separate_with_commas(),
                    y.0
                )),
                TextFont {
                    font_size: 13.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ))
            .observe(
                |event: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut offers: ResMut<CurrentOffers>| {
                    commands.entity(event.entity).despawn();
                    offers.0 -= 1;
                },
            )
            .id();

        commands
            .entity(trades_ui.into_inner())
            .add_children(&[child]);

        offers.0 += 1;
    }
}
