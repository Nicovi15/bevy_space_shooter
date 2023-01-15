use bevy::math::Vec3Swizzles;
use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, components::{Player, Velocity, Movable, SpriteSize, FromPlayer, Laser, MainCamera}, PLAYER_LASER_SIZE};
use bevy::{prelude::*, render::camera::RenderTarget};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_fire_system)
        .add_system(rotate_player_to_cursor);
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
            let y_offset = 10.0;
            let direction = player_tf.up();
            let angle = player_tf.rotation.to_euler(EulerRot::XYZ).2;
            let cos = f32::cos(angle);
            let sin = f32::sin(angle);
            let mut spawn_laser = |offset: (f32, f32)|{
                commands.spawn(SpriteBundle{
                    texture : game_textures.player_laser.clone(),
                    transform : Transform { 
                        translation: Vec3::new(x, y, 0.0) + Vec3::new(offset.0 * cos - offset.1 * sin, offset.0 * sin - offset.1 * cos, 0.0), 
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0), 
                        rotation : player_tf.rotation.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(Velocity{x: direction.x, y: direction.y})
                .insert(Movable{auto_despawn : true});
            };

            spawn_laser((x_offset, y_offset));
            spawn_laser((-x_offset, y_offset));
        }
    }
}

fn rotate_player_to_cursor(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_player: Query<&mut Transform, With<Player>>
){
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let cursor_pos: Vec2 = world_pos.truncate();

        if let Ok(mut player_tf) = q_player.get_single_mut(){
            let player_pos = player_tf.translation.xy();
            let angle = Vec2::angle_between(Vec2::new(0.0, 1.0), cursor_pos - player_pos);
            player_tf.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
        }
    }
}