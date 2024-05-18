use eframe::egui;
use egui::Color32;

use crate::{
    interval::{Direction, Interval},
    note_range::NoteRange,
    notes::Note,
};

#[derive(Default)]
pub struct IntervalTrainer {
    scene: Scene,
}

impl IntervalTrainer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn set_scene(&mut self, new_scene: Scene) {
        self.scene = new_scene;
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
enum Scene {
    #[default]
    Hello,
    CoolSVG,
}

impl eframe::App for IntervalTrainer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click me").clicked() {
                match self.scene {
                    Scene::Hello => self.set_scene(Scene::CoolSVG),
                    Scene::CoolSVG => self.set_scene(Scene::Hello),
                }
            }

            match self.scene {
                Scene::Hello => {
                    ui.label("Hello");
                }
                Scene::CoolSVG => {
                    ui.label("Cool SVG");
                    ui.add(
                        egui::Image::new(egui::include_image!("assets/svg/Eb4_treble.svg"))
                            .fit_to_exact_size([1000.0, 500.0].into())
                            .bg_fill(Color32::WHITE),
                    );
                }
            }
        });
    }
}

fn choose_notes(range: &NoteRange) -> (Note, Note) {
    let interval = Interval::get_random_diatonic();
    let direction = Direction::Up;

    let new_range = match direction {
        Direction::Up => range.crop_top(interval.size_i8()),
        Direction::Down => range.crop_bottom(interval.size_i8()),
    };

    let reference = new_range.rand();
    (reference, reference.up(interval))
}
