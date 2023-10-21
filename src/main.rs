use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
}

// Run once at the begining of the app, produces a fresh instance of Model
fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {_window}
}

// General type of event occurring on a time frequency (60 times per second) 
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Presents the Model to some window
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
