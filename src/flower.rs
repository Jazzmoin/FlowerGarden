use std::ops::Range;
use crate::*;
use nannou;
use nannou::prelude::*;

use std::time::Instant;
use nannou::glam::cast::Vec2Cast;
use rand::Rng;

#[derive(Clone, Debug)]
pub(crate) struct FlowerGene {
    pub(crate) centre_size: f32,
    pub(crate) centre_color: Srgb<u8>,
    pub(crate) num_petals: usize,
    pub(crate) petal_radius: f32,
    pub(crate) petal_color: Srgb<u8>,
    pub(crate) centre_dist: f32,
    pub(crate) bloom_duration: f32,
}

impl Default for FlowerGene {
    fn default() -> Self {
        FlowerGene {
            centre_size: 40.0,
            centre_color: Srgb::new(210, 181, 64),
            num_petals: 5,
            petal_radius: 25.0,
            petal_color: FLORALWHITE,
            centre_dist: 40.0,
            bloom_duration: 5.0,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Flower {
    pub(crate) gene: FlowerGene,
    pub(crate) pos: Vec2,
    pub(crate) start_time: Instant,
    pub(crate) orientation: f32
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

    pub fn new(pos: Vec2, gene: FlowerGene, random_orientation: f32) -> Self {
        Self {
            gene,
            pos,
            start_time: Instant::now(),
            orientation: random_orientation,
        }
    }

    pub fn bloom_progress(&self, elapsed: f32) -> f32 {
        let x = elapsed / self.gene.bloom_duration;
        (1.0 - (1.0 - x).powi(3)).clamp(0.0, 1.0)
    }

    pub fn dist_to_other_flower(&self, other: &Flower) -> f32 {
        self.pos.distance(other.pos)
    }

    pub fn surface_dist_to_other_flower(&self, other: &Flower) -> f32 {
        self.pos.distance(other.pos)
            - (self.gene.petal_radius + self.gene.centre_size)
            - (other.gene.petal_radius + other.gene.centre_size)
            + 50.0
    }

    pub fn is_touching_other_flower(&self, other: &Flower) -> bool {
        self.surface_dist_to_other_flower(other) <= 0.0
    }

    pub fn shift_flowers(&mut self, other: &mut Flower) {
        if self.pos == other.pos {
            let angle = random::<f32>() * TAU;
            self.pos += Vec2::new(angle.cos(), angle.sin()) * 0.01;
        }
        let dist = self.surface_dist_to_other_flower(other);
        if dist < 0.0 {
            self.pos -= (self.pos - other.pos).normalize() * dist / 1.98;
            other.pos -= (other.pos - self.pos).normalize() * dist / 1.98;
        }
    }
    
    pub fn radius(&self) -> f32 {
        self.gene.centre_dist + self.gene.petal_radius
    }
    
    pub fn draw(&self, draw: &Draw, current_time: &Instant) {
        let elapsed = current_time.duration_since(self.start_time).as_secs_f32();
        for petal in 0..self.gene.num_petals {
            let petal_prop = petal as f32 / self.gene.num_petals as f32;
            let petal_angle = petal_prop * TAU + self.orientation; // TAU = 2 * pi = 360 degrees

            let petal_radius = self.bloom_progress(elapsed) * self.gene.petal_radius;
            let distance_from_centre = self.gene.centre_dist * self.bloom_progress(elapsed);
            
            let p = self.pos + Vec2::new(petal_angle.cos(), petal_angle.sin()) * distance_from_centre;
            
            draw.ellipse()
                .xy(p)
                .wh(Vec2::new(1.0,0.6) * petal_radius * 2.0)
                .rotate(petal_angle)
                .color(self.gene.petal_color);
        }

        let centre_size = self.bloom_progress(elapsed) * self.gene.centre_size;
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .w_h(centre_size, centre_size)
            .color(self.gene.petal_color);
         
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .w_h(centre_size / 1.5, centre_size / 1.5)
            .color(self.gene.centre_color);
        
        // draw.ellipse()
        //     .xy(self.pos)
        //     .wh(Vec2::splat(self.radius()))
        //     .rgba8(0,0,0,0)
        //     .stroke(RED)
        //     .stroke_weight(2.0);
        
    }
}
