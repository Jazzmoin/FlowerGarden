use std::cmp::max;
use crate::*;
use nannou;
use nannou::prelude::*;

use std::time::Instant;

#[derive(Clone, Debug)]
pub(crate) struct FlowerGene {
    pub(crate) centre_size: f32,
    pub(crate) centre_dist: f32,
    pub(crate) centre_color: Srgb<u8>,
    pub(crate) num_petals: usize,
    pub(crate) petal_radius: f32,
    pub(crate) petal_color: Srgb<u8>,
    pub(crate) bloom_duration: f32,
}

impl Default for FlowerGene {
    fn default() -> Self {
        FlowerGene {
            centre_size: 40.0,
            centre_dist: 40.0,
            centre_color: Srgb::new(210, 181, 64),
            num_petals: 5,
            petal_radius: 25.0,
            petal_color: FLORALWHITE,
            bloom_duration: 5.0,
        }
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
            .fold(f32::INFINITY, f32::min)
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

            draw.ellipse()
                .xy(p)
                .wh(Vec2::new(1.0, 0.6) * petal_radius * 2.0)
                .rotate(petal_angle)
                .color(self.gene.petal_color);
        }

        let centre_size = self.bloom_progress(elapsed) * self.gene.centre_size;
        draw.ellipse()
            .xy(self.pos)
            .wh(Vec2::splat(centre_size))
            .color(self.gene.petal_color);

        draw.ellipse()
            .xy(self.pos)
            .wh(Vec2::splat(centre_size) / 1.5)
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
