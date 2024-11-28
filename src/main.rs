mod map;
mod snake;
mod window;
mod food;
mod events;
mod game_assets;

use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput, 
};

use events::{
    FoodRequested, 
    check_game_over
};

use snake::{
    Direction,
    SnakeHead,
    SnakePlugin,
    SnakeSectionsRequested,
};

use game_assets::{Score, EndGame};

use food::FoodPlugin;

use window::{
    WindowSize,
    display_score,
    setup_ui,
    setup_window,
    setup_camera,
};

use events::food_ate;

fn read_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut sections: ResMut<SnakeSectionsRequested>,
    mut food_request: ResMut<FoodRequested>,
    mut snake_direction: Query<&mut Direction, With<SnakeHead>>
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
            KeyCode::KeyM => {food_request.requested = true; None},
            KeyCode::KeyN => {sections.requested += 1; None},
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
        .add_systems(Startup, (
            setup_camera, 
            setup_ui,
        ).chain())
        .add_systems(Update, (
            read_input, 
            food_ate,    
            display_score,
            check_game_over,
        ).chain())
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(setup_window()),
                
                ..default()
            }))
        .add_plugins((SnakePlugin, FoodPlugin))
        .init_resource::<Score>()
        .init_resource::<EndGame>()
        .init_resource::<WindowSize>()
        .run();
}
