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
        let scaling_factor = 50.0;
        let vertical_gap = 50.0;
        let thickness = 5.0;
        let speed_factor = 0.01;
        let fade_factor = 0.01;

        let draw = app.draw(); 
        let time = (app.elapsed_frames() as f32) * speed_factor;
        let boundary = app.window_rect();
        let x = boundary.left() + (time*scaling_factor);
        let y = time.sin()*scaling_factor;
        let xy = pt2(x, y);
        draw.ellipse().xy(xy).color(WHITE).radius(thickness);
        
        let xy2 = pt2(x, y + vertical_gap);
        draw.ellipse().xy(xy2).color(GREEN).radius(thickness);
        draw.rect().w_h(boundary.w(), boundary.h()).color(srgba(0.0, 0.0, 0.0, fade_factor));
        draw.to_frame(app, &frame).unwrap();
    }
}
