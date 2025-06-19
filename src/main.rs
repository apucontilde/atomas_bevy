use bevy::{
    math::ops::abs,
    prelude::*,
    sprite::Wireframe2dPlugin,
    window::{PresentMode, PrimaryWindow, WindowMode, WindowTheme},
};
const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 800.0;
fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Atomas Clone".into(),
                name: Some("atomas_clone.app".into()),
                resolution: (WIDTH, HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                mode: WindowMode::Windowed,
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        Wireframe2dPlugin::default(),
    ));
    app.add_systems(Startup, (setup, spawn_ball));
    app.add_systems(Update, spawn_ball.run_if(should_spawn_ball));
    app.add_systems(Update, shoot_ball);
    app.add_systems(Update, move_balls);
    app.run();
}

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
// enum AppState {
//     #[default]
//     Menu,
//     InGame,
// }

const BALL_RADIUS: f32 = 50.0;
const ENTITY_SPAWN: bevy::prelude::Vec3 = Vec3::ZERO;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

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

fn spawn_ball(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
        let hue: f32 = rand::random_range(0.0..1.0);
        let color = Color::hsl(hue, 0.95, 0.7);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(ENTITY_SPAWN),
            Movable::new(),
        ));
}

fn should_spawn_ball(balls: Query<&Transform>) -> bool {
    info!("should_spawn_ball: {}", balls.iter().any(|ball| ball.translation == ENTITY_SPAWN));

    return !balls.iter().any(|ball| ball.translation == ENTITY_SPAWN)
}

fn shoot_ball(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut balls: Query<(&Transform, &mut Movable)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    let Some(mouse_position) = windows.iter().next().and_then(Window::cursor_position) else {
        return;
    };
    info!("mouse_position: {}", mouse_position.extend(0.));
    for (transform, mut movable) in &mut balls {

        if transform.translation == ENTITY_SPAWN {
            *movable = Movable {
                speed_x: mouse_position.x - HEIGHT / 2.0,
                speed_y: mouse_position.y - WIDTH / 2.0,
            };
        }
    }
}

fn move_balls(mut balls: Query<(&mut Transform, &mut Movable)>, timer: Res<Time>) {
    for (mut transform, mut movable) in &mut balls {
        
            if abs(transform.translation.y) >= (HEIGHT / 2.0 -10.)
                || abs(transform.translation.x) >= (WIDTH / 2.0 -5.)
            {
            *movable = Movable {
                    speed_x: 0.,
                    speed_y: 0.,
                };
            }

            if movable.speed_x > 0. || movable.speed_y > 0. {

                transform.translation += Vec3 {
                    x: movable.speed_x,
                    y: movable.speed_y,
                    z: 0.,
                } * timer.delta_secs()
                
            }

        
    }
}
