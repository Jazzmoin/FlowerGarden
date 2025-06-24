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
    current_gene: FlowerGene,
    cursor_pos: Vec2,
}

fn main() {
    nannou::app(setup)
        .update(update)
        .fullscreen()
        .run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb8(81, 119, 64));

    let current_time = Instant::now();
    for flower in model.flowers.iter() {
        flower.draw(&draw, &current_time);
    }
    
    draw.ellipse()
        .xy(model.cursor_pos)
        .wh(Vec2::new(22.0, 10.0))
        .rotate(PI / 4.0)
        .color(rgb8(90, 62, 43))
        .stroke(rgb8(56, 44, 32))
        .stroke_weight(1.0);
    
    
    draw.ellipse()
        .xy(model.cursor_pos)
        .wh(Vec2::new(22.0, 4.0))
        .rotate(PI / 4.0)
        .no_fill()
        .stroke(rgb8(56, 44, 32))
        .stroke_weight(1.0);
    
    draw.to_frame(app, &frame).unwrap();
}

fn setup(app: &App) -> Model {
    let _window_id = app.new_window()
        .size(WIDTH, HEIGHT)
        .title("Bloup")
        .view(view)
        .event(event)
        .build().unwrap();

    app.window(_window_id).unwrap().set_cursor_visible(false);

    Model {
        flowers: Vec::new(),
        current_gene: Default::default(),
        cursor_pos: Vec2::new(0.0,0.0)
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
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

        MouseMoved(_) => {
            model.cursor_pos = app.mouse.position();
        }

        KeyPressed(key) => {
            match key {
                VirtualKeyCode::Key1 => {
                    model.current_gene = FlowerGene::default()
                }
                VirtualKeyCode::Key2 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::new(245, 213, 71),
                        num_petals: 9,
                        petal_radius: 40.0,
                        petal_color: Srgb::new(232, 174, 183),
                        bloom_duration: 7.0,
                    }
                }
                VirtualKeyCode::Key3 => {
                    model.current_gene = FlowerGene {
                        centre_size: 50.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::new(216, 111, 69),
                        num_petals: 6,
                        petal_radius: 40.0,
                        petal_color: Srgb::new(189, 160, 203),
                        bloom_duration: 7.0,
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}



// TODOs:
// add egui
// make a cursor
// make the cursor flash red when a flower can't be placed