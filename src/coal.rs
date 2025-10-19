use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    GOLD_SIZE, GameTextures, MAX_COAL, SPRITE_SCALE, WinSize,
    components::{Base, Coal, Movable, SpriteSize, Velocity},
};

pub struct CoalPlugin;
impl Plugin for CoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (coal_movement, coal_spawn));
    }
}

fn coal_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    coal_query: Query<&Coal>,
) {
    if coal_query.iter().len() >= MAX_COAL {
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
            Sprite::from_image(game_textures.coal.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Coal)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity {
            x: x / 10000.0,
            y: y / 10000.0,
        })
        .insert(SpriteSize::from(GOLD_SIZE));
}

// move coal slowly
// despawn when colliding with base
fn coal_movement(
    mut commands: Commands,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    coal_query: Query<(Entity, &Transform, &SpriteSize), With<Coal>>,
) {
    if let Ok((base_tf, base_size)) = base_query.single() {
        let base_scale = Vec2::from(base_tf.scale.xy());

        for (coal, coal_tf, coal_size) in coal_query {
            let coal_scale = Vec2::from(coal_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                base_size.0 * 2.0 * base_scale,
            )
            .intersects(&Aabb2d::new(
                coal_tf.translation.truncate(),
                coal_size.0 * coal_scale,
            ));

            if collision {
                commands.entity(coal).despawn();
            }
        }
    }
}
