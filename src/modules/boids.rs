#![allow(unused)]
#![allow(dead_code)]
use nannou::{glam::Vec3Swizzles, prelude::*};
use rand::random;

use super::config::*;

pub struct ForceVector {
    pub magnitude: f32,
    pub angle: f32,
}

impl ForceVector {
    pub fn new(magnitude: f32, angle: f32) -> ForceVector {
        ForceVector {
            magnitude: magnitude,
            angle: angle,
        }
    }
    pub fn zero() -> ForceVector {
        ForceVector {
            magnitude: 0.0,
            angle: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Boid {
    pub coord: Vec2,
    pub r: f32,
}

pub struct Flock {
    pub boids: Vec<Boid>,
    pub center_of_flock_list: Vec<Vec2>,
    pub cohesion_forces: Vec<ForceVector>,
    pub separation_forces: Vec<ForceVector>,
    pub allignment_forces: Vec<ForceVector>,
    pub normal_forces: Vec<ForceVector>,
    pub n_lists: Vec<Vec<usize>>,
    pub f_list: Vec<f32>,
}

impl Boid {
    fn new() -> Boid {
        let x = random::<f32>() * BOID_SPAWN_DIST - (BOID_SPAWN_DIST / 2.0);
        let y = random::<f32>() * BOID_SPAWN_DIST - (BOID_SPAWN_DIST / 2.0);
        Boid {
            coord: Vec2::new(x, y),
            r: random::<f32>() * 2.0 * PI,
        }
    }

    fn rotate(&mut self, rads_target: &f32) {
        while self.r > (2. * PI) {
            self.r -= (2. * PI);
        }
        while self.r < 0. {
            self.r += (2. * PI);
        }
        let angle_diff = self.r - rads_target;
        self.r -= angle_diff * BOID_ROT_SPEED;
        // THE BOIDS SEEM TO HAVE AN ISSUE ROTATING TOWARDS THE TARGET WHEN IT IS BEHIND THEM, I
        // NEED TO FIGURE OUT WHY...
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
            separation_forces: Vec::new(),
            allignment_forces: Vec::new(),
            normal_forces: Vec::new()
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
        let mut com_list: Vec<Vec2> = Vec::new();
        for n_list in self.n_lists.iter() {
            let mut total_x = 0.0;
            let mut total_y = 0.0;
            for boid_id in n_list.iter() {
                total_x += &self.boids.get(*boid_id).unwrap().coord.x;
                total_y += &self.boids.get(*boid_id).unwrap().coord.y;
            }
            let coord = Vec2::new(
                total_x / (n_list.len() as f32),
                total_y / (n_list.len() as f32),
            );
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
        let mut cohesion_force_list: Vec<ForceVector> = Vec::new();
        let mut separation_force_list: Vec<ForceVector> = Vec::new();
        for boid_index in 0..self.boids.len() {
            // Normal Forces
            let normal_force = calc_normal_forces(&boid_index, &self);
            self.normal_forces.push(normal_force);
            // Cohesion Forces
            let cohesion_force = calc_cohesion_forces(
                self.boids.get(boid_index).unwrap(),
                self.center_of_flock_list.get(boid_index).unwrap(),
            );
            cohesion_force_list.push(cohesion_force);
            // Separation Forces
            let separation_force = calc_separation_forces(&boid_index, &self);
            separation_force_list.push(separation_force);
        }
        self.cohesion_forces = cohesion_force_list;
    }

    pub fn start_flock(&mut self) {
        self.calc_neighbor_groups();
        self.generate_center_of_flock_list();
        self.generate_force_list();
        let forces = combine_force_vectors(&self.normal_forces, &self.cohesion_forces);
        for boid_index in 0..self.boids.len() {
            let boid = self.boids.get_mut(boid_index).unwrap();
            let angle = forces.get(boid_index).unwrap();
            boid.move_boid(angle);
        }
    }
}

// THINGS TO DO
// - Create a fixed point target for testing
//

pub fn get_angle_of_target(boid_1: &Vec2, target: &Vec2) -> f32 {
    let delta_x = target.x - boid_1.x;
    let delta_y = target.y - boid_1.y;
    let mut angle_to_target: f32 = 0.0;
    if delta_x != 0.0 {
        angle_to_target = (delta_y).atan2(delta_x);
        // IF I REMOVE THE - boid_1.r I THINK I GET THE ABSOLUTE ANGLE
        // WHICH I NEED IF I WANT TO REWORK MY ROTATION FUNCTION
        // TO THE WAY I WANT IT TO WORK
    }
    if angle_to_target < 0. {
        angle_to_target += 2.* PI;
    }
    println!("{angle_to_target}");
    angle_to_target
}

fn calc_cohesion_forces(boid: &Boid, target: &Vec2) -> ForceVector {
    // Boids will tend to flock towards the center of their neighbors
    let force_vec = ForceVector {
        magnitude: calc_distance(&boid.coord, &target),
        angle: get_angle_of_target(&boid.coord, &target),
    };
    force_vec
    // MAYBE I NEED TO CONSIDER THE MAGNITUDE
    // Perhaps proportional to distance? The closer they are to the flock, the more they will
    // want to converge to the center of mass?
}

fn calc_distance(origin: &Vec2, target: &Vec2) -> f32 {
    abs(((target.x - origin.x).powi(2) + (target.y - origin.y).powi(2)).sqrt())
}

fn calc_separation_forces(boid_index: &usize, flock: &Flock) -> ForceVector {
    // Boids will try to not bump into one another
    // Inversly proportional to distance (The closer they are, the bigger the force)
    // Will try to maintain a specific distance to target so the forces
    // will only be active if withing the boundary of the target.
    // Naturally, forces should cancel out, but as opposed to the cohesion forces,
    // I need to calculate the forces for each boid and then average out the force vector.
    // I will need to make sure that the force vector is applied as a "mirror" so that the
    // vector "bounces" of the boids
    let curr_boid = flock.boids.get(*boid_index).unwrap();
    let mut vector_list: Vec<ForceVector> = Vec::new();
    let curr_list = flock.n_lists.get(*boid_index).unwrap();
    if curr_list.len() < 2 {
        return ForceVector::zero();
    } else {
        for boid_i in curr_list.iter() {
            let target_boid = flock.boids.get(*boid_i).unwrap();
            vector_list.push(ForceVector::new(
                calc_distance(&curr_boid.coord, &target_boid.coord),
                get_angle_of_target(&curr_boid.coord, &target_boid.coord),
            ));
        }
    }
    ForceVector::zero()
}

fn calc_allignment_forces() {
    // Boids will fly in the same direction as nearby boids
}

fn calc_normal_forces(boid_index: &usize, flock: &Flock) -> ForceVector {
    ForceVector::new(0.01, flock.boids.get(*boid_index).unwrap().r)
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

fn combine_force_vectors(normal: &Vec<ForceVector>, cohesion: &Vec<ForceVector>) -> Vec<f32> { //separation: Vec<ForceVector>, allignment: Vec<ForceVector>) -> Vec<f32> {
    let mut results: Vec<f32> = Vec::new();
    for i in 0..cohesion.len() {
        let cohesion_vector = cohesion.get(i).unwrap();
        let normal_vector = normal.get(i).unwrap();
        let result_x = cohesion_vector.angle.cos()*cohesion_vector.magnitude +
            normal_vector.angle.cos()*normal_vector.magnitude;
        let result_y = cohesion_vector.angle.sin()*cohesion_vector.magnitude +
            normal_vector.angle.sin()*normal_vector.magnitude;
        let result_r = get_angle_of_target(&Vec2::new(0.0,0.0), &Vec2::new(result_x, result_y));
        results.push(result_r);
        // Need to add polar vectors together
        // Also need to figure out how to get a normal pointing vector to prevent zero force
        // vectors from affecting the boid in bad ways.
    }
    results
}
