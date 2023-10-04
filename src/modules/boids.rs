#![allow(unused)]
#![allow(dead_code)]
use nannou::{glam::Vec3Swizzles, prelude::*};
use rand::random;

use super::config::*;

pub struct ForceVector {
    pub magnitude: f32,
    pub angle: f32,
}

#[derive(Clone, Copy)]
pub struct XYCoord {
    pub x: f32,
    pub y: f32,
}

impl XYCoord {
    pub fn new(x: f32, y: f32) -> XYCoord {
        XYCoord { x: x, y: y }
    }
}

#[derive(Clone, Copy)]
pub struct Boid {
    pub coord: XYCoord,
    pub r: f32,
}

pub struct Flock {
    pub boids: Vec<Boid>,
    pub center_of_flock_list: Vec<XYCoord>,
    pub cohesion_forces: Vec<ForceVector>,
    pub n_lists: Vec<Vec<usize>>,
    pub f_list: Vec<f32>,
}

impl Boid {
    fn new() -> Boid {
        let x = random::<f32>() * BOID_SPAWN_DIST - (BOID_SPAWN_DIST / 2.0);
        let y = random::<f32>() * BOID_SPAWN_DIST - (BOID_SPAWN_DIST / 2.0);
        Boid {
            coord: XYCoord::new(x, y),
            r: random::<f32>() * 2.0 * PI,
        }
    }

    fn rotate_old(&mut self, rads_target: &f32) {
        if rads_target - self.r > PI {
            self.r += 2.0 * PI;
        } else {
            self.r += (rads_target - self.r) * BOID_ROT_SPEED;
        }
    }

    fn rotate(&mut self, rads_target: &f32) {
        while self.r > (2.*PI) {
            self.r -= (2.*PI);
        }
        while self.r < 0. {
            self.r += (2.*PI);
        }
        self.r += BOID_ROT_SPEED * rads_target;
        // WHY ARE THEY TURNING LEFT ALL THE TIME
        // When they encounter a group which has a center that is to the right relatively to
        // the boid, it will turn left and do a loop instead. I need to fix this.
    }

    fn translate(&mut self) {
        self.coord.x += BOID_SPEED * self.r.cos();
        self.coord.y += BOID_SPEED * self.r.sin();
    }

    fn wrap_around(&mut self) {
        if abs(self.coord.x) > ((WINDOW_W / 2) as f32) {
            self.coord.x *= -1.0;
        }
        if abs(self.coord.y) > ((WINDOW_H / 2) as f32) {
            self.coord.y *= -1.0;
        }
    }

    pub fn move_boid(&mut self, rotation_rad: &f32) {
        self.rotate(rotation_rad);
        self.translate();
        self.wrap_around();
    }
}

impl Flock {
    pub fn new(amount: usize) -> Flock {
        let mut flock: Vec<Boid> = Vec::new();
        for _i in 1..(amount + 1) {
            flock.push(Boid::new());
        }
        Flock {
            boids: flock,
            n_lists: Vec::new(),
            center_of_flock_list: Vec::new(),
            f_list: Vec::new(),
            cohesion_forces: Vec::new(),
        }
    }

    fn calc_neighbor_groups(&mut self) {
        let mut n_group_list: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.boids.len() {
            let mut n_group: Vec<usize> = vec![i];
            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                }
                if calc_distance(
                    &self.boids.get(i).unwrap().coord,
                    &self.boids.get(j).unwrap().coord,
                ) < BOID_NEI_DIST_THRESHOLD
                {
                    n_group.push(j);
                }
            }
            n_group_list.push(n_group)
        }
        self.n_lists = n_group_list;
    }

    fn generate_center_of_flock_list(&mut self) {
        let mut com_list: Vec<XYCoord> = Vec::new();
        for n_list in self.n_lists.iter() {
            let mut total_x = 0.0;
            let mut total_y = 0.0;
            for boid_id in n_list.iter() {
                total_x += &self.boids.get(*boid_id).unwrap().coord.x;
                total_y += &self.boids.get(*boid_id).unwrap().coord.y;
            }
            let coord = XYCoord {
                x: total_x / (n_list.len() as f32),
                y: total_y / (n_list.len() as f32),
            };
            com_list.push(coord);
        }
        self.center_of_flock_list = com_list;
    }

    fn generate_force_list(&mut self) {
        // This will generate the list of forces and ensure that there
        // is one vec per boid.
        // This function will call the calc_*_forces functions on a per
        // case basis, by passing the current boid and a neighbor.
        // Step 1: Create new Vec
        // Step 2: Check if the coresponding neighbor list has more than 1 item
        // Step 3: If yes, call the calc_*_forces functions.
        // Step 4: If not, the force will be zero.
        let mut force_list: Vec<ForceVector> = Vec::new();
        for boid_index in 0..self.boids.len() {
            let cohesion_force = calc_cohesion_forces(
                self.boids.get(boid_index).unwrap(),
                self.center_of_flock_list.get(boid_index).unwrap(),
            );
            force_list.push(cohesion_force);
        }
        self.cohesion_forces = force_list;
    }

    pub fn start_flock(&mut self) {
        self.calc_neighbor_groups();
        self.generate_center_of_flock_list();
        self.generate_force_list();
        for boid_index in 0..self.boids.len() {
            let boid = self.boids.get_mut(boid_index).unwrap();
            let angle = self.cohesion_forces.get(boid_index).unwrap();
            boid.move_boid(&angle.angle);
        }
    }
}

// THINGS TO DO
// - Create a fixed point target for testing
//

pub fn get_angle_to_target(boid_1: &Boid, target: &XYCoord) -> f32 {
    let delta_x = target.x - boid_1.coord.x;
    let delta_y = target.y - boid_1.coord.y;
    let mut angle_to_target: f32 = 0.0;
    if delta_x != 0.0 {
        angle_to_target = (delta_y).atan2(delta_x) - boid_1.r;
    }
    if angle_to_target > PI {
        angle_to_target -= 2.0 * PI;
    } else if angle_to_target < -PI {
        angle_to_target += 2.0 * PI;
    }
    angle_to_target
}

fn calc_cohesion_forces(boid: &Boid, target: &XYCoord) -> ForceVector {
    // Boids will tend to flock towards the center of their neighbors
    let force_vec = ForceVector {
        magnitude: calc_distance(&boid.coord, &target),
        angle: get_angle_to_target(&boid, &target),
    };
    force_vec
}

fn calc_distance(origin: &XYCoord, target: &XYCoord) -> f32 {
    abs(((target.x - origin.x).powi(2) + (target.y - origin.y).powi(2)).sqrt())
}

fn calc_separation_forces() {
    // Boids will try to not bump into one another
}

fn calc_allignment_forces() {
    // Boids will fly in the same direction as nearby boids
}

fn normalize_vector(vector: &ForceVector) -> ForceVector {
    // Sets the vector length to 1. Useful to apply force bias for better
    // control of the final vectors. Ultimately, the magniture does not
    // matter after the force calculations since the speed is fixed.
    ForceVector {
        magnitude: 1.0,
        angle: vector.angle,
    }
}

fn combine_force_vectors(cohesion: Vec<f32>, separation: Vec<f32>, allignment: Vec<f32>) {}
