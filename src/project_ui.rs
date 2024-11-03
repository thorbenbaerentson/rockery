use egui::{ CollapsingHeader, Ui };
use log::info;
use rockery_datamodel::prelude::*;
use eframe::egui;

pub fn draw_service_ui(ui: &mut Ui, ser : &mut RockeryService) {
    CollapsingHeader::new(ser.get_name())
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let mut service_type = ser.get_service_type();
                let type_name = service_type.to_string();
                egui::ComboBox::from_label("Service type")
                    .selected_text(type_name)
                    .show_ui(ui,|ui| {
                        ui.selectable_value(&mut service_type, &RockeryServiceType::Generic, RockeryServiceType::Generic.to_string());
                        ui.selectable_value(&mut service_type, &RockeryServiceType::Rest, RockeryServiceType::Rest.to_string());
                        ui.selectable_value(&mut service_type, &RockeryServiceType::Soap, RockeryServiceType::Soap.to_string());
                    });
                    // Todo: Set service value
            })
        });
}

pub fn draw_project_ui(ui: &mut Ui, p : &mut RockeryProject) {
    CollapsingHeader::new(p.get_name())
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(p.get_name());
                if p.is_dirty() && ui.button("Save").on_hover_text(format!("Save '{}'", p.get_name())).clicked() {
                    info!("Saving a project");
                    // match p.save() {
                    //     Ok(_) => { info!("Project saved"); },
                    //     Err(e) => { info!("Could not save project: {}", e); },
                    // };
                }
            });

            for pro in p.get_services().values_mut() {
                draw_service_ui(ui,pro);
            }
        });
}