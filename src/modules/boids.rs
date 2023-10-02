#![allow(dead_code)]
use nannou::prelude::*;
use rand::random;

use super::config::*;

#[derive(Clone)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

pub struct Flock {
    pub boids: Vec<Boid>,
}

impl Boid {
    fn new() -> Boid {
        Boid {
            x: random::<f32>() * 50.0,
            y: random::<f32>() * 50.0,
            r: random::<f32>() * 2.0 * PI,
        }
    }

    fn rotate(&mut self, rads_target: f32) {
        if rads_target - self.r > PI {
            self.r += 2.0 * PI;
        } else {
            self.r += (rads_target - self.r)*BOID_ROT_SPEED;
        }
    }

    fn translate(&mut self) {
        self.x += BOID_SPEED * self.r.cos();
        self.y += BOID_SPEED * self.r.sin();
    }

    pub fn move_boid(&mut self, rotation_rad: f32) {
        self.rotate(rotation_rad);
        self.translate();
    }

}

impl Flock {
    pub fn new(amount: usize) -> Flock {
        let mut flock: Vec<Boid> = Vec::new();
        for _i in 1..amount {
            flock.push(Boid::new());
        }
        Flock { boids: flock }
    }
    
    fn calc_distance(&self, boid_1: &Boid, boid_2: &Boid) -> f32 {
        abs(((boid_2.x - boid_1.x).powi(2) + (boid_2.y - boid_1.y).powi(2)).sqrt())
    }

}
