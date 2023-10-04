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

fn view(app: &App, model: &Model, frame: Frame) {
    // Here goes the code for the rendering of things
    let my_flock = model.flock.boids.iter();
    let draw = app.draw();
    draw.arrow()
        .start(Vec2::new(0.0, 0.0))
        .end(Vec2::new(0.0, 10.0))
        .z_radians(PI / 2.0);
    draw.tri()
        .w(BOID_SIZE)
        .h(BOID_SIZE)
        .z_radians(PI / 2.)
        .x_y(0., 0.)
        .color(BLACK);
    for boid in my_flock {
        let boid_vec2: Vec2 = Vec2::new(boid.coord.x, boid.coord.y);
        draw.arrow()
            .color(BLACK)
            .start(boid_vec2)
            .end(boid_vec2 + BOID_SIZE/5.0 * Vec2::new(10.0 * boid.r.cos(), 10.0 * boid.r.sin()))
            .head_width(BOID_SIZE/3.0)
            .head_length(BOID_SIZE);
        draw.rect()
            .color(RED)
            .w(BOID_SIZE)
            .h(BOID_SIZE)
            .x_y(boid.coord.x, boid.coord.y)
            .z_radians(boid.r);
    }
    for com in model.flock.center_of_flock_list.iter() {
        draw.rect().color(ORANGE).x_y(com.x, com.y).w(5.0).h(5.0);
    }
    draw.rect().color(BLUE).width(5.).height(5.).x_y(0., 0.);
    draw.background().color(DARKGRAY);
    draw.to_frame(app, &frame).unwrap();
}
