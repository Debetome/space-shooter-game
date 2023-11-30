use bevy::ecs::component::Component;
use bevy::time::Timer;

#[derive(Component)]
pub struct SpaceShip;

#[derive(Component)]
pub struct Pumper;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Foe;

#[derive(Component)]
pub struct FoeProjectile;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct ShootingDelay(pub Timer);

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
#[allow(dead_code)]
pub struct GDirection {
    pub x: f32,
    pub y: f32
}

#[allow(dead_code)]
impl GDirection {
    pub fn new(x: f32, y: f32) -> Self { 
        Self { x, y } 
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[allow(dead_code)]
impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct GameBanner;