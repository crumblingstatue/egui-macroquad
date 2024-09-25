//! # egui bindings for macroquad
//!
//! Macroquad integration for egui.
//!
//! [Web demo.](https://optozorax.github.io/egui-macroquad/)
//!
//! # Usage
//!
//! - Create a new integration with [`EguiMqInteg::new`].
//! - Call [`EguiMqInteg::ui`] to do the egui ui
//! - Call [`EguiMqInteg::draw`] to draw the ui
//!
//! Here is the small example on how to use this library:
//! ```rust
//! use macroquad::prelude::*;
//!
//! #[macroquad::main("egui with macroquad")]
//! async fn main() {
//!     let mut integ = egui_macroquad::EguiMqInteg::new();
//!     loop {
//!         clear_background(WHITE);
//!
//!         // Process keys, mouse etc.
//!
//!         integ.ui(|_, egui_ctx| {
//!             egui::Window::new("egui ❤ macroquad")
//!                 .show(egui_ctx, |ui| {
//!                     ui.label("Test");
//!                 });
//!         });
//!
//!         // Draw things before egui
//!
//!         integ.draw();
//!
//!         // Draw things after egui
//!
//!         next_frame().await;
//!     }
//! }
//! ```
//!
//! # Building
//!
//! Building for native and for web works just as in `macroquad`. You can read about it [here](https://github.com/not-fl3/miniquad/#building-examples). Or you could look at building example at [egui-miniquad](https://github.com/not-fl3/egui-miniquad).
//!
//! But for wasm you will need to include two more `.js` files, which is plugins for quads, instruction is written [here](https://github.com/optozorax/quad-url).

#![warn(missing_docs)]

use egui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;

pub use egui;
pub use macroquad;

/// Macroquad integration for egui
pub struct EguiMqInteg {
    egui_mq: EguiMq,
    input_subscriber_id: usize,
}

impl Default for EguiMqInteg {
    fn default() -> Self {
        Self::new()
    }
}

impl EguiMqInteg {
    /// Create a new integration
    pub fn new() -> Self {
        Self {
            egui_mq: EguiMq::new(unsafe { get_internal_gl() }.quad_context),
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
        }
    }
    /// Do the egui ui update
    pub fn ui<F>(&mut self, f: F)
    where
        F: FnOnce(&mut dyn mq::RenderingBackend, &egui::Context),
    {
        let gl = unsafe { get_internal_gl() };
        macroquad::input::utils::repeat_all_miniquad_input(self, self.input_subscriber_id);

        self.egui_mq.run(gl.quad_context, f);
    }
    /// Draw the ui
    pub fn draw(&mut self) {
        let mut gl = unsafe { get_internal_gl() };
        // Ensure that macroquad's shapes are not goint to be lost, and draw them now
        gl.flush();
        self.egui_mq.draw(gl.quad_context);
    }
}

impl mq::EventHandler for EguiMqInteg {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}
