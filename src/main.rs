use std::ops::Add;

use bevy::{core::FixedTimestep, prelude::*};

#[derive(Component)]
struct Player {
    speed: f32,
    direction: Vec3,
}

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(snake_movement),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::SEA_GREEN,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 400.0,
            direction: Vec3::new(1.0, 0.0, 0.0),
        });
}

// System that handles keyboard input
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    for pressed_key in keyboard_input.get_pressed() {
        match pressed_key {
            KeyCode::W => player.direction = Vec3::new(0.0, 1.0, 0.0),
            KeyCode::S => player.direction = Vec3::new(0.0, -1.0, 0.0),
            KeyCode::A => player.direction = Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::D => player.direction = Vec3::new(1.0, 0.0, 0.0),
            _ => continue,
        }
    }

    let translation = &mut transform.translation;
    translation.x += player.direction.x * player.speed * TIME_STEP;
    translation.y += player.direction.y * player.speed * TIME_STEP;
}
