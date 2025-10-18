use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    GOLD_SIZE, GameTextures, MAX_GOLD, SPRITE_SCALE, WinSize,
    components::{Base, Gold, Movable, SpriteSize, Velocity},
};

pub struct GoldPlugin;
impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gold_movement, gold_spawn));
    }
}

fn gold_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    gold_query: Query<&Gold>,
) {
    if gold_query.iter().len() > MAX_GOLD {
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
            Sprite::from_image(game_textures.gold.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Gold)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity {
            x: x / 10000.0,
            y: y / 10000.0,
        })
        .insert(SpriteSize::from(GOLD_SIZE));
}

// move gold slowly
// despawn when colliding with base
fn gold_movement(
    mut commands: Commands,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    gold_query: Query<(Entity, &Transform, &SpriteSize), With<Gold>>,
) {
    if let Ok((base_tf, base_size)) = base_query.single() {
        let base_scale = Vec2::from(base_tf.scale.xy());

        for (gold, gold_tf, gold_size) in gold_query {
            let gold_scale = Vec2::from(gold_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                base_size.0 * 2.0 * base_scale,
            )
            .intersects(&Aabb2d::new(
                gold_tf.translation.truncate(),
                gold_size.0 * gold_scale,
            ));

            if collision {
                commands.entity(gold).despawn();
            }
        }
    }
}
