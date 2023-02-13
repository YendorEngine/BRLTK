use bevy::prelude::*;
use bevy_ascii_terminal::{ascii_terminal::*, BevyAsciiTerminalBackend, TerminalBundleBuilder};
use brltk::prelude::BRLTKPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BRLTKPlugin::with_backend(
            BevyAsciiTerminalBackend::default().with_terminal(
                TerminalBundleBuilder::new([20, 1]).with_border(Border::single_line()),
            ),
        ))
        .add_startup_system(start)
        .add_system(hello_world)
        .run();
}

fn start(mut q: Query<&mut Terminal>) {
    let mut term = q.single_mut();
    term.put_string([0, 0], "Press spacebar".bg(Color::LIME_GREEN));
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            println!("Hello, world! (from the terminal)");
            term.clear();
            term.put_char([0, 0], 'H'.fg(Color::BLUE).bg(Color::GREEN));
            term.put_char([1, 0], 'e'.fg(Color::BLUE).bg(Color::WHITE));
            term.put_char([2, 0], 'l'.fg(Color::GREEN).bg(Color::BLUE));
            term.put_char([3, 0], 'l'.fg(Color::RED).bg(Color::GREEN));
            term.put_char([4, 0], 'o'.fg(Color::GREEN).bg(Color::GRAY));

            term.put_string([6, 0], "World!");
        }
    }
}
