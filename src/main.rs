use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    window: window::Id,
}

// Run once at the begining of the app, produces a fresh instance of Model
fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    Model {window}
}

// General type of event occurring on a time frequency (60 times per second) 
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Presents the Model to some window
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_size = app.window(model.window).unwrap().rect().w_h();
    let quarter_width = window_size.0 * 0.25;
    let quarter_height = window_size.1 * 0.25;
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).x_y(100.0, 100.0);
    draw.line()
        .color(BLACK)
        .start(vec2(0.0, -quarter_height))
        .end(vec2(0.0, quarter_height));
    draw.line()
        .color(BLACK)
        .start(vec2(-quarter_width, 0.0))
        .end(vec2(quarter_width, 0.0));
    draw.to_frame(app, &frame).unwrap();
}
