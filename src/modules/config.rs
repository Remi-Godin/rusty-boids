// nannou config
pub const WINDOW_W: u32 = 1800;
pub const WINDOW_H: u32 = 1200;
pub const UPDATE_RATE: f64 = 60.0;

// Visualization parameters
pub const DISPLAY_COHESION_VECTORS: bool = false;
pub const DISPLAY_ALLIGNMENT_VECTORS: bool = false;
pub const DISPLAY_SEPARATRION_VECTORS: bool = false;
pub const DISPLAY_FINAL_VECTORS: bool = false;
pub const DISPLAY_CENTER_OF_GROUPS: bool = false;

// Boids parameters
pub const BOID_COUNT: usize = 500;
pub const BOID_SPAWN_DIST: f32 = 800.0;
pub const BOID_SIZE: f32 = 10.0;
pub const BOID_SPEED: f32 = 2.8;
pub const BOID_ROT_SPEED: f32 = 0.04;
pub const BOID_NEARBY_DIST_THRESHOLD: f32 = 100.0;
pub const BOID_SEPARATION_DIST_TARGET: f32 = 15.0;
pub const BOID_COHESION_WEIGHT: f32 = 1.0;
pub const BOID_SEPARATION_WEIGHT: f32 = 1.0;
pub const BOID_ALLIGNMENT_WEIGHT: f32 = 1.0;
pub const BOID_ROT_THRESHOLD: f32 = 0.0001;
