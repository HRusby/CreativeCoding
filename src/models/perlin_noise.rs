use nannou::prelude::*;
use nannou::noise::*;
use super::nannou_model::NannouModel;

pub struct PerlinNoise;
pub struct PerlinNoiseModel{
    _window: window::Id,
    points: Vec<Vec2>,
    noise: Perlin
}

impl NannouModel for PerlinNoise{
    type Model = PerlinNoiseModel;

    fn run_model(&self) {
        nannou::app(Self::model)
            .update(Self::update)
            .run();
    }

    fn model(app: &nannou::App) -> Self::Model {
        let _window = app.new_window().view(Self::view).build().unwrap();
        app.draw().background().color(BLACK);
        let mut points = Vec::new();
        
        let boundary = app.window_rect();
        let window_x = boundary.w();
        let window_y = boundary.h();
        for i in 0..50{
            
            let point = vec2(
                (random::<f32>()-0.5)*window_x,
                (random::<f32>()-0.5)*window_y
            );
            points.push(point);
        }
        let noise = Perlin::new();
        PerlinNoiseModel {_window, points, noise}
    }

    fn update(_app: &nannou::App, _model: &mut Self::Model, _update: nannou::prelude::Update) {}

    fn view(app: &nannou::App, model: &Self::Model, frame: nannou::Frame) {

        let draw = app.draw(); 
        let time = app.elapsed_frames() as f32 ;
        let boundary = app.window_rect();



        // draw.rect().w_h(boundary.w(), boundary.h()).color(srgba(0.0, 0.0, 0.0, fade_factor));
        for point in model.points.iter(){
            draw.ellipse().xy(*point).radius(3.0).color(WHITE);
        }
        draw.to_frame(app, &frame).unwrap();
    }
}
