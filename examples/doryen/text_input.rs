use bevy::prelude::*;
use bevy_doryen::{
    doryen::{self, Color, TextAlign},
    DoryenAppOptions, DoryenBackend, DoryenInput, RenderSystemExt, RootConsole,
};
use brltk::prelude::BRLTKPlugin;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    App::new()
        .add_plugin(BRLTKPlugin::with_backend(DoryenBackend {
            app_options: DoryenAppOptions {
                window_title: String::from("bevy_doryen subcell resolution demo"),
                ..Default::default()
            },
            ..Default::default()
        }))
        .init_resource::<TextInput>()
        .add_system(update)
        .add_doryen_render_system(render)
        .run();
}

const WHITE: Color = (255, 255, 255, 255);

#[derive(Default, Resource)]
struct TextInput {
    text: String,
    cursor: usize,
}

fn update(input: Res<DoryenInput>, mut text_input: ResMut<TextInput>) {
    // input.text returns the characters typed by the player since last update
    let text = input.text();
    if !text.is_empty() {
        text_input.text.push_str(text);
    }
    // handle backspace
    if input.key_released(doryen::ScanCode::Backspace) && !text_input.text.is_empty() {
        // convoluted way to remove the last character of the string
        // in a way that also works with utf-8 graphemes
        // where one character != one byte
        let mut graphemes = text_input.text.graphemes(true).rev();
        graphemes.next();
        text_input.text = graphemes.rev().collect();
    }
    // handle tab
    if input.key_released(doryen::ScanCode::Tab) {
        text_input.text.push_str("   ");
    }
    text_input.cursor += 1;
}

fn render(mut root_console: ResMut<RootConsole>, text_input: Res<TextInput>) {
    root_console.clear(None, None, Some(' ' as u16));
    root_console.print(
        5,
        5,
        &format!(
            "Type some text : {}{}",
            text_input.text,
            // blinking cursor
            if text_input.cursor % 25 < 12 {
                '_'
            } else {
                ' '
            }
        ),
        TextAlign::Left,
        Some(WHITE),
        None,
    );
}
