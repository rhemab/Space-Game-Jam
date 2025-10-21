use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    GameTextures, MAX_ROCKS, ROCK1_SIZE, ROCK2_SIZE, ROCK3_SIZE, ROCK4_SIZE, SPRITE_SCALE, WinSize,
    components::{Base, Movable, Rock, SpriteSize, Velocity},
};

pub struct RockPlugin;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rock_movement, rock_spawn));
    }
}

fn rock_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    rock_query: Query<&Rock>,
) {
    if rock_query.iter().len() > MAX_ROCKS {
        return;
    }

    let mut rocks = vec![];
    rocks.push((game_textures.rock1.clone(), ROCK1_SIZE));
    rocks.push((game_textures.rock2.clone(), ROCK2_SIZE));
    rocks.push((game_textures.rock3.clone(), ROCK3_SIZE));
    rocks.push((game_textures.rock4.clone(), ROCK4_SIZE));

    for rock in &rocks {
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
                Sprite::from_image(rock.0.clone()),
                Transform {
                    translation: target_position,
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
            ))
            .insert(Rock)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity {
                x: x / 5000.0,
                y: y / 5000.0,
            })
            .insert(SpriteSize::from(rock.1));
    }
}

// move rocks slowly
// despawn when colliding with base
fn rock_movement(
    mut commands: Commands,
    base_query: Query<(&Transform, &SpriteSize), With<Base>>,
    rock_query: Query<(Entity, &Transform, &SpriteSize), With<Rock>>,
) {
    if let Ok((base_tf, base_size)) = base_query.single() {
        let base_scale = Vec2::from(base_tf.scale.xy());

        for (rock, rock_tf, rock_size) in rock_query {
            let rock_scale = Vec2::from(rock_tf.scale.xy());

            let collision = Aabb2d::new(
                base_tf.translation.truncate(),
                base_size.0 * 2.0 * base_scale,
            )
            .intersects(&Aabb2d::new(
                rock_tf.translation.truncate(),
                rock_size.0 * rock_scale,
            ));

            if collision {
                commands.entity(rock).despawn();
            }
        }
    }
}
