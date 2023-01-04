use bevy::prelude::*;

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
        .add_startup_system(setup_system)
        .run();
    
}

fn setup_system(mut commands: Commands){
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite : Sprite{
            color : Color::rgb(0.25, 0.25, 0.75),
            custom_size : Some(Vec2::new(150.0, 150.0)),
            ..Default::default()
        },
         ..Default::default()
    });
}