use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::{BaseStorage, GameOver, PlayerCash, components::MarketTimer};

use thousands::Separable;

enum ResourceType {
    Gold,
    Iron,
    Copper,
    Coal,
}

pub struct Resource {
    res_type: ResourceType,
    name: String,
    price: u32,
}

#[derive(Resource)]
pub struct ResourceList(Vec<Resource>);

pub struct TradesPlugin;
impl Plugin for TradesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceList(vec![
            Resource {
                res_type: ResourceType::Gold,
                name: format!("Gold"),
                price: 4200,
            },
            Resource {
                res_type: ResourceType::Iron,
                name: format!("Iron"),
                price: 500,
            },
            Resource {
                res_type: ResourceType::Copper,
                name: format!("Copper"),
                price: 600,
            },
            Resource {
                res_type: ResourceType::Coal,
                name: format!("Coal"),
                price: 1200,
            },
        ]))
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, generate_market);
    }
}

fn generate_market(
    mut contexts: EguiContexts,
    mut market_timer: Single<&mut MarketTimer>,
    mut base: ResMut<BaseStorage>,
    mut cash: ResMut<PlayerCash>,
    mut resource_list: ResMut<ResourceList>,
    time: Res<Time>,
    game_over: Res<GameOver>,
) -> Result {
    if game_over.0 {
        egui::Window::new("Game Over").show(contexts.ctx_mut()?, |ui| {
            ui.label("You ran out of credits...");
        });
        return Ok(());
    }
    market_timer.0.tick(time.delta());
    egui::Window::new("Market").show(contexts.ctx_mut()?, |ui| {
        for resource in &mut resource_list.0 {
            if market_timer.0.is_finished() {
                // update prices
                // high chance of going up slow
                // low chance of crash
                use rand::Rng;
                let mut rng = rand::rng();

                // chance to go up:
                let positive = rng.random_bool(0.90);

                if positive {
                    let price_factor = rng.random_range(30..100); // between 1% - 3%
                    let change = resource.price / price_factor;
                    resource.price += change;
                } else {
                    let change;
                    match resource.res_type {
                        ResourceType::Gold => {
                            change = resource.price / 20; // -5%
                        }
                        _ => {
                            change = resource.price / 10; // -10%
                        }
                    }
                    resource.price -= change;
                }
            }

            // generate ui
            ui.horizontal(|ui| {
                if ui.button("Buy").clicked() {
                    if cash.0 >= resource.price {
                        cash.0 -= resource.price;
                        match resource.res_type {
                            ResourceType::Gold => {
                                base.gold += 1;
                            }
                            ResourceType::Iron => {
                                base.iron += 1;
                            }
                            ResourceType::Copper => {
                                base.copper += 1;
                            }
                            ResourceType::Coal => {
                                base.coal += 1;
                            }
                        }
                    }
                }
                if ui.button("Sell").clicked() {
                    match resource.res_type {
                        ResourceType::Gold => {
                            if base.gold >= 1 {
                                base.gold -= 1;
                                cash.0 += resource.price;
                            }
                        }
                        ResourceType::Iron => {
                            if base.iron >= 1 {
                                base.iron -= 1;
                                cash.0 += resource.price;
                            }
                        }
                        ResourceType::Copper => {
                            if base.copper >= 1 {
                                base.copper -= 1;
                                cash.0 += resource.price;
                            }
                        }
                        ResourceType::Coal => {
                            if base.coal >= 1 {
                                base.coal -= 1;
                                cash.0 += resource.price;
                            }
                        }
                    }
                }
                ui.label(&resource.name);
                ui.label(format!("{}", resource.price.separate_with_commas()));
                ui.label("Galactic Credits");
            });
            ui.separator();
        }
    });
    Ok(())
}
