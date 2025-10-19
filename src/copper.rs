use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    GOLD_SIZE, GameTextures, MAX_COPPER, SPRITE_SCALE, WinSize,
    components::{Base, Copper, Movable, SpriteSize, Velocity},
};

pub struct CopperPlugin;
impl Plugin for CopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (copper_movement, copper_spawn));
    }
}

fn copper_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    copper_query: Query<&Copper>,
) {
    if copper_query.iter().len() >= MAX_COPPER {
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
            Sprite::from_image(game_textures.copper.clone()),
            Transform {
                translation: target_position,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Copper)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity {
            x: x / 10000.0,
            y: y / 10000.0,
        })
        .insert(SpriteSize::from(GOLD_SIZE));
}

// move copper slowly
// despawn when colliding with base
fn copper_movement(
    mut commands: Commands,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    copper_query: Query<(Entity, &Transform, &SpriteSize), With<Copper>>,
) {
    if let Ok((base_tf, base_size)) = base_query.single() {
        let base_scale = Vec2::from(base_tf.scale.xy());

        for (copper, copper_tf, copper_size) in copper_query {
            let copper_scale = Vec2::from(copper_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                base_size.0 * 2.0 * base_scale,
            )
            .intersects(&Aabb2d::new(
                copper_tf.translation.truncate(),
                copper_size.0 * copper_scale,
            ));

            if collision {
                commands.entity(copper).despawn();
            }
        }
    }
}
