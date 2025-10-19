use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    BaseStorage, GameTextures, MAX_BASE_STORAGE, MAX_SHIP_STORAGE, PLAYER_SIZE, SPRITE_SCALE,
    ShipStorage, WinSize,
    components::{Base, Coal, Copper, Gold, Iron, Movable, Player, Rock, SpriteSize, Velocity},
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn)
            .add_systems(Update, (player_movement, unload_at_base));
    }
}

fn player_spawn(mut commands: Commands, game_textures: Res<GameTextures>) {
    // spawn right under base
    let target_position = Vec3::new(0., -40.0, 10.);
    commands
        .spawn((
            Sprite::from_image(game_textures.player.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn player_movement(
    mut ship_storage: ResMut<ShipStorage>,
    input: Res<ButtonInput<KeyCode>>,
    win_size: Res<WinSize>,
    mut commands: Commands,
    mut player_query: Query<
        (&mut Velocity, &mut Transform, &SpriteSize),
        (With<Player>, Without<Base>),
    >,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    rock_query: Query<(&Transform, &SpriteSize), (With<Rock>, Without<Base>, Without<Player>)>,
    gold_query: Query<
        (Entity, &Transform, &SpriteSize),
        (With<Gold>, Without<Base>, Without<Rock>, Without<Player>),
    >,
    iron_query: Query<
        (Entity, &Transform, &SpriteSize),
        (
            With<Iron>,
            Without<Gold>,
            Without<Base>,
            Without<Rock>,
            Without<Player>,
        ),
    >,
    copper_query: Query<
        (Entity, &Transform, &SpriteSize),
        (
            With<Copper>,
            Without<Gold>,
            Without<Base>,
            Without<Rock>,
            Without<Iron>,
            Without<Player>,
        ),
    >,
    coal_query: Query<
        (Entity, &Transform, &SpriteSize),
        (
            With<Coal>,
            Without<Gold>,
            Without<Copper>,
            Without<Base>,
            Without<Rock>,
            Without<Iron>,
            Without<Player>,
        ),
    >,
) {
    if let Ok((mut player_vel, mut player_tf, player_size)) = player_query.single_mut() {
        let mut x = 0.0;
        let mut y = 0.0;

        let a = input.pressed(KeyCode::KeyA);
        let left = input.pressed(KeyCode::ArrowLeft);

        let d = input.pressed(KeyCode::KeyD);
        let right = input.pressed(KeyCode::ArrowRight);

        let w = input.pressed(KeyCode::KeyW);
        let up = input.pressed(KeyCode::ArrowUp);

        let s = input.pressed(KeyCode::KeyS);
        let down = input.pressed(KeyCode::ArrowDown);

        // horizontal movement
        if a || left {
            x = -1.0;
            let new_rotation = Quat::from_rotation_z((90.0_f32).to_radians());
            player_tf.rotation = new_rotation;
        } else if d || right {
            x = 1.0;
            let new_rotation = Quat::from_rotation_z((270.0_f32).to_radians());
            player_tf.rotation = new_rotation;
        }

        // vertical movement
        if w || up {
            y = 1.0;
            let new_rotation = Quat::from_rotation_z((0.0_f32).to_radians());
            player_tf.rotation = new_rotation;
        } else if s || down {
            y = -1.0;
            let new_rotation = Quat::from_rotation_z((180.0_f32).to_radians());
            player_tf.rotation = new_rotation;
        }

        // player cannot move off screen
        let tl = player_tf.translation;
        if tl.x < -win_size.w / 2. + PLAYER_SIZE.1 / 2. && x < 0.0 {
            player_vel.x = 0.0;
            return;
        }
        if tl.x > win_size.w / 2. - PLAYER_SIZE.1 / 2. && x > 0.0 {
            player_vel.x = 0.0;
            return;
        }
        if tl.y < -win_size.h / 2. + PLAYER_SIZE.1 / 2. && y < 0.0 {
            player_vel.y = 0.0;
            return;
        }
        if tl.y > win_size.h / 2. - PLAYER_SIZE.1 / 2. && y > 0.0 {
            player_vel.y = 0.0;
            return;
        }

        // player cannot move through base
        // check collision with base
        if let Ok((base_tf, base_size)) = base_query.single() {
            let base_scale = Vec2::from(base_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                (base_size.0 * base_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                player_vel.x *= -2.0;
                player_vel.y *= -2.0;
                return;
            }
        }

        // player cannot move through rocks
        // check collision with rocks
        for (rock_tf, rock_size) in rock_query {
            let rock_scale = Vec2::from(rock_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                rock_tf.translation.truncate(),
                (rock_size.0 * rock_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                if player_vel.x != 0.0 || player_vel.y != 0.0 {
                    player_vel.x *= -8.0;
                    player_vel.y *= -8.0;
                } else {
                    player_vel.x = -8.0;
                    player_vel.y = -8.0;
                }
                return;
            }
        }

        // player collects gold
        // check collision with gold
        for (gold, gold_tf, gold_size) in gold_query {
            let gold_scale = Vec2::from(gold_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                gold_tf.translation.truncate(),
                (gold_size.0 * gold_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                let mut ship_total = ship_storage.gold;
                ship_total += ship_storage.iron;
                ship_total += ship_storage.copper;
                ship_total += ship_storage.coal;

                if ship_total < MAX_SHIP_STORAGE {
                    commands.entity(gold).despawn();
                    ship_storage.gold += 1;
                }
                return;
            }
        }

        // player collects iron
        // check collision with iron
        for (iron, iron_tf, iron_size) in iron_query {
            let iron_scale = Vec2::from(iron_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                iron_tf.translation.truncate(),
                (iron_size.0 * iron_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                let mut ship_total = ship_storage.gold;
                ship_total += ship_storage.iron;
                ship_total += ship_storage.copper;
                ship_total += ship_storage.coal;

                if ship_total < MAX_SHIP_STORAGE {
                    commands.entity(iron).despawn();
                    ship_storage.iron += 1;
                }
                return;
            }
        }

        // player collects copper
        // check collision with copper
        for (copper, copper_tf, copper_size) in copper_query {
            let copper_scale = Vec2::from(copper_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                copper_tf.translation.truncate(),
                (copper_size.0 * copper_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                let mut ship_total = ship_storage.gold;
                ship_total += ship_storage.iron;
                ship_total += ship_storage.copper;
                ship_total += ship_storage.coal;

                if ship_total < MAX_SHIP_STORAGE {
                    commands.entity(copper).despawn();
                    ship_storage.copper += 1;
                }
                return;
            }
        }

        // player collects coal
        // check collision with coal
        for (coal, coal_tf, coal_size) in coal_query {
            let coal_scale = Vec2::from(coal_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                coal_tf.translation.truncate(),
                (coal_size.0 * coal_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                let mut ship_total = ship_storage.gold;
                ship_total += ship_storage.iron;
                ship_total += ship_storage.copper;
                ship_total += ship_storage.coal;

                if ship_total < MAX_SHIP_STORAGE {
                    commands.entity(coal).despawn();
                    ship_storage.coal += 1;
                }
                return;
            }
        }

        player_vel.x = x;
        player_vel.y = y;
    }
}

fn unload_at_base(
    mut ship_storage: ResMut<ShipStorage>,
    mut base_storage: ResMut<BaseStorage>,
    mut player_query: Query<(&Transform, &SpriteSize), With<Player>>,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
) {
    if let Ok((player_tf, player_size)) = player_query.single_mut() {
        if let Ok((base_tf, base_size)) = base_query.single() {
            let base_scale = Vec2::from(base_tf.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                (base_size.0 * base_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                let mut base_total = base_storage.gold;
                base_total += base_storage.iron;
                base_total += base_storage.copper;
                base_total += base_storage.coal;

                let mut ship_total = ship_storage.gold;
                ship_total += ship_storage.iron;
                ship_total += ship_storage.copper;
                ship_total += ship_storage.coal;

                if base_total + ship_total < MAX_BASE_STORAGE {
                    // add resources to base
                    base_storage.gold += ship_storage.gold;
                    base_storage.iron += ship_storage.iron;
                    base_storage.copper += ship_storage.copper;
                    base_storage.coal += ship_storage.coal;

                    // remove resources from ship
                    ship_storage.gold = 0;
                    ship_storage.iron = 0;
                    ship_storage.copper = 0;
                    ship_storage.coal = 0;
                }
            }
        }
    }
}
