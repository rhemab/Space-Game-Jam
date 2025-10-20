use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{BaseStorage, PlayerCash, components::MaintenanceTimer};

use thousands::Separable;

const RESOURCE_LIST: [(&str, u32); 5] = [
    ("Gold", 4200),
    ("Iron", 500),
    ("Copper", 500),
    ("Coal", 1200),
    ("Galactic Credits", 1),
];

pub struct Offer {
    title: String,
    give: usize,
    give_qty: u32,
    get: u32,
}

#[derive(Resource)]
pub struct CurrentOffers(Vec<Offer>);

pub struct TradesPlugin;
impl Plugin for TradesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentOffers(vec![]))
            .add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, generate_trades);
    }
}

fn generate_trades(
    mut contexts: EguiContexts,
    mut offers: ResMut<CurrentOffers>,
    mut base: ResMut<BaseStorage>,
    mut cash: ResMut<PlayerCash>,
    maintenance_timer: Single<&MaintenanceTimer>,
) -> Result {
    if maintenance_timer.0.is_finished() {
        offers.0.remove(0);
    }
    egui::Window::new("Trade").show(contexts.ctx_mut()?, |ui| {
        // only show 6 offers at a time
        if offers.0.len() < 6 {
            // generate a random offer
            use rand::Rng;
            let mut rng = rand::rng();

            let i = rng.random_range(0..4);
            let x = RESOURCE_LIST[i];
            let x_qty = rng.random_range(1..10);

            // y is cash
            let y = RESOURCE_LIST[4];
            let y_qty = x.1 * x_qty;

            let new_offer = Offer {
                title: format!(
                    "Trade {} {} for {} {}",
                    x_qty.separate_with_commas(),
                    x.0,
                    y_qty.separate_with_commas(),
                    y.0
                ),
                give: i,
                give_qty: x_qty,
                get: y_qty,
            };

            offers.0.push(new_offer);
        }

        let mut to_remove = vec![];

        for (i, offer) in offers.0.iter().enumerate() {
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    let mut success = false;
                    let item_to_give = RESOURCE_LIST[offer.give].0;
                    match item_to_give {
                        "Gold" => {
                            if base.gold >= offer.give_qty {
                                base.gold -= offer.give_qty;
                                success = true;
                            }
                        }
                        "Iron" => {
                            if base.iron >= offer.give_qty {
                                base.iron -= offer.give_qty;
                                success = true;
                            }
                        }
                        "Copper" => {
                            if base.copper >= offer.give_qty {
                                base.copper -= offer.give_qty;
                                success = true;
                            }
                        }
                        "Coal" => {
                            if base.coal >= offer.give_qty {
                                base.coal -= offer.give_qty;
                                success = true;
                            }
                        }
                        _ => {}
                    }
                    if success {
                        to_remove.push(i);
                        cash.0 += offer.get;
                    }
                }
                if ui.button("No").clicked() {
                    to_remove.push(i);
                }
                ui.label(&offer.title);
            });
            ui.separator();
        }

        // Remove items in reverse order to avoid shifting indices
        for i in to_remove.iter().rev() {
            offers.0.remove(*i);
        }
    });
    Ok(())
}
