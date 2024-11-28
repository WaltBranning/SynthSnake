use bevy::{color::Color, math::bounding::Aabb2d, prelude::*};

use bevy_inspector_egui::prelude::*;
use crate::events::FoodRequested;
use crate::window::WindowSize;
use rand::Rng;

const FOOD_COLOR: Color = Color::srgb(6.0, 1.2, 3.6);  // Neon pink (hex: #FF3399
const FOOD_SCALE: f32 = 25.0;

#[derive(Component, Debug, InspectorOptions, Reflect)]
pub struct Food {
    pub color: Color,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    pub bounds: Aabb2d,
}

impl Food {
    pub fn new(x_range: f32, y_range: f32) -> Self {
        let x_rand: f32 = rand::thread_rng().gen_range(-x_range/2.0..=x_range/2.0);
        let y_rand: f32 = rand::thread_rng().gen_range(-y_range/2.0..=y_range/2.0);
        let scale = FOOD_SCALE;
        println!("x_rand: {}, y_rand: {}", x_rand, y_rand);
        Food { 
            x: x_rand,
            y: y_rand,
            scale: scale,
            bounds: Aabb2d::new(Vec2::new(x_rand, y_rand), Vec2::new(scale, scale)),
            ..default()
        }
    }
}

impl Default for Food {
    fn default() -> Self {
        let x_rand: f32 = rand::thread_rng().gen_range(-400.0..=400.0);
        let y_rand: f32 = rand::thread_rng().gen_range(-300.0..=300.0);
        let scale = FOOD_SCALE;
        Food { 
            color: FOOD_COLOR, 
            scale: scale, 
            x: x_rand, 
            y: y_rand,
            bounds: Aabb2d::new(Vec2::new(x_rand, y_rand), Vec2::new(scale, scale)),
        }
    }
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food)
        .init_resource::<FoodRequested>();
    }
}

pub fn spawn_food(
    mut commands: Commands, 
    mut food_request: ResMut<FoodRequested>, 
    window: Res<WindowSize>
) {

    if food_request.requested {
        let food = Food::new(window.width, window.height);
        commands.spawn((
            SpriteBundle {
            sprite: Sprite { color: food.color, ..default() },
            transform: Transform {
                translation: Vec3::new(food.x, food.y, -1.0),
                scale: Vec3::new(food.scale, food.scale, 0.0),
                ..default()
            },
            ..default()
        }, food
    ));
        food_request.requested = false;
    }
}
