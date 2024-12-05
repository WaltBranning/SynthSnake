mod events;
mod food;
mod game_assets;
mod map;
mod snake;
mod window;

use bevy::{input::keyboard::KeyboardInput, prelude::*};

use events::{check_game_over, FoodRequested};

use snake::{Direction, SnakeHead, SnakePlugin, SnakeSectionsRequested};

use game_assets::{EndGame, Score, Callback, Triggered, GridlinePlugin, setup_audio};

use food::FoodPlugin;

use window::{display_score, setup_camera, setup_ui, setup_window, WindowSize};

use events::{food_ate, restart_button_system};

fn read_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut sections: ResMut<SnakeSectionsRequested>,
    mut food_request: ResMut<FoodRequested>,
    mut snake_direction: Query<&mut Direction, With<SnakeHead>>,
) {
    for event in keyboard_input_events.read() {
        let key_code = match event.key_code {
            KeyCode::KeyW => Some(Direction::Up),
            KeyCode::KeyA => Some(Direction::Left),
            KeyCode::KeyS => Some(Direction::Down),
            KeyCode::KeyD => Some(Direction::Right),
            KeyCode::ArrowUp => Some(Direction::Up),
            KeyCode::ArrowLeft => Some(Direction::Left),
            KeyCode::ArrowDown => Some(Direction::Down),
            KeyCode::ArrowRight => Some(Direction::Right),
            KeyCode::Space => Some(Direction::Pause),
            KeyCode::KeyM => {
                food_request.requested = true;
                None
            }
            KeyCode::KeyN => {
                sections.requested += 1;
                None
            }
            _ => None,
        };
        if let Some(direction) = key_code {
            if let Ok(mut dir) = snake_direction.get_single_mut() {
                *dir = direction;
            }
        }
    }
}

fn main() {
    App::new()
        .add_systems(Startup, (setup_camera, setup_ui, setup_audio).chain())
        .add_systems(
            Update,
            (
                read_input,
                food_ate,
                display_score,
                check_game_over,
                restart_button_system,
                evaluate_callbacks,
            )
                .chain(),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(setup_window()),
            ..default()
        }))
        .add_plugins((SnakePlugin, FoodPlugin, GridlinePlugin))
        .init_resource::<Score>()
        .init_resource::<EndGame>()
        .init_resource::<WindowSize>()
        .run();
}


fn evaluate_callbacks(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
    for (entity, callback) in query.iter() {
        commands.run_system(callback.0);
        commands.entity(entity).remove::<Triggered>();
    }
}
