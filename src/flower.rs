use egui::ComboBox;
use std::fs;
use crate::*;
use nannou;
use nannou::color::conv::IntoLinSrgba;
use nannou::prelude::*;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use nannou::color::{Srgb, Srgba};
use nannou_egui::egui::emath::Numeric;
use nannou_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use clipboard_win::types::BOOL;
// PathBuf::from(std::env::var("APPDATA")).join("FOLDER NAME")

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct FlowerGene {
    pub(crate) size_px: f32,
    pub(crate) centre_radius_inner_prop: f32,
    pub(crate) centre_radius_outer_prop: f32,
    pub(crate) centre_dist_prop: f32,
    pub(crate) petal_width_prop: f32,
    pub(crate) num_petals: usize,
    pub(crate) bloom_duration: f32,
    pub(crate) life_span: f32,
    pub(crate) centre_colour: LinSrgba<f32>,
    pub(crate) petal_colour: LinSrgba<f32>,
}

impl Default for FlowerGene {
    fn default() -> Self {
        FlowerGene {
            size_px: 65.0,
            centre_radius_inner_prop: 0.3,
            centre_radius_outer_prop: 0.4,
            centre_dist_prop: 0.62,
            petal_width_prop: 0.62,
            num_petals: 5,
            bloom_duration: 5.0,
            life_span: 10.0,
            centre_colour: Srgb::<u8>::new(236, 178, 63).into_lin_srgba(),
            petal_colour: FLORALWHITE.into_lin_srgba(),
        }
    }
}

impl FlowerGene {
    pub fn egui(&mut self, ui: &mut Ui, flower_name: &mut String, enable_flower_death: &mut bool) {
        let spacer = 15.0;

        FlowerGene::slider(&mut self.size_px, "Flower Size:", 10.0..=200.0, ui);

        FlowerGene::slider(&mut self.centre_radius_outer_prop,"Centre Radius (Outer):",(self.centre_radius_inner_prop+0.05)..=(self.centre_radius_inner_prop+0.2), ui, );

        FlowerGene::slider(&mut self.centre_radius_inner_prop, "Centre Radius (Inner):", 0.05..=0.5, ui, );

        FlowerGene::slider(&mut self.centre_dist_prop, "Centre Distance:", 0.4..=0.65, ui, );

        FlowerGene::slider(&mut self.petal_width_prop, "Petal Width:", 0.0..=1.0, ui);

        FlowerGene::stepped_slider(&mut self.num_petals, "Petal Count:", 4..=20, ui, 2.0);

        FlowerGene::slider(&mut self.bloom_duration, "Bloom Duration:", 1.0..=10.0, ui);

        ui.add_space(spacer);

        ui.label("Enable Flower Death:");
        let state = if *enable_flower_death {
            "Enabled"
        } else {
            "Disabled"
        };
        ui.toggle_value(enable_flower_death, state);

        ui.add_space(spacer);

        FlowerGene::picker(&mut self.petal_colour, "Petal Colour:", ui);

        FlowerGene::picker(&mut self.centre_colour, "Centre Colour:", ui);

        ui.add_space(spacer);

        ui.label("Flower Name:");
        ui.text_edit_singleline(flower_name);

        let file_name = if flower_name.trim().is_empty() {
            "flower.json".to_string()
        } else {
            format!("{}.json", flower_name.trim())
        };

        let mut save_button_text = "Save Flower";
        if let Ok(path) = std::env::var("APPDATA") {
            let dir = PathBuf::from(path).join("Flower_Presets");
            let final_path = dir.join(&file_name);

            if final_path.exists() {
                save_button_text = "Update Flower";
            }

            if ui.button(save_button_text).clicked() {
                if fs::create_dir_all(&dir).is_ok() {
                    if let Ok(ser_flower) = serde_json::to_string_pretty(self) {
                        let _ = fs::write(final_path, ser_flower);
                    }
                }
            }
        }

        ui.add_space(spacer);

        let mut selected_file: Option<String> = None;
        let flower_files = load_flower_presets();

        ui.label("Load from Saved Preset:");
        ComboBox::from_label("")
            .selected_text("Select...")
            .show_ui(ui, |files| {
                for file in flower_files {
                    if files.selectable_label(false, &file).clicked() {
                        selected_file = Some(file)
                    }
                }
            });

        if let Some(file) = selected_file {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let path = PathBuf::from(appdata).join("Flower_Presets").join(&file);
                if let Ok(data) = fs::read_to_string(path) {
                    if let Ok(loaded_gene) = serde_json::from_str::<FlowerGene>(&data) {
                        *self = loaded_gene;
                        *flower_name = file.trim_end_matches(".json").to_string();
                    }
                }
            }
        }
    }

    fn slider<T: Numeric>(value: &mut T, name: &str, range: RangeInclusive<T>, ui: &mut Ui) {
        ui.label(name);
        ui.add(egui::Slider::new(value, range));
    }

    fn stepped_slider<T: Numeric>(value: &mut T, name: &str, range: RangeInclusive<T>, ui: &mut Ui, step: f64, ) {
        ui.label(name);
        ui.add(egui::Slider::new(value, range).step_by(step));
    }

    fn picker(value: &mut LinSrgba, name: &str, ui: &mut Ui) {
        ui.label(name);
        let (r, g, b, _) = value
            .into_linear() // γ‑encode
            .into_format::<u8, u8>() // keep u8
            .into();

        let mut srgb8 = [r, g, b];

        if egui::color_picker::color_edit_button_srgb(ui, &mut srgb8).changed() {
            *value = Srgba::<u8>::from_components((srgb8[0], srgb8[1], srgb8[2], 255)) // still γ‑encoded
                .into_format()
                .into_linear();
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

    pub fn death_progress(&self, elapsed: f32) -> f32 {
        let x = (elapsed - self.gene.life_span) / self.gene.bloom_duration;
        1.0 - (1.0 - (1.0 - x).powi(3)).clamp(0.0, 1.0)
    }

    pub fn is_dead(&self) -> bool {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        elapsed > (self.gene.life_span + self.gene.bloom_duration)
    }

    pub fn draw(&self, draw: &Draw, current_time: &Instant, flower_death_enabled: bool) {
        let elapsed = current_time.duration_since(self.start_time).as_secs_f32();
        let scale = self.bloom_progress(elapsed) * self.gene.size_px;

        let death_progress = if flower_death_enabled {
            self.death_progress(elapsed)
        } else {
            1.0
        };

        let centre_colour = {
            let mut x = self.gene.centre_colour;
            x.alpha = death_progress;
            x
        };
        let petal_colour = {
            let mut petal_colour = self.gene.petal_colour;
            petal_colour.alpha = death_progress;
            petal_colour
        };

        let petal_distance = self.gene.centre_dist_prop;
        let petal_radius = 1.0 - self.gene.centre_dist_prop;
        let petal_width = self.gene.petal_width_prop * petal_radius;
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
                .color(petal_colour)
                .stroke(mult_colour(petal_colour, 0.8))
                .stroke_weight(2.0 * scale / 100.);
        }

        // outer centre (back)
        draw.ellipse()
            .xy(self.pos)
            .z(-2.0)
            .radius(self.gene.centre_radius_outer_prop * scale)
            .color(petal_colour)
            .stroke(mult_colour(petal_colour, 0.8))
            .stroke_weight(4.0 * scale / 100.);

        // outer centre (front)
        draw.ellipse()
            .xy(self.pos)
            .radius(self.gene.centre_radius_outer_prop * scale)
            .color(petal_colour);

        // inner centre
        draw.ellipse()
            .xy(self.pos)
            .radius(self.gene.centre_radius_inner_prop * scale)
            .color(centre_colour);
    }

    pub fn max_radius(app: &App, new_pos: Vec2, others: &[Flower]) -> f32 {
        let radius = others
            .iter()
            .map(|other| other.pos.distance(new_pos) - other.gene.size_px)
            .fold(f32::INFINITY, |acc, b| acc.min(b))
            .max(0.0)
            + 1.0;

        let border = app.window_rect();
        let left_dist: f32 = new_pos.x - border.left();
        let right_dist: f32 = border.right() - new_pos.x;
        let top_dist: f32 = border.top() - new_pos.y;
        let bottom_dist: f32 = new_pos.y - border.bottom();

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

pub fn load_flower_presets() -> Vec<String> {
    if let Ok(path) = std::env::var("APPDATA") {
        let dir = PathBuf::from(path).join("Flower_Presets");
        if let Ok(entries) = fs::read_dir(dir) {
            return entries
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();
                    path.file_name()?.to_str().map(|s| {s.to_string()})
            }).collect();
        }
    }
    vec![]
}