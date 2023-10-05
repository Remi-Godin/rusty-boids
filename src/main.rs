#![allow(unused)]
mod modules;
use std::env;
use modules::{boids::*, config::*};
use nannou::prelude::*;

struct Model {
    _window: window::Id,
    flock: Flock,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    nannou::LoopMode::rate_fps(UPDATE_RATE);
    let _window = app
        .new_window()
        .size(WINDOW_W, WINDOW_H)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        flock: Flock::new(BOID_COUNT),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Here goes the code that I want to run every update cycle
    model.flock.start_flock();
}

fn draw_boid(draw: &Draw, boid: &Boid, index: &usize) {
    let text = format!("ID: {}", index);
    draw.text(&text)
        .x_y(boid.coord.x + 45.0, boid.coord.y)
        .color(BLACK);
    draw.arrow()
        .color(BLACK)
        .start(boid.coord)
        .end(
            boid.coord
                + BOID_SIZE / 5.0 * Vec2::new(10.0 * boid.angle.cos(), 10.0 * boid.angle.sin()),
        )
        .head_width(BOID_SIZE / 3.0)
        .head_length(BOID_SIZE);
    draw.rect()
        .color(RED)
        .w(BOID_SIZE)
        .h(BOID_SIZE)
        .x_y(boid.coord.x, boid.coord.y)
        .z_radians(boid.angle);
}

fn draw_cohesion_vectors(draw: &Draw, flock: &Flock) {
    for i in 0..flock.boids.len() {
        let origin = flock.boids.get(i).unwrap();
        let target = flock.cohesion.get(i).unwrap();
        let boid = flock.boids.get(i).unwrap();
        draw.arrow().start(origin.coord).end(
            boid.coord
                + Vec2::new(
                    target.magnitude * target.angle.cos(),
                    target.magnitude * target.angle.sin(),
                ),
        );
    }
}

fn draw_separation_vectors(flock: &Flock) {}

fn draw_allignement_vectors(draw: &Draw, flock: &Flock) {
    for i in 0..flock.boids.len() {
        let origin = flock.boids.get(i).unwrap();
        let target = flock.allignment.get(i).unwrap();
        let boid = flock.boids.get(i).unwrap();
        draw.arrow().start(origin.coord).end(
            boid.coord
                + Vec2::new(
                    target.magnitude * target.angle.cos(),
                    target.magnitude * target.angle.sin(),
                ),
        ).color(BLUE);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Here goes the code for the rendering of things
    let draw = app.draw();
    for boid_index in 0..model.flock.boids.len() {
        draw_boid(&draw, model.flock.boids.get(boid_index).unwrap(), &boid_index);
    }
    draw_cohesion_vectors(&draw, &model.flock);
    draw_allignement_vectors(&draw, &model.flock);
    for com in model.flock.centers_of_flock.iter() {
        draw.rect().color(ORANGE).x_y(com.x, com.y).w(5.0).h(5.0);
    }
    draw.rect().color(BLUE).width(5.).height(5.).x_y(0., 0.);
    draw.background().color(DARKGRAY);
    draw.to_frame(app, &frame).unwrap();
}
