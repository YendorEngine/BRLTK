# Examples

These examples demonstrate the main features of BRLTK and how to use them.
To run an example, use the command `cargo run --example <Example>`. Some examples require additional features. Add the option `--features <feature>` for the example to run.

```sh
cargo run --features bracket_lib --example bracket_basic
cargo run --features ascii_terminal --example ascii_terminal_basic
```

**⚠️ Note: for users of releases on crates.io!**

There are often large differences and incompatible API changes between the latest [crates.io](https://crates.io/crates/brltk) release and the development version of BRLTK in the git main branch!

If you are using a released version of BRLTK, you need to make sure you are viewing the correct version of the examples!

- Latest release: [BRLTK Main Examples](https://github.com/YendorEngine/BRLTK/tree/main/examples)
- Specific version, such as `0.1`: [https://github.com/YendorEngine/BRLTK/tree/v0.1.0/examples](https://github.com/YendorEngine/BRLTK/tree/v0.1.0/examples)

When you clone the repo locally to run the examples, use `git checkout` to get the correct version:

```bash
# `latest` always points to the newest release
git checkout latest
# or use a specific version
git checkout v0.1.0
```

---

## Table of Contents

- [Examples](#examples)

  - [Table of Contents](#table-of-contents)

- [Cross-Platform Examples](#cross-platform-examples)

  - [Doryen Examples](#doryen-rs-examples)
  - [Bracket_Lib Examples](#bracket-lib-examples)
  - [Bevy Ascii Terminal Examples](#bevy-ascii-terminal-examples)

- [Platform-Specific Examples](#platform-specific-examples)

  - [WASM](#wasm)
    - [Setup](#setup)
    - [Build &amp; Run](#build--run)
    - [Loading Assets](#loading-assets)

## Cross-Platform Examples

### doryen-rs examples

| Example                    | Description                              |
| -------------------------- | ---------------------------------------- |
| [basic](./doryen/basic.rs) | Runs a minimal example using `doryen-rs` |

### bracket-lib examples

| Example                         | Description                                |
| ------------------------------- | ------------------------------------------ |
| [basic](./bracket_lib/basic.rs) | Runs a minimal example using `bracket-lib` |

### bevy ascii terminal examples

| Example                            | Description                                        |
| ---------------------------------- | -------------------------------------------------- |
| [basic](./ascii_terminal/basic.rs) | Runs a minimal example using `bevy_ascii_terminal` |

## Platform-Specific Examples

## WASM

### Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

### Build & Run

Following is an example for `doryen_basic`. For other examples, change the `doryen_basic` in the
following commands.

```sh
cargo build --release --example doryen_basic --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example \
  --out-dir examples/wasm/target \
  --target web target/wasm32-unknown-unknown/release/examples/doryen_basic.wasm
```

The first command will build the example for the wasm target, creating a binary. Then,
[wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) is used to create
javascript bindings to this wasm file, which can be loaded using this
[example HTML file](./wasm/index.html).

Then serve `examples/wasm` directory to browser. i.e.

```sh
# cargo install basic-http-server
basic-http-server examples/wasm

# with python
python3 -m http.server --directory examples/wasm

# with ruby
ruby -run -ehttpd examples/wasm
```

### Optimizing

On the web, it's useful to reduce the size of the files that are distributed.
With rust, there are many ways to improve your executable sizes.
Here are some.

#### 1. Tweak your `Cargo.toml`

Add a new [profile](https://doc.rust-lang.org/cargo/reference/profiles.html)
to your `Cargo.toml`:

```toml
[profile.wasm-release]
# Use release profile as default values
inherits = "release"

# Optimize with size in mind, also try "s", sometimes it is better.
# This doesn't increase compilation times compared to -O3, great improvements
opt-level = "z"

# Do a second optimization pass removing duplicate or unused code from dependencies.
# Slows compile times, marginal improvements
lto = "fat"

# When building crates, optimize larger chunks at a time
# Slows compile times, marginal improvements
codegen-units = 1
```

Now, when building the final executable, use the `wasm-release` profile
by replacing `--release` by `--profile wasm-release` in the cargo command.

```sh
cargo build --profile wasm-release --example lighting --target wasm32-unknown-unknown
```

Make sure your final executable size is smaller, some of those optimizations
may not be worth keeping, due to compilation time increases.

#### 2. Use `wasm-opt` from the binaryen package

Binaryen is a set of tools for working with wasm. It has a `wasm-opt` CLI tool.

First download the `binaryen` package,
then locate the `.wasm` file generated by `wasm-bindgen`.
It should be in the `--out-dir` you specified in the command line,
the file name should end in `_bg.wasm`.

Then run `wasm-opt` with the `-Oz` flag. Note that `wasm-opt` is _very slow_.

Note that `wasm-opt` optimizations might not be as effective if you
didn't apply the optimizations from the previous section.

```sh
wasm-opt -Oz --output optimized.wasm examples/wasm/target/lighting_bg.wasm
mv optimized.wasm examples/wasm/target/lighting_bg.wasm
```

For a small project with a basic 3d model and two lights,
the generated file sizes are, as of Jully 2022 as following:

| profile                          | wasm-opt | no wasm-opt |
| -------------------------------- | -------- | ----------- |
| Default                          | 8.5M     | 13.0M       |
| opt-level = "z"                  | 6.1M     | 12.7M       |
| "z" + lto = "thin"               | 5.9M     | 12M         |
| "z" + lto = "fat"                | 5.1M     | 9.4M        |
| "z" + "thin" + codegen-units = 1 | 5.3M     | 11M         |
| "z" + "fat" + codegen-units = 1  | 4.8M     | 8.5M        |

There are more advanced optimization options available,
check the following pages for more info:

- [https://rustwasm.github.io/book/reference/code-size.html](https://rustwasm.github.io/book/reference/code-size.html)
- [https://rustwasm.github.io/docs/wasm-bindgen/reference/optimize-size.html](https://rustwasm.github.io/docs/wasm-bindgen/reference/optimize-size.html)
- [https://rustwasm.github.io/book/game-of-life/code-size.html](https://rustwasm.github.io/book/game-of-life/code-size.html)

### Loading Assets

To load assets, they need to be available in the folder examples/wasm/assets. Cloning this
repository will set it up as a symlink on Linux and macOS, but you will need to manually move
the assets on Windows.
