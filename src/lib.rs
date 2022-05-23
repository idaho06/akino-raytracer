#![warn(clippy::all, rust_2018_idioms)]
//use image::{Rgba, RgbaImage};

mod app;
pub use app::RaytracerApp;

pub mod canvas {
    use image::{Rgba, RgbaImage};

    pub struct Canvas {
        img: RgbaImage,
        tex: Option<egui::TextureHandle>,
    }

    impl Canvas {
        pub fn new(width: usize, height: usize) -> Canvas {
            Canvas {
                img: RgbaImage::new(width as u32, height as u32),
                tex: None,
            }
        }

        pub fn put_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
            let size = [self.img.width() as i32, self.img.height() as i32];
            let x = (size[0] / 2) + x;
            let y = (size[1] / 2) - y;

            self.img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
        }

        pub fn update_tex(&mut self, ctx: &egui::Context) -> &mut egui::TextureHandle {
            let size = [self.img.width() as usize, self.img.height() as usize];
            let pixels = self.img.as_flat_samples();
            self.tex.insert(ctx.load_texture(
                "render",
                egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
            ))
        }

        pub fn get_tex(&mut self, ctx: &egui::Context) -> &mut egui::TextureHandle {
            self.tex
                .get_or_insert_with(|| ctx.load_texture("default", egui::ColorImage::example()))
        }
    }
}

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(RaytracerApp::new(cc))))
}
