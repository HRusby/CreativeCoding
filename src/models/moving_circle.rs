use nannou::prelude::*;
use super::nannou_model::NannouModel;

pub struct MovingCircle;
pub struct MovingCircleModel{
    _window: window::Id,
}

impl NannouModel for MovingCircle{
    type Model = MovingCircleModel;

    fn run_model(&self) {
        nannou::app(Self::model)
            .update(Self::update)
            .run();
    }

    fn model(app: &nannou::App) -> Self::Model {
        let _window = app.new_window().view(Self::view).build().unwrap();
        MovingCircleModel{_window}
    }

    fn update(_app: &nannou::App, _model: &mut Self::Model, _update: nannou::prelude::Update) {
    }

    fn view(app: &nannou::App, _model: &Self::Model, frame: nannou::Frame) {
        let draw = app.draw();

        // Define two Sine Waves based on app time
        let sine = app.time.sin();
        let slowersine = (app.time/2.0).sin();

        // Get Boundary of our current window
        let boundary = app.window_rect();

        // Map each sin wave to ranges between the boundaries of the window
        let x = map_range(slowersine, -1.0, 1.0, boundary.left(), boundary.right());
        let y = map_range(sine, -1.0, 1.0, boundary.bottom(), boundary.top());

        // Clear the background to a plain black
        draw.background().color(BLACK);
        // Draw the Ellipse at the calculated x/y co-ords
        draw.ellipse().color(WHITE).x_y(x, y);

        draw.to_frame(app, &frame).unwrap();
    }
}
