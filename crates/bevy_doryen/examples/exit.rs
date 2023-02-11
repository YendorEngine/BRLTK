use bevy::{app::AppExit, prelude::*};
use bevy_doryen::{
    doryen::{Color, TextAlign},
    DoryenAppOptions, DoryenBackend, DoryenInput, RenderSystemExt, RootConsole,
};
use brltk_common::Backend;

const WHITE: Color = (255, 255, 255, 255);

#[derive(Default, Resource)]
struct CloseRequested(bool);

fn main() {
    let mut app = App::new();

    DoryenBackend {
        app_options: DoryenAppOptions {
            intercept_close_request: true,
            window_title: String::from("alpha test"),
            ..Default::default()
        },
        ..Default::default()
    }
    .build(&mut app);

    app.init_resource::<CloseRequested>()
        .add_system(process_input)
        .add_doryen_render_system(render)
        .run();
}

fn process_input(
    input: Res<DoryenInput>,
    mut close_requested: ResMut<CloseRequested>,
    mut app_exit: EventWriter<AppExit>,
) {
    if close_requested.0 {
        if input.key(doryen_rs::ScanCode::Y) {
            app_exit.send(AppExit);
        } else if input.key(doryen_rs::ScanCode::N) {
            close_requested.0 = false;
        }
    } else if input.key(doryen_rs::ScanCode::Escape) || input.close_requested() {
        close_requested.0 = true;
    }
}

fn render(mut root_console: ResMut<RootConsole>, close_requested: Res<CloseRequested>) {
    root_console.clear(None, None, Some(' ' as u16));
    if close_requested.0 {
        root_console.print(
            5,
            5,
            "Exit game ? (press Y or N)",
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    } else {
        root_console.print(
            5,
            5,
            "Press ESC to exit",
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    }
}
