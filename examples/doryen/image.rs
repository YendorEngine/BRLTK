use bevy::prelude::*;
use bevy_doryen::{DoryenAppOptions, DoryenBackend, DoryenImage, RenderSystemExt, RootConsole};
use brltk::prelude::BRLTKPlugin;

fn main() {
    App::new()
        .add_plugin(BRLTKPlugin::with_backend(DoryenBackend {
            app_options: DoryenAppOptions {
                window_title: String::from("bevy_doryen image demo"),
                ..Default::default()
            },
            ..Default::default()
        }))
        .init_resource::<SkullImage>()
        .add_system(update)
        .add_doryen_render_system(render)
        .run();
}

#[derive(Resource)]
struct SkullImage {
    angle: f32,
    scale_time: f32,
    skull: DoryenImage,
}

impl Default for SkullImage {
    fn default() -> Self {
        Self {
            angle: 0.0,
            scale_time: 0.0,
            skull: DoryenImage::new("skull.png"),
        }
    }
}

fn update(mut skull: ResMut<SkullImage>) {
    skull.angle += 0.01;
    skull.scale_time += 0.01;
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: ResMut<SkullImage>) {
    let root_console = &mut **root_console;
    let skull = &mut *skull;
    let scale = skull.scale_time.cos();
    root_console.clear(None, Some((0, 0, 0, 255)), None);
    skull.skull.blit_ex(
        root_console,
        (root_console.get_width() / 2) as f32,
        (root_console.get_height() / 2) as f32,
        scale,
        scale,
        skull.angle,
        None,
    );
}
