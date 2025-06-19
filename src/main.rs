use bevy::{
    ecs::system::SystemId,
    input::common_conditions::input_just_released,
    math::ops::abs,
    platform::collections::HashMap,
    prelude::*,
    sprite::Wireframe2dPlugin,
    window::{PresentMode, PrimaryWindow, WindowMode, WindowTheme},
};

const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 800.0;
const BALL_RADIUS: f32 = 50.0;
const ENTITY_SPAWN: bevy::prelude::Vec3 = Vec3::ZERO;

#[derive(Component, PartialEq)]
struct Movable {
    speed_x: f32,
    speed_y: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    fn new() -> Self {
        Movable {
            speed_x: 0.,
            speed_y: 0.,
        }
    }
}

#[derive(Resource)]
struct Systems(HashMap<String, SystemId>);

impl FromWorld for Systems {
    fn from_world(world: &mut World) -> Self {
        let mut my_item_systems = Systems(HashMap::new());

        my_item_systems
            .0
            .insert("spawn_ball".into(), world.register_system(spawn_ball));

        my_item_systems
    }
}
fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Atomas Clone".into(),
                name: Some("atomas_clone.app".into()),
                resolution: (WIDTH, HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                mode: WindowMode::Windowed,
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        Wireframe2dPlugin::default(),
    ));
    app.add_systems(Startup, (setup, spawn_ball));
    app.add_systems(
        Update,
        shoot_ball.run_if(input_just_released(MouseButton::Left)),
    );
    app.add_systems(Update, move_balls);
    app.init_resource::<Systems>();
    app.run();
}

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
// enum AppState {
//     #[default]
//     Menu,
//     InGame,
// }
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_ball(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    debug!("spawn_ball");
    let r: f32 = rand::random_range(0.0..1.0);
    let g: f32 = rand::random_range(0.0..1.0);
    let b: f32 = rand::random_range(0.0..1.0);

    let color = Color::linear_rgb(r, g, b);
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(ENTITY_SPAWN),
        Movable::new(),
    ));
}

fn shoot_ball(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut balls: Query<(&Transform, &mut Movable)>,
) {
    let mut mouse_position = windows.single().unwrap().cursor_position().unwrap();

    mouse_position = Vec2 {
        x: mouse_position.x - WIDTH / 2.0,
        y: mouse_position.y - HEIGHT / 2.0,
    }
    .normalize();
    debug!("mouse_position: {}", mouse_position);

    for (transform, mut movable) in &mut balls {
        if transform.translation == ENTITY_SPAWN {
            *movable = Movable {
                speed_x: mouse_position.x * 1000.,
                speed_y: mouse_position.y * -1000.,
            };
            debug!("movable: {}, {}", movable.speed_x, movable.speed_y);
        }
    }
}

fn move_balls(
    mut balls: Query<(&mut Transform, &mut Movable)>,
    timer: Res<Time>,
    mut commands: Commands,
    systems: Res<Systems>,
) {
    for (mut transform, mut movable) in &mut balls {
        if movable.speed_x != 0. || movable.speed_y != 0. {
            if abs(transform.translation.y) >= (HEIGHT / 2.0 - 10.)
                || abs(transform.translation.x) >= (WIDTH / 2.0 - 5.)
            {
                debug!("movable in: {}, {}", movable.speed_x, movable.speed_y);

                *movable = Movable {
                    speed_x: 0.,
                    speed_y: 0.,
                };
                debug!("movable out: {}, {}", movable.speed_x, movable.speed_y);

                let id = systems.0["spawn_ball"];
                commands.run_system(id);
            } else {
                debug!(
                    "translation in: {}, {}",
                    transform.translation.x, transform.translation.y
                );
                transform.translation += Vec3 {
                    x: movable.speed_x,
                    y: movable.speed_y,
                    z: 0.,
                } * timer.delta_secs();
                debug!(
                    "translation out: {}, {}",
                    transform.translation.x, transform.translation.y
                );
                debug!("movable: {}, {}", movable.speed_x, movable.speed_y);
            }
        }
    }
}
