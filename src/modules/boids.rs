#![allow(dead_code)]
use nannou::prelude::*;
use rand::random;

use super::config::*;

#[derive(Clone, Copy)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

pub struct Flock {
    pub boids: Vec<Boid>,
    pub n_lists: Vec<Vec<usize>>,
}

impl Boid {
    fn new() -> Boid {
        Boid {
            x: random::<f32>() * 600.0 - 300.0,
            y: random::<f32>() * 600.0 - 300.0,
            r: random::<f32>() * 2.0 * PI,
        }
    }

    fn rotate(&mut self, rads_target: f32) {
        if rads_target - self.r > PI {
            self.r += 2.0 * PI;
        } else {
            self.r += (rads_target - self.r) * BOID_ROT_SPEED;
        }
    }

    fn translate(&mut self) {
        self.x += BOID_SPEED * self.r.cos();
        self.y += BOID_SPEED * self.r.sin();
    }

    pub fn move_boid(&mut self, rotation_rad: f32) {
        println!("{}", rotation_rad);
        self.rotate(rotation_rad);
        self.translate();
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
        }
    }

    fn calc_distance(&self, boid_1: &Boid, boid_2: &Boid) -> f32 {
        abs(((boid_2.x - boid_1.x).powi(2) + (boid_2.y - boid_1.y).powi(2)).sqrt())
    }

    fn calc_neighbor_groups(&mut self) {
        let mut n_group_list: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.boids.len() {
            let mut n_group: Vec<usize> = vec![i];
            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                }
                if self.calc_distance(self.boids.get(i).unwrap(), self.boids.get(j).unwrap())
                    < BOID_NEI_DIST_THRESHOLD
                {
                    n_group.push(j);
                }
            }
            n_group_list.push(n_group)
        }
        self.n_lists = n_group_list;
    }

    fn calc_rotational_forces(&mut self) -> Vec<f32> {
        let mut force_list: Vec<Vec<f32>> = Vec::new();
        for (boid_ind, sub_vec) in self.n_lists.iter().enumerate() {
            let mut forces: Vec<f32> = Vec::new();
            for boid2_ind in sub_vec.iter() {
                if boid_ind == *boid2_ind {
                    continue
                }
                let dist = self.calc_distance(
                    self.boids.get(boid_ind).unwrap(),
                    self.boids.get(*boid2_ind).unwrap(),
                );
                let weight = 1.0 / dist;
                let force = weight
                    * self.calc_rot_diff(
                        self.boids.get(boid_ind).unwrap().r,
                        self.boids.get(*boid2_ind).unwrap().r,
                    );
                forces.push(force);
            }
            force_list.push(forces);
        }
        let mut result_list: Vec<f32> = Vec::new();
        for list in force_list.iter() {
            let mut average: f32 = 0.0;
            if list.len() == 0 {
                continue
            }
            for force in list.iter() {
                average += force;
            }
            average = average / (list.len() as f32);
            result_list.push(average);
        }
        result_list
    }

    fn apply_rotational_forces(&mut self) {
        let forces = self.calc_rotational_forces();
        for (index, boid) in self.boids.iter_mut().enumerate() {
            if forces.len() == 0 {
                continue
            }
            if index < forces.len() {
                boid.move_boid(*forces.get(index).unwrap());
            }
        }
    }

    fn calc_rot_diff(&self, var1: f32, var2: f32) -> f32 {
        var2 - var1
    }

    pub fn start_flock(&mut self) {
        self.calc_neighbor_groups();
        self.apply_rotational_forces();
    }
}
