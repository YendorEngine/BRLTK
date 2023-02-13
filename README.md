# Bevy Roguelike Toolkit (BRLTK)

An opinionated rougelike toolkit for bevy

BRLTK lets you use [Bevy](https://github.com/bevyengine/bevy) with various roguelike libraries. Currently, it supports [doryen-rs](https://github.com/jice-nospam/doryen-rs) and [bracket-lib](https://github.com/amethyst/bracket-lib).

<div align="center">
    <a href="https://blog.rust-lang.org/2023/02/09/Rust-1.67.1.html">
        <img src="https://img.shields.io/badge/supports%20rust-1.67+-blue" alt="rustc"/>
    </a>
    <a href="https://crates.io/crates/brltk">
        <img src="https://img.shields.io/crates/v/brltk" alt="Crates.io"/>
    </a>
    <a href="https://crates.io/crates/brltk">
        <img src="https://img.shields.io/crates/l/brltk" alt="License"/>
    </a>
    <a href="https://crates.io/crates/brltk">
        <img src="https://img.shields.io/crates/d/brltk" alt="Downloads"/>
    </a>
    <a href="https://docs.rs/brltk">
        <img src="https://docs.rs/brltk/badge.svg" alt="Docs.io"/>
    </a>
    <a href="https://github.com/YendorEngine/BRLTK/actions">
        <img src="https://github.com/bevyengine/bevy/workflows/CI/badge.svg" alt="CI"/>
    </a>
    <a href="https://discord.gg/vHaseWJp">
        <img src="https://img.shields.io/discord/1027393534627692645?label=Discord&logo=Discord&style=plastic" alt="Discord"/>
    </a>
</div>

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release

Compatibility of `BRLTK` versions:
| `BRLTK` | `bevy` |
| :-- | :-- |
| `0.1.0` | `0.9` |

## Getting Started

Using `doryen-rs` backend:

```rust
App::build()
    // Add the `BRLTKPlugin` to Bevy.
    .add_plugin(BRLTKPlugin::with_backend(DoryenBackend {
        // here are all the available options.
        // better practice is to use default values (see other examples)
        app_options: DoryenAppOptions {
            console_width: CONSOLE_WIDTH,
            console_height: CONSOLE_HEIGHT,
            screen_width: CONSOLE_WIDTH * 8,
            screen_height: CONSOLE_HEIGHT * 8,
            window_title: String::from("my roguelike"),
            font_path: String::from("terminal_8x8.png"),
            vsync: true,
            fullscreen: false,
            show_cursor: true,
            resizable: true,
            intercept_close_request: false,
            max_fps: 60,
        },
        ..Default::default()
    }))
    // Add your Bevy systems like usual. Excluding startup systems, which
    // only run once, these systems are run during Doryen's update phase;
    // i.e. 60 times per second.
    .add_startup_system(init)
    .add_system(input)
    // The `RenderSystemExtensions` trait lets you add systems that should
    // be run during Doryen's render phase.
    .add_doryen_render_system(render)
    .run();
```

Using `bracket-lib` backend:

⚠️(Default Plugins are required for the `bracket-lib` backend)⚠️

```rust
App::build()
    .add_plugins(DefaultPlugins)
    // Add the `BRLTKPlugin` to Bevy.
    .add_plugin(BRLTKPlugin::with_backend(
        BracketLibBackend::simple_80x50()
            .with_named_color("blue", BLUE)
            .with_named_color("pink", Color::PINK),
    ))
    .insert_resource(State {
        y: 0,
        going_down: true,
    })
    .add_system(tick)
    .run();
```

Once set up, you can quickly try out the [examples](https://github.com/YendorEngine/BRLTK/tree/master/examples) by cloning this repo and running the following commands:

```sh
# Switch to the correct version (latest release, default is main development branch)
git checkout latest

# Runs the "basic_doryen" example
cargo run --example basic_doryen

or
# Runs the "basic_bracket" example
cargo run --example basic_bracket
```

## Libraries Used

Bevy is only possible because of the hard work put into these foundational technologies:

- [bevy](https://github.com/bevyengine/bevy): a refreshingly simple data-driven game engine built in Rust
- [doryen-rs](https://github.com/jice-nospam/doryen-rs): Ascii roguelike library in rust with native and wasm support
- [bracket-lib](https://github.com/amethyst/bracket-lib): bracket-lib is a wrapper of the bracket- set of crates designed initally for roguelike development (as RLTK) and later transitioned into a general use crate.

## Thanks and Alternatives

Additionally, I would like to thank the [Jice](https://github.com/jice-nospam) and his library [doryen-rs](https://github.com/jice-nospam/doryen-rs) for providing the bones to build this toolkit. This crate uses a modified version of `doryen-rs` to provide a more bevy-like interface.

I would also like to thank [alexschrod](https://github.com/alexschrod) who started this implementation back in bevy 0.5, which allowed me to fork the repository and continue the work.

And as always, a special shoutout to [thebracket](https://github.com/thebracket) for his [bracket-lib](https://github.com/amethyst/bracket-lib) game engine which began my journey into the world of roguelikes. He is my inspiration for this project.

## License

BRLTK is free, open source and permissively licensed!
All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

The [assets](assets) included in this repository (for our [examples](./examples/README.md)) typically fall under different open licenses, but most are free for commercial use.

See [CREDITS.md](CREDITS.md) for the details of the licenses of those files.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
