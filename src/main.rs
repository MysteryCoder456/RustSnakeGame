use bevy::{core::FixedTimestep, prelude::*, sprite::collide_aabb};
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Player {
    speed: f32,
    direction: Vec3,
    points: u16,
}

#[derive(Component)]
struct TailPiece {
    tail_position: u16,
}

#[derive(Component)]
struct Food;

const TIME_STEP: f32 = 1.0 / 5.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rusty Snake".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(snake_movement_system)
                .with_system(tail_movement_system)
                .with_system(food_system),
        )
        .add_system(keyboard_input_system)
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    let mut rng = thread_rng();
    let window = windows.get_primary().unwrap();
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
            speed: 30.0,
            direction: Vec3::new(1.0, 0.0, 0.0),
            points: 0,
        });

    // Food
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    rng.gen_range(-window.width() / 2.0, window.width() / 2.0),
                    rng.gen_range(-window.height() / 2.0, window.height() / 2.0),
                    0.0,
                ),
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

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    let mut player = query.single_mut();

    for pressed_key in keyboard_input.get_pressed() {
        match pressed_key {
            KeyCode::W => player.direction = Vec3::new(0.0, 1.0, 0.0),
            KeyCode::S => player.direction = Vec3::new(0.0, -1.0, 0.0),
            KeyCode::A => player.direction = Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::D => player.direction = Vec3::new(1.0, 0.0, 0.0),
            _ => continue,
        }
    }
}

fn snake_movement_system(mut query: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();

    let translation = &mut transform.translation;
    translation.x += player.direction.x * player.speed;
    translation.y += player.direction.y * player.speed;
}

fn tail_movement_system(
    mut tail_query: Query<(&TailPiece, &mut Transform)>,
    player_query: Query<(&Transform, (With<Player>, Without<TailPiece>))>,
) {
    if tail_query.is_empty() {
        return;
    }

    let mut tail = tail_query
        .iter_mut()
        .map(|(tp, transform)| (tp.tail_position, transform))
        .collect::<Vec<(u16, Mut<Transform>)>>();

    let player_transform = player_query.single().0;
    let mut i = tail.len() - 1;

    loop {
        let tail_pos = tail[i].0;
        let next_position: Vec3;

        if tail_pos == 0 {
            next_position = player_transform.translation.clone();
        } else {
            next_position = tail[tail_pos as usize - 1].1.translation.clone();
        }

        let transform = &mut tail[i].1;
        transform.translation = next_position;

        if i <= 0 {
            break;
        } else {
            i -= 1;
        }
    }
}

fn food_system(
    mut commands: Commands,
    mut food_transform_query: Query<(&mut Transform, With<Food>)>,
    player_transform_query: Query<(&Transform, (With<Player>, Without<Food>))>,
    mut player_query: Query<&mut Player>,
    windows: Res<Windows>,
) {
    let mut food_transform = food_transform_query.single_mut().0;
    let player_transform = player_transform_query.single().0;
    let mut player = player_query.single_mut();
    let window = windows.get_primary().unwrap();

    let collision = collide_aabb::collide(
        food_transform.translation,
        food_transform.scale.truncate(),
        player_transform.translation,
        player_transform.scale.truncate(),
    );

    if collision.is_some() {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: player_transform.translation.clone(),
                    scale: Vec3::new(25.0, 25.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::SEA_GREEN,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(TailPiece {
                tail_position: player.points,
            });

        let mut rng = thread_rng();
        player.points += 1;
        food_transform.translation.x = rng.gen_range(-window.width() / 2.0, window.width() / 2.0);
        food_transform.translation.y = rng.gen_range(-window.height() / 2.0, window.height() / 2.0);
    }
}
