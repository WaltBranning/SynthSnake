use bevy::{
    color::Color,
    prelude::*,
    math::bounding::Aabb2d,
    math::bounding::BoundingVolume,
};

const SNAKE_HEAD_COLOR: Color = Color::srgb(3.0, 6.0, 3.0);
const SNAKE_BODY_COLOR: Color = Color::srgb(0.75, 0.75, 6.0);
const SNAKE_SCALE: f32 = 25.0;
const SNAKE_SPEED: f32 = 5.5;

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Pause,
}

#[derive(Resource, Debug)]
pub struct SnakeSectionsRequested {
    pub count: u8,
    pub requested: u8,
}

impl Default for SnakeSectionsRequested {
    fn default() -> Self {
        SnakeSectionsRequested { count: 1, requested: 1 }
    }
}

#[derive(Component, Debug)]
pub struct SnakePart {
    pub speed: f32,
    pub scale: f32, 
    pub section_index: usize,
    pub bounds: Aabb2d,
}

impl Default for SnakePart {
    fn default() -> Self {
        SnakePart { speed: SNAKE_SPEED, scale: SNAKE_SCALE, section_index: 0, bounds: Aabb2d::new(Vec2::new(0.0, 0.0), Vec2::new(SNAKE_SCALE, SNAKE_SCALE)) }
    }
}

#[derive(Component)]
pub struct SnakeHead;

pub fn spawn_snake(mut commands: Commands) {
    let snake_part = SnakePart::default();
    commands
        .spawn((
            // Collider::cuboid(snake_part.scale, snake_part.scale),
            SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(snake_part.scale, snake_part.scale, 0.0),
                ..default()
            },
            ..default()
        }, Direction::Pause, snake_part))
        // .insert(Sensor)
        // .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SnakeHead);
}

pub fn spawn_snake_section(
    mut commands: Commands, 
    mut sections: ResMut<SnakeSectionsRequested>,
    position: Query<(&mut Transform, &SnakePart)>, 
){
    
    if sections.count < sections.requested {

        let mut last_section_index: usize = 0;
        let mut penultimate_section_index: usize = 0;
        let mut last_location: Transform = Transform::default();
        let mut penultimate_location: Transform = Transform::default();

        for (transform, snake_part) in &position {
            println!("snake_part speed = {:?}", snake_part.speed);
            if last_section_index < snake_part.section_index {
                last_section_index = snake_part.section_index;
                penultimate_location = last_location;
                last_location = *transform;
            } else if last_section_index == snake_part.section_index {
                last_location = transform.to_owned();
            }

            if (penultimate_section_index < snake_part.section_index) && !(snake_part.section_index <= 0) {
                penultimate_section_index = snake_part.section_index - 1;
            }

        }

        let new_snake_part = SnakePart{
            section_index: last_section_index + 1,
            ..default()
        };
        // println!("last_section_index = {:?} penultimate_section_index = {:?}", last_section_index, penultimate_section_index);

        println!("last_location = {:?}", last_location);
        println!("penultimate_location = {:?}", penultimate_location);

        commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                color: SNAKE_BODY_COLOR,
                ..default()
            },
            transform: Transform {
                translation: last_location.translation - last_location.rotation.mul_vec3(Vec3::new(0.0, new_snake_part.scale, 0.0)),
                rotation: last_location.rotation,
                scale: Vec3::new(new_snake_part.scale, new_snake_part.scale, 1.0),
            },
            ..default()
        }, new_snake_part));
        sections.count = sections.requested;
    }
}

pub fn move_snake(
    mut head_query: Query<(&Direction, &mut Transform, &mut SnakePart), With<SnakeHead>>,
    mut body_query: Query<(&mut Transform, &mut SnakePart), Without<SnakeHead>>,
) {
    // First move the head
    if let Ok((direction, mut head_transform, mut snake_part)) = head_query.get_single_mut() {
        // println!("snake_part = {:?}", snake_part.bounds);
        match *direction {
            Direction::Up => {
                let translation = Vec2::new(0.0, snake_part.speed);
                head_transform.translation.y += translation.y;
                head_transform.rotation = Quat::from_rotation_z(0.0);
                snake_part.bounds.translate_by(translation);
            },
            Direction::Down => {
                let translation = Vec2::new(0.0, -snake_part.speed);
                head_transform.translation.y += translation.y;
                
                head_transform.rotation = Quat::from_rotation_z(std::f32::consts::PI);
                snake_part.bounds.translate_by(translation);
            },
            Direction::Left => {
                let translation = Vec2::new(-snake_part.speed, 0.0);
                head_transform.translation.x += translation.x;
                head_transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
                snake_part.bounds.translate_by(translation);
            },
            Direction::Right => {
                let translation = Vec2::new(snake_part.speed, 0.0);
                head_transform.translation.x += translation.x;
                head_transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2);
                snake_part.bounds.translate_by(translation);
            },
            Direction::Pause => (),
        }

        // Sort body parts by section index to ensure proper following order
        let mut body_parts: Vec<_> = body_query.iter_mut().collect();
        body_parts.sort_by_key(|(_, part)| part.section_index);

        let mut previous_pos = head_transform.translation;
        let mut previous_rot = head_transform.rotation;

        // Update each body segment to follow the one in front of it
        for (mut transform, snake_part) in body_query.iter_mut() {
            let current_pos = transform.translation;
            let current_rot = transform.rotation;
            
            // Calculate target position based on the previous segment
            let direction = (previous_pos - current_pos).normalize_or(Vec3::Y);
            let target_pos = previous_pos - direction * snake_part.scale;

            // Smoothly move towards target position
            transform.translation = target_pos;
            transform.rotation = previous_rot;

            // Store current position for next segment
            previous_pos = current_pos;
            previous_rot = current_rot;
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        
       app.add_systems(Startup, (
            spawn_snake,
        ))
       .add_systems(Update, (
            move_snake, 
            spawn_snake_section
        ))
        .init_resource::<SnakeSectionsRequested>();
    }
}
