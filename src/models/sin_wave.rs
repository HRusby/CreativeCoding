use nannou::prelude::*;
use super::nannou_model::NannouModel;

pub struct SinWave;

impl NannouModel for SinWave{
    type Model = SinWaveModel;
    fn run_model(&self) {
        nannou::app(Self::model)
            .update(Self::update)
            .run();
    }

    fn model(app: &App) -> Self::Model{
        let _window = app.new_window().view(Self::view).build().unwrap();
        SinWaveModel {_window}
    }
    
    fn update(_app: &App, _model: &mut Self::Model, _update: Update) {}

    fn view(app: &App, _model: &Self::Model, frame: Frame) {
        let draw = app.draw();
        let boundary = app.window_rect();
        draw.background().color(BLACK);
        let scaling_factor = 50.0;
        let points = (0..10000).map(|i|{
            let x = (i as f32)*0.01;
            let point = pt2(boundary.left()+(x*scaling_factor), x.sin()*scaling_factor);
            (point, WHITE)
        });
        draw.polyline()
            .weight(2.0)
            .points_colored(points);
        draw.to_frame(app, &frame).unwrap();
    }

}

pub struct SinWaveModel {
    _window: window::Id,
}
