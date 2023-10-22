use nannou::prelude::*;
use super::nannou_model::NannouModel;

pub struct MovingSinWave;
pub struct MovingSinWaveModel{
    _window: window::Id,
}

impl NannouModel for MovingSinWave{
    type Model = MovingSinWaveModel;

    fn run_model(&self) {
        nannou::app(Self::model)
            .update(Self::update)
            .run();
    }

    fn model(app: &nannou::App) -> Self::Model {
        let _window = app.new_window().view(Self::view).build().unwrap();
        app.draw().background().color(BLACK);
        MovingSinWaveModel {_window}
    }

    fn update(_app: &nannou::App, _model: &mut Self::Model, _update: nannou::prelude::Update) {}

    fn view(app: &nannou::App, _model: &Self::Model, frame: nannou::Frame) {
        let draw = app.draw(); 

        let time = (app.elapsed_frames() as f32) * 0.01;
        let xy = pt2(time - 10.0, time.sin())*50.0;
        draw.ellipse().xy(xy).color(WHITE).radius(1.0);
        
        draw.to_frame(app, &frame).unwrap();
    }
}
