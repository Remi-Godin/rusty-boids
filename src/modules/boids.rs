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
        let mut new_r = self.r + (rads_target * BOID_ROT_SPEED);
        if new_r >= 2.0 * PI {
            new_r -= 2.0 * PI
        } else if new_r < 0.0 {
            new_r += 2.0 * PI
        }
        self.r = new_r;
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
}
