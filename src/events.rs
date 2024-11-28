use rand::Rng;
use bevy::{
    prelude::*,
    math::bounding::{
        IntersectsVolume,
        Aabb2d,
    }
};

use crate::game_assets::Score;
use crate::food::Food;
use crate::snake::{SnakeHead, SnakePart, SnakeSectionsRequested, Direction};
use crate::game_assets::EndGame;
use crate::window::{WindowSize, GameOverWindowBundle};

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
    mut score: ResMut<Score>,
    mut section_requested: ResMut<SnakeSectionsRequested>,
    mut food_query: Query<(&mut Transform, &mut Food), Without<SnakeHead>>,
    mut head_query: Query<(&Transform, &mut SnakePart), With<SnakeHead>>,
) {
    let (mut food_transform, mut food) = food_query.get_single_mut().unwrap();
    let (mut _head_transform, mut head )= head_query.get_single_mut().unwrap();

    if food.bounds.intersects(&head.bounds) {

        head.speed += 0.25;
        let x_rand: f32 = rand::thread_rng().gen_range(-400.0..=400.0);
        let y_rand: f32 = rand::thread_rng().gen_range(-300.0..=300.0);
        food.bounds = Aabb2d::new(Vec2::new(x_rand, y_rand), Vec2::new(food.scale, food.scale));        
        food_transform.translation = Vec3::new(x_rand, y_rand, 0.0);
        score.increment();
        section_requested.requested += 1;
    }
}

pub fn check_game_over(
    mut commands: Commands,
    mut end_game: ResMut<EndGame>,
    score: Res<Score>,
    window: Res<WindowSize>,
    mut head_query: Query<(&Transform, &mut Direction, &mut SnakePart), With<SnakeHead>>,
) {
    let (head_transform, mut direction, head) = head_query.get_single_mut().unwrap();

    // Check if the head is out of bounds of the window

    let border_intersects = |head: &Vec3, offset: f32, win: &WindowSize| -> bool {        
        [
            (head.x + offset / 2.0) >= (win.width / 2.0),
            (head.x - offset / 2.0) <= -(win.width / 2.0),
            (head.y + offset / 2.0) >= (win.height / 2.0),
            (head.y - offset / 2.0) <= -(win.height / 2.0),
        ].into_iter().any(|v| v)
    };

    if border_intersects(&head_transform.translation, head.scale, &window) {
        end_game.is_game_over = true;
        *direction = Direction::Pause;
        commands.spawn(GameOverWindowBundle::init(score.value));
    }

}
