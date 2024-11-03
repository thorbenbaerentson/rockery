use std::path::PathBuf;
use std::collections::HashMap;

use eframe::egui;

use egui_file_dialog::FileDialog;
use log::{error, info};
use rockery_datamodel::prelude::*;

use crate::draw_project_ui;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct RockeryApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,

    projects : HashMap<String, RockeryProject>,
}

impl Default for RockeryApp {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            selected_file: None,

            projects : HashMap::new(),
        }
    }
}

impl RockeryApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for RockeryApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Update file dialog. 
            self.file_dialog.update(ctx);

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if ui.button("New").on_hover_text("Add a new project").clicked() {
                        let mut name = "Unnamed".to_owned();
                        let mut counter = 1;
                        while self.projects.contains_key(&name) {
                            name = format!("{}_{}", name, counter);
                            counter += 1;
                        }

                        self.projects.insert(name.clone(), RockeryProject::new(&name));
                    }

                    if ui.button("Load").on_hover_text("Load a project from disk").clicked() {
                        // Actually loading the file is done down below. Check for the comment.
                        self.file_dialog.select_file();
                    }

                    if ui.button("Save").on_hover_text("Save all projects").clicked() {
                        for pro in self.projects.values_mut() {
                            // match pro.save() {
                            //     Ok(_) => {
                            //         info!("{} saved.", pro.get_name())
                            //     },
                            //     Err(e) => {
                            //         error!("Could not save project '{}'. Error: {}", pro.get_name(), e)
                            //     },
                            // }
                        }
                    }

                    if !is_web && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            });

            //  Loading is done here!
            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());

                match RockeryProject::load(path) {
                    Ok(pro) => {
                        if self.projects.contains_key(pro.get_name()) {
                            error!("Could not load {}, because a project with the same name already exists. Project names must be unique.", pro.get_name());
                        } else {
                            info!("Project: {} loaded.", pro.get_name());
                            self.projects.insert(pro.get_name().to_owned(), pro);
                        }
                    }
                    Err(e) => {
                        error!("Could not load project. Error: {}", e);
                    }
                }
            }
        });

        egui::SidePanel::left("Projects")
            .min_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                for p in &mut self.projects.values_mut() {
                    draw_project_ui(ui, p);
                }
        });

        egui::TopBottomPanel::bottom("Console")
            .min_height(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("This should become the console.");
        });

        egui::CentralPanel::default().show(ctx, |_ui| {

        });
    }
}

