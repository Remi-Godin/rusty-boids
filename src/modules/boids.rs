#![allow(unused)]
#![allow(dead_code)]
use super::config::*;
use nannou::prelude::*;
use rand::random;

pub struct ForceVector {
    pub magnitude: f32,
    pub angle: f32,
}

pub struct Boid {
    pub coord: Vec2,
    pub angle: f32,
}

pub struct Flock {
    pub boids: Vec<Boid>,
    pub nearby: Vec<Vec<usize>>,
    pub centers_of_flock: Vec<Vec2>,
    pub cohesion: Vec<ForceVector>,
    pub separation: Vec<ForceVector>,
    pub allignment: Vec<ForceVector>,
}

impl ForceVector {
    pub fn new(magnitude: f32, angle: f32) -> ForceVector {
        ForceVector {
            magnitude,
            angle
        }
    }

    pub fn zero() -> ForceVector {
        ForceVector {
            magnitude: 0.0,
            angle: 0.0,
        }
    }
}

impl Boid {
    fn new() -> Boid {
        let x = random_f32() * BOID_SPAWN_DIST - BOID_SPAWN_DIST / 2.0;
        let y = random_f32() * BOID_SPAWN_DIST - BOID_SPAWN_DIST / 2.0;
        Boid {
            coord: Vec2::new(x, y),
            angle: random_f32() * 2.0 * PI,
        }
    }

    fn translate(&mut self) {
        let target = Vec2::new(self.angle.cos(), self.angle.sin());
        self.coord += target * BOID_SPEED;
    }

    fn rotate(&mut self, target_angle: &f32) {
        let angle_diff = self.angle - target_angle;
        let mut adjustment = -angle_diff;
        if angle_diff > PI {
            adjustment = 2.0 * PI - angle_diff;
        }
        if angle_diff < -PI {
            adjustment -= 2.0 * PI;
        }
        if abs(adjustment) < BOID_ROT_THRESHOLD {
            self.angle = *target_angle;
            return
        }
        while self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
        while self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        self.angle += adjustment * BOID_ROT_SPEED;
    }

    fn move_boid(&mut self, target_angle: &f32) {
        self.translate();
        self.rotate(target_angle);
    }
}

impl Flock {
    pub fn new(amount: usize) -> Flock {
        let mut boids: Vec<Boid> = Vec::new();
        for i in 0..amount {
            boids.push(Boid::new());
        }
        Flock {
            boids,
            nearby: Vec::new(),
            cohesion: Vec::new(),
            separation: Vec::new(), 
            allignment: Vec::new(),
            centers_of_flock: Vec::new()
        }
    }

    pub fn start_flock(&mut self) {
        self.nearby = generate_nearby_lists(&self);
        self.centers_of_flock = generate_center_of_flock_list(&self);
        for i in self.boids.iter_mut() {
            // THE LITERAL ANGLE VALUE NEEDS TO BE CHANGED FOR A VAR THAT WILL
            // BE CALCULATED FROM THE COMBINATION OF ALL FORCES. IF ALL EXTERNAL
            // FORCES ADD UP TO A ZERO MAGNITUDE VECTOR, THEN THE BOID SHOULD JUST
            // CONTINUE ON ITS WAY. 
            //
            // I MIGHT NEED TO GET THE CURRENT BOID INSIDE THE FORCE CALCULATION
            // TO RETURN IT'S ANGLE AS A RESULT IN THE CASE OF ZERO MAGNITUDE VECTOR
            // ADDITION.
            i.move_boid(&6.0);
        }
    }
}

fn generate_nearby_lists(flock: &Flock) -> Vec<Vec<usize>> {
    let mut list_of_nearby_list: Vec<Vec<usize>> = Vec::new();
    for first_boid_index in 0..flock.boids.len() {
        let mut nearby_list: Vec<usize> = Vec::new();
        let boid_1 = flock.boids.get(first_boid_index).unwrap();
        for second_boid_index in 0..flock.boids.len() {
            let boid_2 = flock.boids.get(second_boid_index).unwrap();
            if boid_1.coord.distance(boid_2.coord) < BOID_NEARBY_DIST_THRESHOLD {
                nearby_list.push(second_boid_index);
            }
        }
        list_of_nearby_list.push(nearby_list);
    }
    list_of_nearby_list
}

fn generate_center_of_flock_list(flock: &Flock) -> Vec<Vec2> {
    let mut list_of_vec2: Vec<Vec2> = Vec::new();
    for list_index in 0..flock.nearby.len() {
        let mut center_of_flock = Vec2::new(0.0, 0.0);
        for boid_index in flock.nearby.get(list_index).unwrap().iter() {
            center_of_flock += flock.boids.get(*boid_index).unwrap().coord;
        }
        list_of_vec2.push(center_of_flock / (flock.nearby.get(list_index).unwrap().len() as f32));
        // SEEMS TO BE WORKING
        // NEED TO MAKE SURE THAT THE BOIDS CAN FLY ON THEIR OWN
        // SO I NEED TO INTEGRATE THE COMBINE FORCES FUNCTION
        // SOON SO THAT I CAN HAVE A DEFAULT VECTOR AND IGNORE ZERO
        // MAGNITUDE VECTORS
    }
    list_of_vec2
}
