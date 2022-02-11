use bevy::{core::FixedTimestep, prelude::*};

#[derive(Component)]
struct Player {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Food;

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(snake_movement_system)
                .with_system(food_system),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::SEA_GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 200.0,
            direction: Vec3::new(1.0, 0.0, 0.0),
        });

    // Food
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food {});
}

fn snake_movement_system(
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

fn food_system(
    mut food_query: Query<(&mut Transform, With<Food>)>,
    mut player_query: Query<(&Transform, (With<Player>, Without<Food>))>,
) {
    let mut food_transform = food_query.single_mut().0;
    let player_transform = player_query.single_mut().0;

    println!("{:?}", food_transform.scale);
}
