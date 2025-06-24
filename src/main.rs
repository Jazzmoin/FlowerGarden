mod flower;

use nannou::prelude::*;
use nannou;
use std::time::Instant;
use nannou::color::IntoLinSrgba;
use nannou_egui::{self, egui, Egui};
use nannou::winit::event::VirtualKeyCode;
use flower::*;

// TODO:
//  - new name
//  - github repo
//  - flower death
//  - petal definition 
//  - petal shapes
//  - petal layering?
//  - three flower presets
//  - petal width param
//  - allow the flowers to spread on their own
//  - serialisable flower gene (google serde derive)
//  - visual indicator if a flower won't fit
//  - colour picker in egui 
//  - add a master size to the flower gene and make the flowers a fraction of that size
//  - inner and outer circle for flower centre


const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;

struct Model {
    flowers: Vec<Flower>,
    current_gene: FlowerGene,
    egui: Egui,
}

fn main() {
    nannou::app(setup)
        .update(update)
        .fullscreen()
        .run();
}

fn setup(app: &App) -> Model {
    let window_id = app.new_window()
        .size(WIDTH, HEIGHT)
        .title("Bloup")
        .view(view)
        .raw_event(raw_window_event)
        .event(event)
        .build().unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    app.window(window_id).unwrap().set_cursor_visible(false);

    Model {
        flowers: Vec::new(),
        current_gene: Default::default(),
        egui
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb8(81, 119, 64));

    let current_time = Instant::now();
    for flower in model.flowers.iter() {
        flower.draw(&draw, &current_time);
    }


    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
    let new_draw = app.draw();

    draw_cursor(&new_draw, app.mouse.position());
    new_draw.to_frame(app, &frame).unwrap();
}

fn draw_cursor(draw: &Draw, cursor_pos: Vec2) {
    draw.ellipse()
        .xy(cursor_pos)
        .wh(Vec2::new(22.0, 10.0))
        .rotate(PI / 4.0)
        .color(rgb8(90, 62, 43))
        .stroke(rgb8(56, 44, 32))
        .stroke_weight(1.0);


    draw.ellipse()
        .xy(cursor_pos)
        .wh(Vec2::new(22.0, 4.0))
        .rotate(PI / 4.0)
        .no_fill()
        .stroke(rgb8(56, 44, 32))
        .stroke_weight(1.0);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let gene = &mut model.current_gene;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Flower Controls").show(&ctx, |ui| {
        ui.label("Petal Count:");
        ui.add(egui::Slider::new(&mut gene.num_petals, 3..=20));

        ui.label("Petal Radius:");
        ui.add(egui::Slider::new(&mut gene.petal_radius, 1.0..=100.0));

        ui.label("Centre Size:");
        ui.add(egui::Slider::new(&mut gene.centre_radius, 1.0..=50.0));

        ui.label("Centre Distance:");
        ui.add(egui::Slider::new(&mut gene.centre_dist, 0.0..=100.0));

        ui.label("Bloom Duration:");
        ui.add(egui::Slider::new(&mut gene.bloom_duration, 1.0..=10.0));
    });
    
}

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
            scaled_flower.centre_radius *= scale;
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

        KeyPressed(key) => {
            match key {
                VirtualKeyCode::Key1 => {
                    model.current_gene = FlowerGene::default()
                }
                VirtualKeyCode::Key2 => {
                    model.current_gene = FlowerGene {
                        centre_radius: 25.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::<u8>::new(245, 213, 71).into_lin_srgba(),
                        num_petals: 9,
                        petal_radius: 40.0,
                        petal_color: Srgb::<u8>::new(232, 174, 183).into_lin_srgba(),
                        bloom_duration: 7.0,
                        ..Default::default()
                    }
                }
                VirtualKeyCode::Key3 => {
                    model.current_gene = FlowerGene {
                        centre_radius: 25.0,
                        centre_dist: 50.0,
                        centre_color: Srgb::<u8>::new(216, 111, 69).into_lin_srgba(),
                        num_petals: 6,
                        petal_radius: 40.0,
                        petal_color: Srgb::<u8>::new(189, 160, 203).into_lin_srgba(),
                        bloom_duration: 7.0,
                        ..Default::default()
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}