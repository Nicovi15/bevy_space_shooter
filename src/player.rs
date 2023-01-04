use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, components::{Player, Velocity}, BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(player_movement_system)
        .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(mut commands : Commands, game_textures: Res<GameTextures>, win_size : Res<WinSize>){
    // Player
    let bottom = -win_size.height / 2.0;
    commands.spawn(SpriteBundle {
        texture : game_textures.player.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
         ..Default::default()
    })
    .insert(Player)
    .insert(Velocity{ x : 0.0, y : 0.0 });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>){
    for (velocity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        *translation += Vec3::new(velocity.x, velocity.y, 0.0).normalize_or_zero() * BASE_SPEED * TIME_STEP;
        //translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        //translation.y += velocity.y * BASE_SPEED * TIME_STEP; 
    }
}

fn player_keyboard_event_system(kb : Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut(){
        velocity.x = 0.0;
        if kb.pressed(KeyCode::Left){ velocity.x += -1.0};
        if kb.pressed(KeyCode::Right){ velocity.x +=1.0};

        velocity.y = 0.0;
        if kb.pressed(KeyCode::Down){ velocity.y += -1.0};
        if kb.pressed(KeyCode::Up){ velocity.y += 1.0};
    }
}