use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, components::{Player, Velocity, Movable, SpriteSize, Enemy}, BASE_SPEED, TIME_STEP, ENEMY_SIZE};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app : &mut App){
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>){

    let mut rng = thread_rng();
    let w_span = win_size.width / 2.0 - 100.0;
    let h_span = win_size.height / 2.0 - 100.0;
    let coord : (f32, f32) = (rng.gen_range(-w_span..w_span), rng.gen_range(-h_span..h_span));

    commands.spawn(SpriteBundle{
        texture : game_textures.enemy.clone(),
        transform : Transform{
            translation: Vec3::new(coord.0, coord.1, 10.0),
            scale : Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Enemy)
    .insert(SpriteSize::from(ENEMY_SIZE));
}