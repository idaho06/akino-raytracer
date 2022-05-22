use image::{Rgba, RgbaImage};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/*
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
*/
pub struct RaytracerApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    //#[serde(skip)]
    value: f32,
    //#[serde(skip)]
    img: RgbaImage,
    render_result: Option<egui::TextureHandle>,
    update_render_result: bool,

    // pixel values
    red: u8,
    green: u8,
    blue: u8,
    pixelx: usize,
    pixely: usize,
}

impl Default for RaytracerApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: String::from("Hello, Akino!"),
            value: 2.7,
            img: RgbaImage::new(800, 600),
            render_result: None,
            update_render_result: true,
            red: 255,
            green: 255,
            blue: 255,
            pixelx: 0,
            pixely: 0,
        }
    }
}

impl RaytracerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */
        Default::default()
    }
}

impl eframe::App for RaytracerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            img,
            render_result,
            update_render_result,
            red,
            green,
            blue,
            pixelx,
            pixely,
        } = self; // This removes the need to write "self.whatever" everytime. And apparently everything is mut

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            //ui.heading("Side Panel");

            /*
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });
            */
            /*
            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }
            */

            ui.add(egui::Slider::new(red, 0..=255).text("Red"));
            ui.add(egui::Slider::new(green, 0..=255).text("Green"));
            ui.add(egui::Slider::new(blue, 0..=255).text("Blue"));
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(pixelx));
            });
            ui.horizontal(|ui| {
                ui.label("y: ");
                ui.add(egui::DragValue::new(pixely));
            });

            ui.horizontal(|ui| {
                if ui.button("Add pixel").clicked() {
                    img.put_pixel(
                        *pixelx as u32,
                        *pixely as u32,
                        Rgba([*red, *green, *blue, 255]),
                    );
                    *update_render_result = true;
                }
                if ui.button("Set background").clicked() {
                    //
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("made by ");
                    ui.hyperlink_to("@idaho06", "https://github.com/idaho06");
                    ui.label(" from ");
                    ui.hyperlink_to("AkinoSoft", "https://akinosoft.itch.io/");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let size = [img.width() as usize, img.height() as usize];
            let pixels = img.as_flat_samples();

            let render_texture_handler: &egui::TextureHandle;
            if *update_render_result == true {
                render_texture_handler = render_result.insert(ui.ctx().load_texture(
                    "render",
                    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                ));
                *update_render_result = false;
            } else {
                render_texture_handler = render_result.get_or_insert_with(|| {
                    ui.ctx()
                        .load_texture("default", egui::ColorImage::example())
                });
            }

            // let render_result: &egui::TextureHandle = render_result.get_or_insert_with(|| {
            //     println!("Updating render_result");
            //     *update_render_result = false;
            //     ui.ctx()
            //         .load_texture(
            //             "render",
            //             egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
            // });

            ui.image(render_texture_handler, render_texture_handler.size_vec2());

            //ui.heading("Render result");
            /*
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            */
            egui::warn_if_debug_build(ui);
        });

        /*
        egui::Window::new("Render result")
        .resizable(true)
        .scroll2([true,true])
        .show(ctx, |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });
        */
    }
}
