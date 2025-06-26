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
    pub(crate) centre_radius: f32,
    pub(crate) centre_dist: f32,
    pub(crate) centre_color: LinSrgba<f32>,
    pub(crate) num_petals: usize,
    pub(crate) petal_radius: f32,
    pub(crate) petal_thickness: f32,
    pub(crate) petal_color: LinSrgba<f32>,
    pub(crate) bloom_duration: f32,
}

impl Default for FlowerGene {
    fn default() -> Self {
        FlowerGene {
            centre_radius: 20.0,
            centre_dist: 40.0,
            centre_color: Srgb::<u8>::new(236, 178, 63).into_lin_srgba(),
            num_petals: 5,
            petal_radius: 25.0,
            petal_thickness: 0.75,
            petal_color: FLORALWHITE.into_lin_srgba(),
            bloom_duration: 5.0,
        }
    }
}

impl FlowerGene {
    pub fn egui(&mut self, ui: &mut Ui) {
        // let x = (1.)..=10.;
        FlowerGene::slider(&mut self.num_petals, "Petal Count:", 3..=20, ui);

        FlowerGene::slider(&mut self.petal_radius, "Petal Radius:", 1.0..=100.0, ui);

        FlowerGene::slider(&mut self.centre_radius, "Centre Size:", 1.0..=50.0, ui);

        FlowerGene::slider(&mut self.centre_dist, "Centre Distance:", 0.0..=100.0, ui);

        FlowerGene::slider(&mut self.bloom_duration, "Bloom Duration:", 1.0..=10.0, ui);

        FlowerGene::picker(&mut self.petal_color, "Petal Colour:", ui);
    }
    
    fn slider<T: Numeric>(value: &mut T, name: &str, range: RangeInclusive<T>, ui: &mut Ui) {
        ui.label(name);
        ui.add(egui::Slider::new(value, range));
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

    pub fn radius(&self) -> f32 {
        self.gene.centre_dist + self.gene.petal_radius
    }

    pub fn max_radius(new_pos: Vec2, others: &[Flower]) -> f32 {
        others
            .iter()
            .map(|other| other.pos.distance(new_pos) - other.radius())
            .fold(f32::INFINITY, |acc, b| acc.min(b) )
            .max(0.0)
    }
    // return option or add an assert when creating a flower

    pub fn draw(&self, draw: &Draw, current_time: &Instant) {
        let elapsed = current_time.duration_since(self.start_time).as_secs_f32();
        for petal in 0..self.gene.num_petals {
            let petal_prop = petal as f32 / self.gene.num_petals as f32;
            let petal_angle = petal_prop * TAU + self.orientation; // TAU = 2 * pi = 360 degrees

            let petal_radius = self.bloom_progress(elapsed) * self.gene.petal_radius;
            let distance_from_centre = self.gene.centre_dist * self.bloom_progress(elapsed);

            let p =
                self.pos + Vec2::new(petal_angle.cos(), petal_angle.sin()) * distance_from_centre;

            let petal_height = (petal % 2) as f32;
            
            draw.ellipse()
                .xy(p)
                .z(-petal_height)
                .wh(Vec2::new(1.0, self.gene.petal_thickness) * petal_radius * 2.0)
                .rotate(petal_angle)
                .color(self.gene.petal_color)
                .stroke(mult_colour(self.gene.petal_color, 0.5))
                .stroke_weight(2.0);
        }

        let centre_radius = self.bloom_progress(elapsed) * self.gene.centre_radius;

        draw.ellipse()
            .xy(self.pos)
            .z(-2.0)
            .radius(centre_radius)
            .color(self.gene.petal_color)
            .stroke(mult_colour(self.gene.petal_color, 0.5))
            .stroke_weight(4.0);
        
        draw.ellipse()
            .xy(self.pos)
            .radius(centre_radius)
            .color(self.gene.petal_color);

        draw.ellipse()
            .xy(self.pos)
            .radius(centre_radius / 1.5)
            .color(self.gene.centre_color);

        // debug cirle
        // draw.ellipse()
        //     .xy(self.pos)
        //     .radius(self.radius())
        //     .no_fill()
        //     .stroke(RED)
        //     .stroke_weight(1.0);
    }
}

pub fn mult_colour(colour: LinSrgba<f32>, mult: f32) -> LinSrgba {
    let mut colour = colour;
    colour.red *= mult;
    colour.blue *= mult;
    colour.green *= mult;
    
    colour
}