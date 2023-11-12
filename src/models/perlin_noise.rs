use nannou::prelude::*;
use nannou::noise::*;
use super::nannou_model::NannouModel;

pub struct Thing{
    position: Vec2,
    color: Srgba,
    radius: f32,
}
impl Thing{
    pub fn new(p:Vec2, c:Srgba, r:f32) -> Self{
        Thing {
            position:p,
            color:c,
            radius:r,
        }
    }
}

pub struct PerlinNoise;
pub struct PerlinNoiseModel{
    _window: window::Id,
    points: Vec<Thing>,
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
        for _i in 0..500{
            
            let point = Thing::new(
                vec2(
                    (random::<f32>()-0.5)*window_x,
                    (random::<f32>()-0.5)*window_y
                ),
                srgba(255.0*random::<f32>()+1.0, 255.0*random::<f32>()+1.0, 255.0*random::<f32>()+1.0, 255.0),
                random::<f32>()+0.25*2.0,
            );
            points.push(point);
        }
        let noise = Perlin::new();
        noise.set_seed(10);
        PerlinNoiseModel {_window, points, noise}
    }

    fn update(_app: &nannou::App, model: &mut Self::Model, _update: nannou::prelude::Update) {
        let ns = 0.01;
        for point in model.points.iter_mut(){
            let scaled_point = [ns*point.position.x as f64, ns*point.position.y as f64];
            point.position += vec2(
                model.noise.get([scaled_point[0], scaled_point[1], 0.0]) as f32,
                model.noise.get([scaled_point[0], scaled_point[1], 0.5]) as f32
            );
        }
    }

    fn view(app: &nannou::App, model: &Self::Model, frame: nannou::Frame) {

        let draw = app.draw(); 
        let _time = app.elapsed_frames() as f32 ;
        let _boundary = app.window_rect();


        for point in model.points.iter(){
            draw.ellipse()
                .xy(point.position)
                .radius(point.radius)
                .color(point.color);
        }
        draw.to_frame(app, &frame).unwrap();
    }
}
