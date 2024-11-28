use bevy::{
    prelude::*,
    window::{Window, WindowResolution},
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
};
use crate::game_assets::Score;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 1000.0;

const BACKGROUND_COLOR: Color = Color::srgb(0.02, 0.02, 0.08);  // Very dark blue-black (hex: #050514)
const GAME_OVER_COLOR: Color = Color::srgb(0.1, 0.8, 0.1);
const BACKGROUND_GRID_COLOR: Color = Color::srgb(0.2, 0.0, 0.4);  // Dim purple (hex: #330066)

// Bloom settings
const BLOOM_INTENSITY: f32 = 0.15;
const BLOOM_LOW_FREQUENCY_BOOST: f32 = 0.5;
const BLOOM_LOW_FREQUENCY_BOOST_CURVATURE: f32 = 0.8;
const BLOOM_HIGH_PASS_FREQUENCY: f32 = 0.75;
const BLOOM_MODE: BloomCompositeMode = BloomCompositeMode::Additive;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: BLOOM_INTENSITY.clamp(0.0, 1.0),
            low_frequency_boost: BLOOM_LOW_FREQUENCY_BOOST,
            low_frequency_boost_curvature: BLOOM_LOW_FREQUENCY_BOOST_CURVATURE,
            high_pass_frequency: BLOOM_HIGH_PASS_FREQUENCY,
            composite_mode: BLOOM_MODE,
            ..default()
        }
    ));
}

pub fn setup_window() -> Window {
    let window_size = WindowSize::default();

    Window {
            title: "Snake".to_string(),
            resolution: WindowResolution::new(
                window_size.width,
                 window_size.height),
            ..default()
        }    
}

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize { width: WINDOW_WIDTH, height: WINDOW_HEIGHT }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Creates the UI components
    commands.spawn(
        TextBundle::from_section(
            "Score: 0", 
            TextStyle { 
                font: asset_server.load("fonts/MonaspaceKrypton.ttf"),
                font_size: 40.0,
                 
                color: Color::WHITE, 
                ..default() 
            })
        .with_text_justify(JustifyText::Center)
        .with_style(Style { 
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            margin: UiRect{
                left: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
                ..default()
            },
            ..default()
        })
    );
}

pub fn display_score(score: ResMut<Score>, mut text_query: Query<&mut Text>) {
    for mut text in &mut text_query {
        text.sections[0].value = format!("Score: {}", score.value)
    }   
}

#[derive(Bundle)]
pub struct GameOverWindowBundle {
    pub window: NodeBundle,
} 

impl GameOverWindowBundle {
    pub fn init(score: i32) -> Self {

        let score_text = format!("Score: {}", score);
        let width_val = Val::Percent(65.0);
        let height_val = Val::Percent(50.0);

        GameOverWindowBundle { 
            window: NodeBundle {
                style: Style {
                    width: width_val,
                    height: height_val,
                    position_type: PositionType::Absolute,
                    left: width_val / 4.0,
                    top: height_val / 2.0,
                    
                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                border_color: GAME_OVER_COLOR.into(),
                ..default()
            }
        }
    }
}
