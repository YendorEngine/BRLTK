use bevy::prelude::*;
use bevy_doryen::{
    doryen::TextAlign, DoryenAppOptions, DoryenBackend, DoryenImage, RenderSystemExt, RootConsole,
};
use brltk::prelude::BRLTKPlugin;

fn main() {
    App::new()
        .add_plugin(BRLTKPlugin::with_backend(DoryenBackend {
            app_options: DoryenAppOptions {
                window_title: String::from("bevy_doryen subcell resolution demo"),
                ..Default::default()
            },
            ..Default::default()
        }))
        .init_resource::<SkullImage>()
        .add_doryen_render_system(render)
        .run();
}

#[derive(Resource)]
struct SkullImage {
    skull: DoryenImage,
}

impl Default for SkullImage {
    fn default() -> Self {
        Self {
            skull: DoryenImage::new("skull.png"),
        }
    }
}

fn render(mut root_console: ResMut<RootConsole>, mut skull: ResMut<SkullImage>) {
    root_console.clear(None, Some((0, 0, 0, 255)), None);
    skull
        .skull
        .blit_2x(&mut root_console, 23, 0, 0, 0, None, None, None);
    root_console.print(
        40,
        4,
        "Those pixels\nare twice smaller\nthan a console cell.\nMagic!",
        TextAlign::Center,
        Some((0, 0, 0, 255)),
        None,
    );
}
