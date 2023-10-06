use super::config::*;
use nannou::prelude::*;

#[derive(Clone, Debug)]
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
    pub result_forces: Vec<ForceVector>
}

impl ForceVector {
    pub fn new(magnitude: f32, angle: f32) -> ForceVector {
        ForceVector { magnitude, angle }
    }

    pub fn zero() -> Self {
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
        self.wrap_around();
        let target = Vec2::new(self.angle.cos(), self.angle.sin());
        self.coord += target * BOID_SPEED;
    }

    fn wrap_around(&mut self) {
        if abs(self.coord.x) > ((WINDOW_W / 2) as f32) {
            self.coord.x *= -1.;
        }
        if abs(self.coord.y) > ((WINDOW_H / 2) as f32) {
            self.coord.y *= -1.;
        }
    }

    fn rotate(&mut self, force_vector: &ForceVector) {
        if force_vector.magnitude < 0.0001 {
            return
        }
        let angle_diff = self.angle - force_vector.angle;
        let mut adjustment = -angle_diff;

        if angle_diff > PI {
            adjustment = 2.0 * PI - angle_diff;
        }
        if angle_diff < -PI {
            adjustment -= 2.0 * PI;
        }

        if abs(adjustment) < BOID_ROT_THRESHOLD {
            self.angle = force_vector.angle;
            return;
        }
        while self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
        while self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        self.angle += adjustment * BOID_ROT_SPEED;
    }

    fn move_boid(&mut self, force_vector: &ForceVector) {
        self.translate();
        self.rotate(force_vector);
    }
}

impl Flock {
    pub fn new(amount: usize) -> Flock {
        let mut boids: Vec<Boid> = Vec::new();
        for _i in 0..amount {
            boids.push(Boid::new());
        }
        Flock {
            boids,
            nearby: Vec::new(),
            cohesion: Vec::new(),
            separation: Vec::new(),
            allignment: Vec::new(),
            centers_of_flock: Vec::new(),
            result_forces: Vec::new()
        }
    }

    pub fn start_flock(&mut self) {
        self.nearby = generate_nearby_lists(&self);
        self.centers_of_flock = generate_center_of_flock_list(&self);
        self.cohesion = calc_cohesion_forces(&self);
        self.allignment = calc_allignment_forces(&self);
        self.separation = calc_separation_forces(&self);
        self.result_forces = combine_force_vectors(&self);
        for index in 0..self.boids.len() {
            self.boids
                .get_mut(index)
                .unwrap()
                .move_boid(&self.result_forces.get(index).unwrap());
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
    }
    list_of_vec2
}

fn calc_cohesion_forces(flock: &Flock) -> Vec<ForceVector> {
    let mut force_list: Vec<ForceVector> = Vec::new();
    for boid_index in 0..flock.boids.len() {
        let boid = flock.boids.get(boid_index).unwrap();
        let target = flock.centers_of_flock.get(boid_index).unwrap();
        let mut force = ForceVector::new(
            (boid.coord.distance(*target)/5.0).pow(2) * BOID_COHESION_WEIGHT,
            get_angle_to_target(&boid.coord, target),
        );
        if force.magnitude == 0. {
            force = ForceVector::zero();
        }
        force_list.push(force);
    }
    force_list
}

fn calc_allignment_forces(flock: &Flock) -> Vec<ForceVector> {
    let mut force_list: Vec<ForceVector> = Vec::new();
    for boid_index in 0..flock.boids.len() {
        let boid = flock.boids.get(boid_index).unwrap();
        let nearby_group = flock.nearby.get(boid_index).unwrap();
        if nearby_group.len() == 1 {
            force_list.push(ForceVector::zero());
            continue
        }
        let mut intermediate_force_list: Vec<ForceVector> = Vec::new();
        for nearby_list_index in 0..nearby_group.len() {
            let nearby = nearby_group.get(nearby_list_index).unwrap();
            let target_boid = flock.boids.get(*nearby).unwrap();
            let dist = boid.coord.distance(target_boid.coord);
            if dist == 0.0 {
                continue
            }
            let weight = clamp((BOID_ALLIGNMENT_WEIGHT / dist) * BOID_NEARBY_DIST_THRESHOLD * 10.0, 0.0, BOID_NEARBY_DIST_THRESHOLD);
            let force = ForceVector::new(weight, target_boid.angle);
            intermediate_force_list.push(force);
        }
        let mut result_force: ForceVector = ForceVector::zero();
        for i in 0..intermediate_force_list.len() {
            let second_force_vector = intermediate_force_list.get(i).unwrap();
            result_force = add_polar_vectors(&result_force, second_force_vector);
        }
        force_list.push(result_force);
    }
    force_list
}

fn calc_separation_forces(flock: &Flock) -> Vec<ForceVector> {
    let mut force_list: Vec<ForceVector> = Vec::new();
    for boid_index in 0..flock.boids.len() {
        let boid = flock.boids.get(boid_index).unwrap();
        let nearby_group = flock.nearby.get(boid_index).unwrap();
        if nearby_group.len() == 1 {
            force_list.push(ForceVector::zero());
            continue
        }
        let mut intermediate_force_list: Vec<ForceVector> = Vec::new();
        for nearby_list_index in 0..nearby_group.len() {
            let nearby = nearby_group.get(nearby_list_index).unwrap();
            let target_boid = flock.boids.get(*nearby).unwrap();
            let dist = boid.coord.distance(target_boid.coord);
            if dist == 0.0 {
                continue
            }
            let weight = clamp((BOID_SEPARATION_WEIGHT / dist) * BOID_SEPARATION_DIST_TARGET * 50.0 , 0.0, BOID_SEPARATION_DIST_TARGET);
            let angle = (get_angle_to_target(&boid.coord, &target_boid.coord)) + PI;
            let force = ForceVector::new(weight, angle);
            intermediate_force_list.push(force);
        }
        let mut result_force: ForceVector = ForceVector::zero();
        for i in 0..intermediate_force_list.len() {
            let second_force_vector = intermediate_force_list.get(i).unwrap();
            result_force = add_polar_vectors(&result_force, second_force_vector);
        }
        force_list.push(result_force);
    }
    force_list
}


fn combine_force_vectors(flock: &Flock) -> Vec<ForceVector> {
    let mut force_list: Vec<ForceVector> = Vec::new();
    for boid_index in 0..flock.boids.len() {
        let result = add_polar_vectors(
            flock.cohesion.get(boid_index).unwrap_or(&ForceVector::zero()),
            &add_polar_vectors(
                flock.allignment.get(boid_index).unwrap_or(&ForceVector::zero()),
                flock.separation.get(boid_index).unwrap_or(&ForceVector::zero())));
        let force_vector = ForceVector::new(result.magnitude, result.angle);
        force_list.push(force_vector);
    }
    force_list
}

fn add_polar_vectors(v1: &ForceVector, v2: &ForceVector) -> ForceVector {
    let magnitude = ((v1.magnitude.powi(2) + v2.magnitude.powi(2))
        + (v1.magnitude * v2.magnitude) * (v2.angle - v1.angle).cos())
    .sqrt();
    let angle = v1.angle
        + f32::atan2(
            v2.magnitude * (v2.angle - v1.angle).sin(),
            v1.magnitude + v2.magnitude * (v2.angle - v1.angle).cos(),
        );
    ForceVector::new(magnitude, angle)
}

pub fn get_angle_to_target(origin: &Vec2, target: &Vec2) -> f32 {
    let delta_x = target.x - origin.x;
    let delta_y = target.y - origin.y;
    let mut angle = (delta_y).atan2(delta_x);
    if angle < 0. {
        angle += PI * 2.0;
    }
    angle
}
