mod flower;

use flower::*;
use nannou;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::time::Instant;

// TODO:
//  - new name
//  - github repo
//  - flower death
//  - allow the flowers to spread on their own

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

struct Model {
    flowers: Vec<Flower>,
    current_gene: FlowerGene,
    egui: Egui,
    mouse_down: bool,
    mouse_history: Vec<Vec2>,
}

fn main() {
    nannou::app(setup).update(update).fullscreen().run();
}

fn setup(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .title("Bloup")
        .view(view)
        .raw_event(raw_window_event)
        .event(event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    app.window(window_id).unwrap().set_cursor_visible(false);

    Model {
        flowers: Vec::new(),
        current_gene: Default::default(),
        egui,
        mouse_down: false,
        mouse_history: Vec::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb8(81, 119, 64));

    let current_time = Instant::now();
    for flower in model.flowers.iter() {
        flower.draw(&draw, &current_time);
    }

    // flower path guide
    if model.mouse_history.len() >= 2 {
        draw.polyline()
            .weight(1.5)
            .color(rgba8(200, 200, 200, 50))
            .points(model.mouse_history.clone());
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();

    // new draw to bring cursor in front of egui
    let new_draw = app.draw();
    draw_cursor(app, &new_draw, model, app.mouse.position());
    new_draw.to_frame(app, &frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    ctx.set_style(style);

    egui::Window::new("Flower Editor").show(&ctx, |ui| {
        model.current_gene.egui(ui);
        if ui.button("Reset").clicked() {
            model.flowers.clear()
        }
    });
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    let mouse_position = app.mouse.position();

    match event {
        MousePressed(button) => match button {
            MouseButton::Left => {
                model.mouse_down = true;
                model.mouse_history.push(mouse_position);
            }
            MouseButton::Right => {
                if let Some(flower_index) = model
                    .flowers
                    .iter()
                    .position(|f| mouse_position.distance(f.pos) < f.gene.size_px)
                {
                    model.flowers.remove(flower_index);
                }
            }
            _ => {}
        },
        MouseReleased(button) => {
            if button == MouseButton::Left {
                model.mouse_down = false;
                for point in model.mouse_history.iter() {
                    if let Some(scaled_flower) = can_place_flower(app, model, *point) {
                        let orientation = random::<f32>() * TAU;
                        let new_flower = Flower::new(*point, scaled_flower, orientation);
                        model.flowers.push(new_flower);
                    }
                }
                model.mouse_history.clear();
            }
        }
        MouseMoved(p) => {
            if model.mouse_down {
                model.mouse_history.push(p);
            }
        }
        _ => {}
    }
}

fn draw_cursor(app: &App, draw: &Draw, model: &Model, cursor_pos: Vec2) {
    let colour = if let Some(_gene) = can_place_flower(app, model, cursor_pos) {
        rgb8(90, 62, 43)
    } else {
        RED
    };

    draw.ellipse()
        .xy(cursor_pos)
        .wh(Vec2::new(22.0, 10.0))
        .rotate(PI / 4.0)
        .color(colour)
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

fn can_place_flower(app: &App, model: &Model, mouse_position: Vec2) -> Option<FlowerGene> {
    let max_radius = Flower::max_radius(app, mouse_position, &model.flowers);
    let scale = (max_radius / model.current_gene.size_px).min(1.0);

    if scale > 0.25 {
        let mut new = model.current_gene.clone();
        new.size_px *= scale;
        Some(new)
    } else {
        None
    }
}
