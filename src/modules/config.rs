#![allow(dead_code)]
// nannou config
pub const WINDOW_W: u32 = 1000;
pub const WINDOW_H: u32 = 1000;
pub const UPDATE_RATE: f64 = 60.0;

// Boids parameters
pub const BOID_COUNT: usize = 50;
pub const BOID_SPAWN_DIST: f32 = 400.0;
pub const BOID_SIZE: f32 = 10.0;
pub const BOID_SPEED: f32 = 0.5;
pub const BOID_ROT_SPEED: f32 = 0.02;
pub const BOID_NEI_DIST_THRESHOLD: f32 = 200.0;
pub const BOID_GROUP_DIST: f32 = 500.0;
pub const BOID_SEPARATION_DIST_TARGET: f32 = BOID_SIZE * 0.1;
pub const BOID_COHESION_WEIGHT: f32 = 0.0;
pub const BOID_GROUPING_WEIGHT: f32 = 0.9;
