mod flower;

use nannou::prelude::*;
use nannou;
use std::time::Instant;
use nannou_egui::{self, egui, Egui};
use flower::*;

// TODO:
//  - new name
//  - github repo
//  - flower death
//  - petal shapes
//  - three flower presets
//  - allow the flowers to spread on their own
//  - serialisable flower gene (google serde derive)


const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;

struct Model {
    flowers: Vec<Flower>,
    current_gene: FlowerGene,
    egui: Egui,
    mouse_down: bool,
    mouse_history: Vec<Vec2>
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
    // 
    // style.visuals.window_fill = egui::Color32::from_rgb(253, 246, 227); // #FDF6E3
    // 
    // style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(244, 198, 165); // #F4C6A5
    // style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(242, 140, 140);  // #F28C8C
    // style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(225, 91, 100);    // #E15B64
    // style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(161, 134, 111); // #A1866F
    // 
    // 
    // style.visuals.window_rounding = egui::Rounding::same(6.0);
    // style.visuals.window_shadow = egui::epaint::Shadow::small_dark();
    // style.spacing.item_spacing = egui::vec2(8.0, 8.0);

    ctx.set_style(style);




    egui::Window::new("Flower Editor").show(&ctx, |ui| {
        model.current_gene.egui(ui);
    });
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    let mouse_position = app.mouse.position();
    let orientation = random::<f32>() * TAU;

    match event {
        MousePressed(button) => {
            model.mouse_down = true;
            model.mouse_history.push(mouse_position);
           
            match button {
                MouseButton::Left => {
                    if let Some(scaled_flower) = can_place_flower(app, model, mouse_position) {
                        let new_flower = Flower::new(mouse_position, scaled_flower, orientation);
                        model.flowers.push(new_flower);
                    }
                }
                MouseButton::Right => {
                    if let Some(flower_index) = model.flowers.iter().position(|f|{
                        mouse_position.distance(f.pos) < f.gene.size_px
                    }) {
                        model.flowers.remove(flower_index);
                    }
                }
                _ => {}
            }
        }
        MouseReleased(button) => {
            if button == MouseButton::Left {
                model.mouse_down = false;
                model.mouse_history.clear();
            }
        }
        MouseMoved(p) => {
            if model.mouse_down {
                if model.mouse_history.last().map_or(true, |last| last.distance(p) > model.current_gene.size_px + 1.0) {
                    model.mouse_history.push(p);

                    if let Some(scaled_flower) = can_place_flower(app, model, mouse_position) {
                        let new_flower = Flower::new(mouse_position, scaled_flower, orientation);
                        model.flowers.push(new_flower);
                    }
                }
            }
        }
        _ => {}
    }
}

fn draw_cursor(app: &App, draw: &Draw, model: &Model, cursor_pos: Vec2) {
    let colour = if let Some(gene) = can_place_flower(app, model, cursor_pos) {
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
