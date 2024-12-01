use crate::game_assets::Score;
use bevy::{
    core_pipeline::{
        bloom::{Bloom, BloomCompositeMode},
        tonemapping::Tonemapping,
    },
    prelude::*,
    window::{Window, WindowResolution},
};

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 1000.0;

// const RESTART_BUTTON_WIDTH: f32 = 50.0;
// const RESTART_BUTTON_HEIGHT: f32 = 25.0;
const RESTART_BUTTON_FONT: &str = "fonts/CyberAlert-xRv8j.otf";
const RESTART_BUTTON_NORMAL_COLOR: Color = Color::srgb(0.15, 0.05, 0.25);
// const RESTART_BUTTON_PRESSED_COLOR: Color = Color::srgb(0.02, 0.02, 0.08);
// const RESTART_BUTTON_HOVERED_COLOR: Color = Color::srgb(0.02, 0.02, 0.08);
const RESTART_BUTTON_BORDER_COLOR: Color = Color::srgb(0.29, 0.10, 0.51);
const RESTART_BUTTON_TEXT: &str = "Try Again";
const RESTART_BUTTON_FONT_SIZE: f32 = 50.0;

const BACKGROUND_COLOR: Color = Color::srgb(0.02, 0.02, 0.08); // Very dark blue-black (hex: #050514)
const GAME_OVER_COLOR: Color = Color::srgb(0.02, 0.02, 0.08); // Very dark blue-black (hex: #050514)
const GAME_OVER_BORDER_COLOR: Color = Color::srgb(0.29, 0.10, 0.51);
// const BACKGROUND_GRID_COLOR: Color = Color::srgb(0.2, 0.0, 0.4); // Dim purple (hex: #330066)

const SCORE_COLOR: Color = Color::srgb(1.0, 1.0, 0.1);
const SCORE_FONT: &str = "fonts/CyberAlert-xRv8j.otf";
const GAME_OVER_FONT: &str = "fonts/CyberAlertItalic-pgo9y.otf";

// Bloom settings
const BLOOM_INTENSITY: f32 = 0.15;
const BLOOM_LOW_FREQUENCY_BOOST: f32 = 0.5;
const BLOOM_LOW_FREQUENCY_BOOST_CURVATURE: f32 = 0.8;
const BLOOM_HIGH_PASS_FREQUENCY: f32 = 0.75;
const BLOOM_MODE: BloomCompositeMode = BloomCompositeMode::Additive;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: BLOOM_INTENSITY.clamp(0.0, 1.0),
            low_frequency_boost: BLOOM_LOW_FREQUENCY_BOOST,
            low_frequency_boost_curvature: BLOOM_LOW_FREQUENCY_BOOST_CURVATURE,
            high_pass_frequency: BLOOM_HIGH_PASS_FREQUENCY,
            composite_mode: BLOOM_MODE,
            ..default()
        },
    ));
}

pub fn setup_window() -> Window {
    let window_size = WindowSize::default();

    Window {
        title: "Snake".to_string(),
        resolution: WindowResolution::new(window_size.width, window_size.height),
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
        WindowSize {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Creates the UI components
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font: asset_server.load(SCORE_FONT),
            font_size: 40.0,
            ..default()
        },
        TextColor(SCORE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            margin: UiRect {
                left: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn display_score(score: ResMut<Score>, mut text_query: Query<&mut Text>) {
    for mut text in &mut text_query {
        text.0 = format!("Score: {}", score.value)
    }
}

pub fn show_game_over_window(mut commands: Commands, asset_server: Res<AssetServer>, score: i32) {
    let score_text = format!("Game Over!\nYour Score: {}", score);
    let width_val = Val::Percent(65.0);
    let height_val = Val::Percent(50.0);

    commands
        .spawn((
            Node {
                width: width_val,
                height: height_val,
                position_type: PositionType::Absolute,
                left: width_val / 4.0,
                top: height_val / 2.0,
                ..default()
            },
            BackgroundColor(GAME_OVER_BORDER_COLOR),
        ))
        .with_children(|background_node| {
            background_node
                .spawn((
                    // This node is the main container for the game over screen.
                    Node {
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        margin: UiRect {
                            top: Val::Percent(0.35),
                            left: Val::Percent(0.35),
                            right: Val::Percent(0.35),
                            bottom: Val::Percent(0.35),
                        },
                        ..default()
                    },
                    BackgroundColor(GAME_OVER_COLOR),
                ))
                .with_children(|foreground_node| {
                    // Game over text
                    foreground_node
                        .spawn(Node {
                            display: Display::Block,
                            height: Val::Percent(50.0),
                            width: Val::Percent(100.0),
                            ..default()
                        })
                        .with_children(|text_div| {
                            text_div.spawn((
                                Text::new(score_text),
                                TextFont {
                                    font: asset_server.load(GAME_OVER_FONT),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(SCORE_COLOR),
                                TextLayout::new_with_justify(JustifyText::Center),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    margin: UiRect {
                                        top: Val::Percent(10.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });

                    // Spawn the restart button
                    foreground_node
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Auto),
                                // width: Val::Percent(RESTART_BUTTON_WIDTH),
                                // height: Val::Percent(RESTART_BUTTON_HEIGHT),
                                padding: UiRect::all(Val::Px(25.0)),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(RESTART_BUTTON_NORMAL_COLOR.into()),
                            BorderColor(RESTART_BUTTON_BORDER_COLOR.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(RESTART_BUTTON_TEXT),
                                TextFont {
                                    font: asset_server.load(RESTART_BUTTON_FONT),
                                    font_size: RESTART_BUTTON_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(SCORE_COLOR),
                            ));
                        });
                });
        });
}
