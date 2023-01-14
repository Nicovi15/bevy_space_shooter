use bevy::{prelude::*, sprite::collide_aabb::collide, prelude::Vec3};
use bevy::math::Vec3Swizzles;
use components::{Movable, Velocity, SpriteSize, Enemy, FromPlayer, Laser};
use player::PlayerPlugin;
use enemy::EnemyPlugin;

mod components;
mod player;
mod enemy;

const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER_SIZE: (f32, f32) = (98.0, 75.0);
const SPRITE_SCALE: f32 = 1.0;
const PLAYER_LASER_SPRITE : &str = "laser_a_01.png";
const PLAYER_LASER_SIZE : (f32, f32) = (9.0, 54.0);
const ENEMY_SPRITE : &str = "enemy_a_01.png";
const ENEMY_SIZE : (f32, f32) = (93.0, 84.0);

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;

// Ressources 
#[derive(Resource)]
pub struct WinSize{
    width : f32,
    height : f32
}

#[derive(Resource)]
pub struct GameTextures{
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy : Handle<Image>,
}

fn main() {

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Bevy Space Shooter".to_string(),
				width: 1600.0,
				height: 900.0,
				..Default::default()
			},
			..Default::default()
		}))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .run();
    
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>){
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Capture window size
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {width :window.width(), height : window.height()};
    commands.insert_resource(win_size);

    window.set_position(MonitorSelection::Current, IVec2::new(0, 0));

    // add GameTextures
    let game_textures = GameTextures{
        player : asset_server.load(PLAYER_SPRITE),
        player_laser : asset_server.load(PLAYER_LASER_SPRITE),
        enemy : asset_server.load(ENEMY_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movable_system(mut commands : Commands, win_size : Res<WinSize>, mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>){
    for (entity, velocity, mut transform, movable) in query.iter_mut(){
        let translation = &mut transform.translation;
        *translation += Vec3::new(velocity.x, velocity.y, 0.0).normalize_or_zero() * BASE_SPEED * TIME_STEP;
        //translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        //translation.y += velocity.y * BASE_SPEED * TIME_STEP; 

        if movable.auto_despawn{
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.height / 2.0 + MARGIN
            || translation.y < -win_size.height / 2.0 - MARGIN
            || translation.x > win_size.width / 2.0 + MARGIN
            || translation.x < -win_size.width / 2.0 - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands : Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
)
{
    for (laser_entity, laser_tf, laser_size) in laser_query.iter(){
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter(){
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(laser_tf.translation, laser_size.0 * laser_scale, enemy_tf.translation, enemy_size.0 * enemy_scale);

            if let Some(_) = collision{
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}