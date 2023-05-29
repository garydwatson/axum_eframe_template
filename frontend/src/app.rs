use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::rc::Rc;

use egui::style::Interaction;
use shared_types::CreateUser;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    nagel: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    stuff: Rc<RefCell<String>>,

    current_tab: CurrentTab,

}

#[derive(serde::Deserialize, serde::Serialize)]
enum CurrentTab {
    CreateUser,
    DeleteUser,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            nagel: "Hello world!".to_owned(),
            value: 2.9,
            stuff: Rc::new(RefCell::new(String::new())),
            current_tab: CurrentTab::CreateUser,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { nagel: shabel, value, stuff, current_tab } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::right("tabs").show(ctx, |ui| {
            ui.heading("Tabs");
            if ui.button("Create User").clicked() {
                *current_tab = CurrentTab::CreateUser;
            }
            if ui.button("Create User").clicked() {
                *current_tab = CurrentTab::DeleteUser;
            }

        });

        egui::SidePanel::left("side_panel").exact_width(300.0).show(ctx, |ui| {
            ui.heading("Side Panel");

            match current_tab {
                CurrentTab::CreateUser => {

                    ui.horizontal(|ui| {
                        ui.label("Write something: ");
                        ui.text_edit_singleline(shabel);
                    });

                    ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                    if ui.button("Increment").clicked() {
                        *value += 1.0;
                    }

                    if ui.button("Create User").clicked() {
                        //let client = reqwest::Client::new();


                        let cloned_stuff = stuff.clone();
                        let future = async move {
                            let client = reqwest_wasm::Client::new();
                            let baseurl = web_sys::window().unwrap().origin();

                            let user = 
                                shared_types::CreateUser 
                                    { 
                                        email: "smotchy@smotch.com".to_string(), 
                                        password: "passywassy".to_string(), 
                                    };

                            ////*shabel = format!("yoyoyo = {:#?} - baseurl {}", serde_json::to_string(&user).unwrap(), baseurl.to_owned());
                            let r = 
                                client
                                    .post(format!("{}/users", baseurl))
                                    .body(serde_json::to_string(&user).unwrap())
                                    .header("Content-Type", "application/json")
                                    .send().await.unwrap();
                            let rr = r.text().await.unwrap();

                            cloned_stuff.replace(format!("{}\n{}", (*cloned_stuff).borrow().clone(), rr));
                        };
                        //futures::executor::block_on(future);
                        wasm_bindgen_futures::spawn_local(future);
                        
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("powered by ");
                            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                            ui.label(" and ");
                            ui.hyperlink_to(
                                "eframe",
                                "https://github.com/emilk/egui/tree/master/crates/eframe",
                            );
                            ui.label(".");
                        });
                    });





















                },
                CurrentTab::DeleteUser => {


                        ui.label("Ok nice");
                        ui.text_edit_singleline(shabel);

                        if ui.button("Increment").clicked() {
                            *value += 1.0;
                        }



                },
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.text_edit_singleline(shabel);

////            ui.text_edit_multiline(&mut (**stuff).borrow().clone());

            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
////                ui.style_mut().wrap = Some(false);
                ui.add(egui::TextEdit::multiline(&mut (**stuff).borrow().clone().as_str()).desired_width(f32::INFINITY));
////                ui.text_edit_multiline(&mut (**stuff).borrow().clone().as_str());
////                for line in (**stuff).borrow().clone().lines() {
////                    ui.label(line);
////                }
            });


            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
