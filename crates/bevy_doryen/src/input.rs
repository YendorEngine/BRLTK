//! Input handling for `doryten-rs` input back to bevy.

use std::collections::HashMap;

use bevy::prelude::{App, Plugin, Resource};
use doryen_rs::{Keys, MouseButton, ScanCode};

use crate::doryen::InputApi;

/// The Bevy Doryen input plugin to handle interloping
/// input events from `doryen-rs` to `Bevy`.
#[derive(Default, Clone, Copy, Debug)]
pub struct DoryenInputPlugin;
impl Plugin for DoryenInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DoryenInput>();
    }
}

/// Provides access to the input events handled by the Doryen engine. See the
/// documentation for the [`InputApi`] type for details on what values should
/// be used with the various `key` methods.
#[derive(Default, Debug, Resource)]
pub struct DoryenInput {
    text: String,
    close_requested: bool,

    keys_down: HashMap<ScanCode, bool>,
    keys_pressed: HashMap<ScanCode, bool>,
    keys_released: HashMap<ScanCode, bool>,

    mouse_position: (f32, f32),
    mouse_buttons_down: HashMap<MouseButton, bool>,
    mouse_buttons_pressed: HashMap<MouseButton, bool>,
    mouse_buttons_released: HashMap<MouseButton, bool>,
}

impl DoryenInput {
    fn on_frame(&mut self) {
        self.text.clear();
        self.close_requested = false;

        self.keys_released.clear();
        self.keys_pressed.clear();

        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
    }

    pub(crate) fn handle_input(
        &mut self,
        mouse_button_listeners: &[MouseButton],
        input: &mut dyn InputApi,
    ) {
        self.on_frame();

        self.handle_pressed(input.keys_pressed());
        self.handle_released(input.keys_released());
        self.handle_mouse(mouse_button_listeners, input);

        self.text = input.text();
        self.mouse_position = input.mouse_pos();
        self.close_requested = input.close_requested();
    }

    /// Returns the current status of the given key (true if currently pressed).
    pub fn key(&self, scan_code: ScanCode) -> bool {
        matches!(self.keys_down.get(&scan_code), Some(&true))
    }

    /// Returns true if the given key was pressed since the last update.
    pub fn key_pressed(&self, scan_code: ScanCode) -> bool {
        matches!(self.keys_pressed.get(&scan_code), Some(&true))
    }

    /// Returns an iterator over all the keys that were pressed since the last
    /// update in no particular order.
    pub fn keys_pressed(&self) -> impl Iterator + '_ {
        self.keys_pressed.iter().filter(|&(_, &v)| v)
    }

    /// Returns true if the given key was released since the last update.
    pub fn key_released(&self, scan_code: ScanCode) -> bool {
        matches!(self.keys_released.get(&scan_code), Some(&true))
    }

    /// Returns an iterator over all the keys that were released since the last
    /// update in no particular order.
    pub fn keys_released(&self) -> impl Iterator + '_ {
        self.keys_released.iter().filter(|&(_, &v)| v)
    }

    /// Characters typed since last update.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the current status of the given mouse button (true if
    /// currently pressed).
    pub fn mouse_button(&self, mouse_button: MouseButton) -> bool {
        matches!(self.mouse_buttons_down.get(&mouse_button), Some(&true))
    }

    /// Returns true if the given mouse button was pressed since the last
    /// update.
    pub fn mouse_pressed(&self, mouse_button: MouseButton) -> bool {
        matches!(self.mouse_buttons_pressed.get(&mouse_button), Some(&true))
    }

    /// Returns true if the given mouse button was released since the last
    /// update.
    pub fn mouse_released(&self, mouse_button: MouseButton) -> bool {
        matches!(self.mouse_buttons_released.get(&mouse_button), Some(&true))
    }

    /// Returns the current mouse position in console cells coordinates.
    /// The decimal part of the value indicates sub-cell location.
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    /// Whether the window close button has been activated.
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

impl DoryenInput {
    fn handle_pressed(&mut self, pressed: Keys<'_>) {
        pressed.for_each(|k| {
            if let Some(v) = self.keys_pressed.get_mut(k) {
                *v = true;
            } else {
                self.keys_pressed.insert(*k, true);
            }

            if let Some(v) = self.keys_down.get_mut(k) {
                *v = true;
            } else {
                self.keys_down.insert(*k, true);
            }
        });
    }

    fn handle_released(&mut self, released: Keys<'_>) {
        released.for_each(|k| {
            if let Some(v) = self.keys_released.get_mut(k) {
                *v = true;
            } else {
                self.keys_released.insert(*k, true);
            }
            if let Some(v) = self.keys_down.get_mut(k) {
                *v = false;
            } else {
                self.keys_down.insert(*k, false);
            }
        });
    }

    fn handle_mouse(&mut self, mouse_button_listeners: &[MouseButton], input: &mut dyn InputApi) {
        mouse_button_listeners.iter().for_each(|&mouse_button| {
            // On Down
            if input.mouse_button_pressed(mouse_button) {
                self.mouse_buttons_pressed.insert(mouse_button, true);
                self.mouse_buttons_down.insert(mouse_button, true);
            }

            // On Release
            if input.mouse_button_released(mouse_button) {
                self.mouse_buttons_down.insert(mouse_button, false);
                self.mouse_buttons_pressed.insert(mouse_button, false);
                self.mouse_buttons_released.insert(mouse_button, true);
            }
        });
    }
}
