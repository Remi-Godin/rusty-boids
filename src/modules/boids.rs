#![allow(unused)]
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
    pub f_list: Vec<f32>,
}

impl Boid {
    fn new() -> Boid {
        Boid {
            x: random::<f32>() * 200.0 - 100.0,
            y: random::<f32>() * 200.0 - 100.0,
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
        self.r += rads_target * BOID_ROT_SPEED;
    }

    fn translate(&mut self) {
        self.x += BOID_SPEED * self.r.cos();
        self.y += BOID_SPEED * self.r.sin();
    }

    pub fn move_boid(&mut self, rotation_rad: &f32) {
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
            f_list: Vec::new(),
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

    fn calc_cohesion_forces(&mut self) -> Vec<f32> {
        // Forces that moves the boids in the same direction
        let mut force_list: Vec<f32> = Vec::new();
        let mut forces: Vec<f32> = Vec::new();
        for (index, _) in self.boids.iter().enumerate() {
            let n_list = self.n_lists.get(index).unwrap();
            if n_list.len() < 2 {
                force_list.push(0.0);
                continue
            }
            for neighbor_index in n_list.iter().skip(1) {
                let target_boid = self.boids.get(*n_list.get(0).unwrap()).unwrap();
                let neighbor_boid = self.boids.get(*neighbor_index).unwrap();
                let dist = self.calc_distance(target_boid, neighbor_boid);
                let weight = (BOID_NEI_DIST_THRESHOLD - dist) / BOID_NEI_DIST_THRESHOLD;
                let force = 2.0 * (neighbor_boid.r - target_boid.r) * weight;
                forces.push(force);
            }
            let mut average: f32 = 0.0;
            for i in forces.iter() {
                average += i;
            }
            average = average / (forces.len() as f32);
            force_list.push(average);
        }
        force_list
    }

    pub fn start_flock(&mut self) {
        self.calc_neighbor_groups();
        let force_list = self.calc_cohesion_forces();
        for (index, boid) in self.boids.iter_mut().enumerate() {
            boid.move_boid(force_list.get(index).unwrap());
        }
    }
}
