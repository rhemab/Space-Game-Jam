use bevy::prelude::*;

use crate::{
    BASE_SIZE, GameTextures, SPRITE_SCALE,
    components::{Base, SpriteSize},
};

pub struct BasePlugin;
impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, base_spawn);
    }
}

fn base_spawn(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn((
            Sprite::from_image(game_textures.base.clone()),
            Transform {
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Base)
        .insert(SpriteSize::from(BASE_SIZE));
}
