use bevy::{color::Color, math::bounding::Aabb2d, prelude::*};
use crate::events::FoodRequested;
use crate::window::WindowSize;
use rand::Rng;
use crate::game_assets::Callback;

const FOOD_COLOR: Color = Color::srgb(6.0, 1.2, 3.6);  // Neon pink (hex: #FF3399
const FOOD_SCALE: f32 = 25.0;

#[derive(Component, Debug, Reflect)]
pub struct Food {
    pub color: Color,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    pub bounds: Aabb2d,
}

impl Food {
    pub fn new(x_range: f32, y_range: f32) -> Self {
        let x_rand: f32 = rand::thread_rng().gen_range(-x_range/2.0 - 100.0..=x_range/2.0 + 100.0);
        let y_rand: f32 = rand::thread_rng().gen_range(-y_range/2.0 - 100.0..=y_range/2.0 + 100.0);
        let scale = FOOD_SCALE;
        
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
        app.add_systems(Startup, (spawn_food_system, spawn_food))
        .init_resource::<FoodRequested>();
    }
}

pub fn spawn_food(
    mut commands: Commands,
    window: Res<WindowSize>
) {
        let food = Food::new(window.width, window.height);
        commands.spawn((
            Sprite::from_color(food.color, Vec2::new(food.scale, food.scale)),
            Transform {
                translation: Vec3::new(food.x, food.y, -1.0),
                ..default()
            },
         food
    ));
        
}


#[derive(Component)]
pub struct FoodRequest;

pub fn spawn_food_system(
    mut commands: Commands,
){
    let system_id = commands.register_system(spawn_food);
    commands.spawn((Callback(system_id), FoodRequest));
}
