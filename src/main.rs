use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER_SIZE: (f32, f32) = (98.0, 75.0);
const SPRITE_SCALE: f32 = 1.0;

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
    player: Handle<Image>
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
        .add_startup_system(setup_system)
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
        player : asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures);
    
}