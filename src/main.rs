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

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.flowers.len() {
        for j in (i + 1)..model.flowers.len() {
            let (flower_1, flower_2) = {
                let (left, right) = model.flowers.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };

            if flower_1.is_touching_other_flower(flower_2) {
                Flower::shift_flowers(flower_2, flower_1)
            }
        }
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                VirtualKeyCode::Key1 => {
                    model.current_gene = FlowerGene::default()
                }
                VirtualKeyCode::Key2 => {
                    model.current_gene = FlowerGene {
                        centre_size: 40.0,
                        centre_color: Srgb::new(210, 181, 64),
                        num_petals: 10,
                        petal_color: WHITESMOKE,
                        petal_radius: 50.0,
                        centre_dist: 40.0,
                        bloom_duration: 7.0,
                    }
                }
                VirtualKeyCode::Key3 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_color: Srgb::new(95, 34, 21),
                        num_petals: 9,
                        petal_color: Srgb::new(239, 191, 18),
                        petal_radius: 50.0,
                        centre_dist: 40.0,
                        bloom_duration: 7.0,
                    }
                }
                VirtualKeyCode::Key4 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_color: Srgb::new(95, 34, 21),
                        num_petals: 6,
                        petal_color: Srgb::new(201, 165, 201),
                        petal_radius: 50.0,
                        centre_dist: 40.0,
                        bloom_duration: 7.0,
                    }
                }
                VirtualKeyCode::Key5 => {}
                VirtualKeyCode::Key6 => {}
                VirtualKeyCode::Key7 => {}
                VirtualKeyCode::Key8 => {}
                VirtualKeyCode::Key9 => {}
                _ => {}
            }
        }
        MousePressed(_) => {
            let mouse_position = app.mouse.position();
            let random_orientation = random::<f32>() * TAU;
            let new_flower = Flower::new(mouse_position, model.current_gene.clone(), random_orientation);
            
            model.flowers.push(new_flower);

            // Todo: mutate model.flower_gene which will result in the next flower being different.
            let mutation_val = 2.0;
            model.current_gene.centre_size += mutation_val;
            model.current_gene.centre_dist += mutation_val;
            model.current_gene.petal_radius += mutation_val;
        }
        _ => {}
    }
}