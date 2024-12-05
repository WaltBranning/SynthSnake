use bevy::{
    audio::PlaybackMode, math::bounding::{Aabb2d, IntersectsVolume}, prelude::*
};
use rand::Rng;

use crate::food::{Food, FoodRequest};
use crate::game_assets::{EndGame, GameAudio, Triggered};
use crate::game_assets::Score;
use crate::snake::{Direction, SnakeHead, SnakePart, SnakeSectionsRequested, SnakeRequest};
use crate::window::{show_game_over_window, GameOverWindow, WindowSize};

#[derive(Resource, Debug)]
pub struct FoodRequested {
    pub requested: bool,
}

impl Default for FoodRequested {
    fn default() -> Self {
        FoodRequested { requested: true }
    }
}

pub fn food_ate(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut section_requested: ResMut<SnakeSectionsRequested>,
    mut food_query: Query<(&mut Transform, &mut Food), Without<SnakeHead>>,
    mut head_query: Query<(&Transform, &mut SnakePart), With<SnakeHead>>,
    game_audio: Res<GameAudio>,
) {
    if let Ok((mut food_transform, mut food)) = food_query.get_single_mut() {
        if let Ok((_head_transform, mut head)) = head_query.get_single_mut() {
            if food.bounds.intersects(&head.bounds) {
                head.speed += 0.25;
                let x_rand: f32 = rand::thread_rng().gen_range(-400.0..=400.0);
                let y_rand: f32 = rand::thread_rng().gen_range(-300.0..=300.0);
                food.bounds =
                    Aabb2d::new(Vec2::new(x_rand, y_rand), Vec2::new(food.scale, food.scale));
                food_transform.translation = Vec3::new(x_rand, y_rand, 0.0);
                score.increment();
                section_requested.requested += 1;

                commands.spawn((
                    AudioPlayer::new(game_audio.food_eaten.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Once,
                        ..default()
                    }
                ));
            }
        };
    }
}

pub fn check_game_over(
    commands: Commands,
    mut end_game: ResMut<EndGame>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    window: Res<WindowSize>,
    mut head_query: Query<(&Transform, &mut Direction, &mut SnakePart), With<SnakeHead>>,
) {
    if let Ok((head_transform, mut direction, head)) = head_query.get_single_mut() {
        let border_intersects = |head: &Vec3, offset: f32, win: &WindowSize| -> bool {
            [
                (head.x + offset / 2.0) >= (win.width / 2.0),
                (head.x - offset / 2.0) <= -(win.width / 2.0),
                (head.y + offset / 2.0) >= (win.height / 2.0),
                (head.y - offset / 2.0) <= -(win.height / 2.0),
            ]
            .into_iter()
            .any(|v| v)
        };

        if border_intersects(&head_transform.translation, head.scale, &window) {
            end_game.is_game_over = true;
            *direction = Direction::Pause;
            show_game_over_window(commands, asset_server, score.value);
        }
    };
    // Check if the head is out of bounds of the window
}

pub fn restart_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    snake_parts_query: Query<Entity, With<SnakePart>>,
    food_query: Query<Entity, With<Food>>,
    mut score: ResMut<Score>,
    mut end_game: ResMut<EndGame>,
    game_over_window_query: Query<Entity, With<GameOverWindow>>,
    food_request_query: Single<Entity, With<FoodRequest>>,
    snake_request_query: Single<Entity, With<SnakeRequest>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                
                snake_parts_query.into_iter().for_each(|snake_part| {
                    commands.entity(snake_part).despawn_recursive();
                });

                food_query.into_iter().for_each(|food| {
                    commands.entity(food).despawn_recursive();
                });

                game_over_window_query.into_iter().for_each(|game_over_window| {
                    commands.entity(game_over_window).despawn_recursive();
                });

                score.reset();
                end_game.is_game_over = false;

                // Repawn all the entities to reset the game
                commands.entity(*food_request_query).insert(Triggered);
                commands.entity(*snake_request_query).insert(Triggered);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(background_color.0.lighter(0.5));
                // border_color.0 = Color::WHITE.into();
            }
            _ => {}
        }
    }
}
