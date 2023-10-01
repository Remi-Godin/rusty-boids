mod modules;
use modules::{boids::Flock, config::*};
use nannou::prelude::*;

struct Model {
    _window: window::Id,
    flock: Flock,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    nannou::LoopMode::rate_fps(UPDATE_RATE);
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        flock: Flock::new(10),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Here goes the code that I want to run every update cycle
    for boid in model.flock.boids.iter_mut() {
        boid.move_boid(0.01);
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    // Here goes the code for the rendering of things
    let my_flock = model.flock.boids.iter();
    let draw = app.draw();
    for boid in my_flock {
        draw.tri()
            .color(RED)
            .w(5.0)
            .h(5.0)
            .x_y(boid.x, boid.y)
            .z_radians(boid.r);
    }
    draw.background().color(DARKGRAY);
    draw.to_frame(app, &frame).unwrap();
}
