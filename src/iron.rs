use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    GOLD_SIZE, GameTextures, MAX_IRON, SPRITE_SCALE, WinSize,
    components::{Base, Iron, Movable, SpriteSize, Velocity},
};

pub struct IronPlugin;
impl Plugin for IronPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (iron_movement, iron_spawn));
    }
}

fn iron_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    iron_query: Query<&Iron>,
) {
    if iron_query.iter().len() >= MAX_IRON {
        return;
    }

    use rand::Rng;
    let mut rng = rand::rng();
    let offset = 0.0;
    let w_span = win_size.w / 2.0 + offset;
    let h_span = win_size.h / 2.0 + offset;
    let x = rng.random_range(-w_span..w_span);
    let y = rng.random_range(-h_span..h_span);
    let z = 1.0;
    let target_position = Vec3::new(x, y, z);

    commands
        .spawn((
            Sprite::from_image(game_textures.iron.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Iron)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity {
            x: x / 10000.0,
            y: y / 10000.0,
        })
        .insert(SpriteSize::from(GOLD_SIZE));
}

// move iron slowly
// despawn when colliding with base
fn iron_movement(
    mut commands: Commands,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    iron_query: Query<(Entity, &Transform, &SpriteSize), With<Iron>>,
) {
    if let Ok((base_tf, base_size)) = base_query.single() {
        let base_scale = Vec2::from(base_tf.scale.xy());

        for (iron, iron_tf, iron_size) in iron_query {
            let iron_scale = Vec2::from(iron_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                base_size.0 * 2.0 * base_scale,
            )
            .intersects(&Aabb2d::new(
                iron_tf.translation.truncate(),
                iron_size.0 * iron_scale,
            ));

            if collision {
                commands.entity(iron).despawn();
            }
        }
    }
}
