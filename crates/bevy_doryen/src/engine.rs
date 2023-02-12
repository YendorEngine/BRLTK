use bevy::{
    app::AppExit,
    ecs::event::ManualEventReader,
    prelude::{Events, Schedule, Stage},
};
use doryen_rs::{Console, DoryenApi, Engine, MouseButton, UpdateEvent};

use crate::*;

#[derive(Default)]
pub struct DoryenEngine {
    pub(crate) resize_mode: ResizeMode,
    pub(crate) bevy_app: bevy::prelude::App,
    pub(crate) swap_console: Option<Console>,
    pub(crate) previous_screen_size: (u32, u32),
    pub(crate) previous_console_size: (u32, u32),
    pub(crate) mouse_button_listeners: Vec<MouseButton>,
    pub(crate) app_exit_event_reader: ManualEventReader<AppExit>,
    pub(crate) set_font_path_event_reader: ManualEventReader<SetFontPath>,
}

impl DoryenEngine {
    #[inline]
    fn take_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take ownership of the Doryen root console
        swap(api.con(), self.swap_console.as_mut().unwrap());

        // Insert it into the DoryenRootConsole resource
        let mut doryen_root_console = self
            .bevy_app
            .world
            .get_resource_mut::<RootConsole>()
            .unwrap();
        doryen_root_console.0 = self.swap_console.take();
    }

    #[inline]
    fn restore_root_console_ownership(&mut self, api: &mut dyn DoryenApi) {
        use std::mem::swap;

        // Take the root console out of the DoryenRootConsole resource
        let mut doryen_root_console = self
            .bevy_app
            .world
            .get_resource_mut::<RootConsole>()
            .unwrap();
        self.swap_console = doryen_root_console.0.take();

        // Hand ownership of the Doryen root console back to Doryen
        swap(api.con(), self.swap_console.as_mut().unwrap());
    }

    #[inline]
    fn take_doryen_render_schedule(&mut self) -> Schedule {
        let mut doryen_render_systems = self
            .bevy_app
            .world
            .get_resource_mut::<DoryenRenderSystems>()
            .unwrap();
        doryen_render_systems.0.take().unwrap()
    }

    #[inline]
    fn restore_doryen_render_schedule(&mut self, doryen_render_schedule: Schedule) {
        let mut doryen_render_systems = self
            .bevy_app
            .world
            .get_resource_mut::<DoryenRenderSystems>()
            .unwrap();
        doryen_render_systems.0.replace(doryen_render_schedule);
    }

    #[inline]
    fn handle_input(&mut self, api: &mut dyn DoryenApi) {
        let input = api.input();
        let mut doryen_input = self
            .bevy_app
            .world
            .get_resource_mut::<DoryenInput>()
            .unwrap();
        doryen_input.handle_input(&self.mouse_button_listeners, input);
    }
}

impl Engine for DoryenEngine {
    fn init(&mut self, _api: &mut dyn DoryenApi) {
        self.bevy_app.init_resource::<DoryenInput>();
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let mut doryen_fps_info = self.bevy_app.world.get_resource_mut::<FpsInfo>().unwrap();
        doryen_fps_info.fps = api.fps();
        doryen_fps_info.average_fps = api.average_fps();

        self.handle_input(api);

        self.take_root_console_ownership(api);
        self.bevy_app.update();
        self.restore_root_console_ownership(api);

        // Process the latest SetFontPath event
        let doryen_set_font_path_events = self
            .bevy_app
            .world
            .get_resource_mut::<Events<SetFontPath>>()
            .unwrap();
        if let Some(doryen_set_font_path) = self
            .set_font_path_event_reader
            .iter(&doryen_set_font_path_events)
            .last()
        {
            api.set_font_path(&format!("assets/{}", doryen_set_font_path.0.as_ref()));
        }

        if let Some(app_exit_events) = self.bevy_app.world.get_resource_mut::<Events<AppExit>>() {
            if self
                .app_exit_event_reader
                .iter(&app_exit_events)
                .last()
                .is_some()
            {
                return Some(UpdateEvent::Exit);
            }
        }

        None
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        self.take_root_console_ownership(api);

        let wc = self.bevy_app.world.cell();
        let mut rs = wc.get_resource_mut::<RenderState>().unwrap();
        if rs.0 {
            for f in &rs.1 {
                f(&wc);
            }
            rs.0 = false;
        }
        drop(rs);
        drop(wc);

        let mut doryen_render_schedule = self.take_doryen_render_schedule();
        doryen_render_schedule.run(&mut self.bevy_app.world);
        self.restore_doryen_render_schedule(doryen_render_schedule);

        self.restore_root_console_ownership(api);
    }

    fn resize(&mut self, api: &mut dyn DoryenApi) {
        let (previous_width, previous_height) = self.previous_screen_size;
        let (new_width, new_height) = api.get_screen_size();

        let mut resized_events = self
            .bevy_app
            .world
            .get_resource_mut::<Events<Resized>>()
            .unwrap();
        let resized = Resized {
            previous_width,
            previous_height,
            new_width,
            new_height,
        };
        resized_events.send(resized);

        match self.resize_mode {
            ResizeMode::Nothing => (),
            ResizeMode::Automatic => {
                let (previous_console_width, previous_console_height) = self.previous_console_size;

                let w_ratio = previous_width / previous_console_width;
                let h_ratio = previous_height / previous_console_height;

                let new_console_width = new_width / w_ratio;
                let new_console_height = new_height / h_ratio;
                api.con().resize(new_console_width, new_console_height);
            }
            ResizeMode::Callback(callback) => {
                self.take_root_console_ownership(api);
                callback(
                    &mut self.bevy_app.world.get_resource_mut().unwrap(),
                    resized,
                );
                self.restore_root_console_ownership(api);
            }
        }

        self.previous_screen_size = (new_width, new_height);
        self.previous_console_size = api.con().get_size();
    }
}
