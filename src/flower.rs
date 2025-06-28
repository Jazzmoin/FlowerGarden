use std::ops::RangeInclusive;
use nannou::color::conv::IntoLinSrgba;
use crate::*;
use nannou;
use nannou::prelude::*;
use nannou::color::IntoColor;

use std::time::Instant;
use nannou_egui::color_picker::Alpha;
use nannou_egui::egui::emath::Numeric;
use nannou_egui::egui::{Color32, Ui};

#[derive(Clone, Debug)]
pub(crate) struct FlowerGene {
    pub(crate) size_px: f32,
    pub(crate) centre_radius_inner_prop: f32,
    pub(crate) centre_radius_outer_prop: f32,
    pub(crate) centre_dist_prop: f32,
    pub(crate) petal_radius_prop: f32,
    pub(crate) petal_width_prop: f32,
    pub(crate) num_petals: usize,
    pub(crate) bloom_duration: f32,
    pub(crate) centre_color: LinSrgba<f32>,
    pub(crate) petal_color: LinSrgba<f32>,
}

impl Default for FlowerGene {
    fn default() -> Self {
        FlowerGene {
            size_px: 65.0,
            centre_radius_inner_prop: 0.3,
            centre_radius_outer_prop: 0.4,
            centre_dist_prop: 0.65,
            petal_radius_prop: 0.35 ,
            petal_width_prop: 0.27,
            num_petals: 5,
            bloom_duration: 5.0,
            centre_color: Srgb::<u8>::new(236, 178, 63).into_lin_srgba(),
            petal_color: FLORALWHITE.into_lin_srgba(),
        }
    }
}

impl FlowerGene {
    pub fn egui(&mut self, ui: &mut Ui) {
        // let x = (1.)..=10.;
        FlowerGene::slider(&mut self.size_px, "Flower Size:", 10.0..=200.0, ui);

        FlowerGene::stepped_slider(&mut self.num_petals, "Petal Count:", 4..=20, ui,2.0);

        FlowerGene::slider(&mut self.bloom_duration, "Bloom Duration:", 1.0..=10.0, ui);

        FlowerGene::picker(&mut self.petal_color, "Petal Colour:", ui);


    }
    
    fn slider<T: Numeric>(value: &mut T, name: &str, range: RangeInclusive<T>, ui: &mut Ui) {
        ui.label(name);
        ui.add(egui::Slider::new(value, range));
    }
    
    fn stepped_slider<T: Numeric>(value: &mut T, name: &str, range: RangeInclusive<T>, ui: &mut Ui, step: f64) {
        ui.label(name);
        ui.add(egui::Slider::new(value, range).step_by(step));
    }

    fn picker(value: &mut LinSrgba, name: &str, ui: &mut Ui) {
        ui.label(name);
        let c: Srgba<u8> = value.into_encoding().into_format();
        let mut c = Color32::from_rgb(c.red, c.green, c.blue);
        if egui::color_picker::color_picker_color32(ui, &mut c, Alpha::Opaque) {
            *value = Srgb::<u8>::from_components((c.r(), c.g(), c.b())).into_lin_srgba();
        };
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Flower {
    pub(crate) gene: FlowerGene,
    pub(crate) pos: Vec2,
    pub(crate) start_time: Instant,
    pub(crate) orientation: f32,
}

impl Flower {
    pub fn new_default(pos: Vec2) -> Self {
        Flower {
            gene: Default::default(),
            pos,
            start_time: Instant::now(),
            orientation: 0.0,
        }
    }

    pub fn new(pos: Vec2, gene: FlowerGene, orientation: f32) -> Self {
        Self {
            gene,
            pos,
            start_time: Instant::now(),
            orientation,
        }
    }

    pub fn bloom_progress(&self, elapsed: f32) -> f32 {
        let x = elapsed / self.gene.bloom_duration;
        (1.0 - (1.0 - x).powi(3)).clamp(0.0, 1.0)
    }

    pub fn draw(&self, draw: &Draw, current_time: &Instant) {
        let elapsed = current_time.duration_since(self.start_time).as_secs_f32();
        let scale = self.bloom_progress(elapsed) * self.gene.size_px;

        let sum = self.gene.centre_dist_prop + self.gene.petal_radius_prop;
        let petal_distance = self.gene.centre_dist_prop / sum;
        let petal_radius = self.gene.petal_radius_prop / sum;
        let petal_width = self.gene.petal_width_prop / sum;

        let petal_wh = Vec2::new(petal_radius, petal_width) * 2.0;

        for petal in 0..self.gene.num_petals {
            let petal_prop = petal as f32 / self.gene.num_petals as f32;
            let petal_angle = petal_prop * TAU + self.orientation; // TAU = 2 * pi = 360 degrees
            let pos_offset = Vec2::new(petal_angle.cos(), petal_angle.sin()) * petal_distance;
            let petal_z = (petal % 2) as f32;

            draw.ellipse()
                .xy(self.pos + pos_offset * scale)
                .z(-petal_z)
                .wh(petal_wh * scale)
                .rotate(petal_angle)
                .color(self.gene.petal_color)
                .stroke(mult_colour(self.gene.petal_color, 0.5))
                .stroke_weight(2.0);
        }

        draw.ellipse()
            .xy(self.pos)
            .z(-2.0)
            .radius(self.gene.centre_radius_outer_prop * scale)
            .color(self.gene.petal_color)
            .stroke(mult_colour(self.gene.petal_color, 0.5))
            .stroke_weight(4.0);

        draw.ellipse()
            .xy(self.pos)
            .radius(self.gene.centre_radius_outer_prop * scale)
            .color(self.gene.petal_color);

        draw.ellipse()
            .xy(self.pos)
            .radius(self.gene.centre_radius_inner_prop * scale)
            .color(self.gene.centre_color);

        // debug cirle
        // draw.ellipse()
        //     .xy(self.pos)
        //     .radius(scale)
        //     .no_fill()
        //     .stroke(RED)
        //     .stroke_weight(1.0);
    }

    pub fn max_radius(app: &App, new_pos: Vec2, others: &[Flower]) -> f32 {
        let radius = others
            .iter()
            .map(|other| other.pos.distance(new_pos) - other.gene.size_px)
            .fold(f32::INFINITY, |acc, b| acc.min(b) )
            .max(0.0) + 1.0;
        
        let border = app.window_rect();
        let left_dist:f32 = new_pos.x - border.left();
        let right_dist:f32 = border.right() - new_pos.x;
        let top_dist:f32 = border.top() - new_pos.y;
        let bottom_dist:f32 = new_pos.y - border.bottom();
        
        let closest_edge = left_dist.min(right_dist).min(top_dist).min(bottom_dist);
        
        radius.min(closest_edge)
    }
}

pub fn mult_colour(colour: LinSrgba<f32>, mult: f32) -> LinSrgba {
    let mut colour = colour;
    colour.red *= mult;
    colour.blue *= mult;
    colour.green *= mult;

    colour
}