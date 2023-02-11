use bevy::prelude::*;
use bevy_doryen::{doryen::TextAlign, DoryenAppOptions, DoryenBackend, RenderSystemExt, RootConsole};
use brltk_common::Backend;
const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

fn main() {
    let mut app = App::new();
    DoryenBackend {
        app_options: DoryenAppOptions {
            console_width: CONSOLE_WIDTH,
            console_height: CONSOLE_HEIGHT,
            screen_width: CONSOLE_WIDTH * 16,
            screen_height: CONSOLE_HEIGHT * 16,
            window_title: String::from("doryen-rs unicode demo"),
            font_path: String::from("unicode_16x16.png"),
            ..Default::default()
        },
        ..Default::default()
    }
    .build(&mut app);

    app.add_doryen_render_system(render).run();
}

fn render(mut root_console: ResMut<RootConsole>) {
    root_console.clear(Some((32, 16, 0, 255)), Some((255, 240, 224, 255)), None);
    root_console.area(
        5,
        5,
        30,
        15,
        Some((255, 255, 255, 255)),
        Some((0, 0, 0, 255)),
        Some(' ' as u16),
    );
    root_console.print(20, 8, "こんにちは!", TextAlign::Center, None, None);
    root_console.print(20, 10, "真棒!", TextAlign::Center, None, None);
    root_console.print(20, 12, "классно", TextAlign::Center, None, None);
    root_console.print(20, 14, "Φοβερός!", TextAlign::Center, None, None);
    root_console.print(20, 16, "Ça, c'est énorme!", TextAlign::Center, None, None);
}
