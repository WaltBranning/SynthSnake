use crate::window::WindowSize;
use bevy::audio::PlaybackMode;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

const GRIDLINE_COLOR: Color = Color::srgb(1.2 / 2.0, 0.0, 1.4 / 2.0); // Bright purple (hex: #FF00FF) Dim purple (hex: #330066)
const GRIDLINE_THICKNESS: f32 = 0.50;
const GRIDLINE_SPACING: usize = 100;

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}

impl Score {
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct EndGame {
    pub is_game_over: bool,
}

impl Default for EndGame {
    fn default() -> Self {
        EndGame {
            is_game_over: false,
        }
    }
}

#[derive(Component)]
pub struct Triggered;

#[derive(Component)]
pub struct Callback(pub SystemId);

pub struct GridlinePlugin;

impl Plugin for GridlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gridline);
    }
}

#[derive(Component)]
pub struct Gridline {
    pub color: Color,
    pub thickness: f32,
    pub width: Option<f32>,
}

impl Default for Gridline {
    fn default() -> Self {
        Gridline {
            color: GRIDLINE_COLOR,
            thickness: GRIDLINE_THICKNESS,
            width: None,
        }
    }
}

pub fn spawn_gridline(mut commands: Commands, window: Res<WindowSize>) {
    println! {"Spawning Gridlines with window size horizontal: {:?} Vertical: {:?}", window.width, window.height};
    let gridline = Gridline {
        width: Some(window.width - (GRIDLINE_SPACING * 2) as f32),
        ..default()
    };

    let horizontal_steps = (((-window.height / 2.0 + 100.0) as i32)
        ..=((window.height / 2.0 - 100.0) as i32))
        .step_by(GRIDLINE_SPACING);
    let vertical_steps = (((-window.width / 2.0 + 100.0) as i32)
        ..=((window.width / 2.0 - 100.0) as i32))
        .step_by(GRIDLINE_SPACING);

    for line in horizontal_steps {
        commands.spawn((
            Sprite::from_color(
                gridline.color,
                Vec2::new(gridline.thickness, gridline.width.unwrap()),
            ),
            Transform {
                translation: Vec3::new(line as f32, 0.0, -1.0),
                ..default()
            },
        ));
    }

    for line in vertical_steps {
        commands.spawn((
            Sprite::from_color(
                gridline.color,
                Vec2::new(gridline.width.unwrap(), gridline.thickness),
            ),
            Transform {
                translation: Vec3::new(0.0, line as f32, -1.0),
                ..default()
            },
        ));
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub food_eaten: Handle<AudioSource>,
}

pub fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/SynthSnakeEpic_01.mp3")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        }
    ));

    commands.insert_resource(GameAudio {
        food_eaten: asset_server.load("audio/eat_effect.mp3"),
    });
}
