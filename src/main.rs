mod flower;

use nannou::prelude::*;
use nannou;
use std::time::Instant;

use nannou::winit::event::VirtualKeyCode;
use flower::*;

const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;

struct Model {
    flowers: Vec<Flower>,
    current_gene: FlowerGene
}

fn main() {
    nannou::app(setup)
        .update(update)
        .fullscreen()
        .run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKOLIVEGREEN);

    let current_time = Instant::now();
    for flower in model.flowers.iter() {
        flower.draw(&draw, &current_time);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn setup(app: &App) -> Model {
    let _window_id = app.new_window()
        .size(WIDTH, HEIGHT)
        .title("Bloup")
        .view(view)
        .event(event)
        .build().unwrap();

    Model {
        flowers: Vec::new(),
        current_gene: Default::default(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                VirtualKeyCode::Key1 => {
                    model.current_gene = FlowerGene::default()
                }
                VirtualKeyCode::Key2 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::new(95, 34, 21),
                        num_petals: 9,
                        petal_radius: 40.0,
                        petal_color: Srgb::new(239, 191, 18),
                        bloom_duration: 7.0,
                    }
                }
                VirtualKeyCode::Key3 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::new(247, 225, 148),
                        num_petals: 6,
                        petal_radius: 40.0,
                        petal_color: Srgb::new(201, 165, 201),
                        bloom_duration: 7.0,
                    }
                }
                _ => {}
            }
        }
        MousePressed(_) => {
            let mouse_position = app.mouse.position();
            let orientation = random::<f32>() * TAU;
            let max_radius = Flower::max_radius(mouse_position, &model.flowers);

            let initial_radius = model.current_gene.centre_dist + model.current_gene.petal_radius;
            let scale = (max_radius / initial_radius).min(1.0);
            
            if scale < 0.25 {
                return
            }

            let mut scaled_flower = model.current_gene.clone();
            scaled_flower.centre_size *= scale;
            scaled_flower.centre_dist *= scale;
            scaled_flower.petal_radius *= scale;
            
            let new_flower = Flower::new(mouse_position, scaled_flower, orientation);
            model.flowers.push(new_flower);
            
            // // Todo: mutate model.flower_gene which will result in the next flower being different.
            // let mutation_val = 2.0;
            // model.current_gene.centre_size += mutation_val;
            // model.current_gene.centre_dist += mutation_val;
            // model.current_gene.petal_radius += mutation_val;
        }
        _ => {}
    }
}