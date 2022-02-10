use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::SEA_GREEN,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
