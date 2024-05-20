use eframe::egui;
use egui::Color32;

use crate::{
    interval::{Direction, Interval},
    note_range::NoteRange,
    notes::Note,
};

pub struct IntervalTrainer {
    scene: Scene,
}

impl IntervalTrainer {
    pub fn new() -> Self {
        println!("hello from IntervalTrainer constructor");

        let slf = Self { scene: Scene::Hello };

        slf
    }

    pub fn run(&self) {
        println!("hi");
    }

    pub fn new_from_egui_context(cc: &eframe::CreationContext<'_>) -> Self {
        println!("hello from IntervalTrainer constructor");

        let slf = Self { scene: Scene::Hello };

        slf
    }

    fn set_scene(&mut self, new_scene: Scene) {
        self.scene = new_scene;
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Scene {
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
                        egui::Image::new(egui::include_image!("assets/svg/A4_treble.svg"))
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
