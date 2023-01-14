use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, components::{Player, Velocity, Movable, SpriteSize, FromPlayer, Laser}, BASE_SPEED, TIME_STEP, PLAYER_LASER_SIZE};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_fire_system);
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
    .insert(SpriteSize::from(PLAYER_SIZE))
    .insert(Velocity{ x : 0.0, y : 0.0 })
    .insert(Movable{auto_despawn: false});
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

fn player_fire_system(mut commands : Commands, kb : Res<Input<KeyCode>>, game_textures: Res<GameTextures>, query: Query<&Transform, With<Player>>){
    if let Ok(player_tf) = query.get_single(){
        if kb.just_pressed(KeyCode::Space){
            let (x,y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.0 * SPRITE_SCALE - 12.5;

            let mut spawn_laser = |x_offset: f32|{
                commands.spawn(SpriteBundle{
                    texture : game_textures.player_laser.clone(),
                    transform : Transform { 
                        translation: Vec3::new(x + x_offset, y + 10.0, 0.0), 
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0), 
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(Velocity{x: 0.0, y: 1.0})
                .insert(Movable{auto_despawn : true});
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}