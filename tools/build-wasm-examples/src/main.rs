use std::{fs::File, io::Write};

use clap::Parser;
use xshell::{cmd, Shell};

#[derive(Parser, Debug)]
struct Args {
    /// Examples to build
    examples: Vec<String>,

    #[arg(short, long)]
    /// Run tests
    test: bool,

    #[arg(short, long)]
    /// Run on the given browsers. By default, chromium, firefox, webkit
    browsers: Vec<String>,

    #[arg(short, long)]
    /// Stop after this number of frames
    frames: Option<usize>,
    // #[arg(short, long)]
    // all: bool,
}

fn main() {
    let cli = Args::parse();
    eprintln!("CLI: {cli:?}");

    assert!(!cli.examples.is_empty(), "must have at least one example");

    let mut ci_testing = vec![];
    if let Some(frames) = cli.frames {
        let mut file = File::create("ci_testing_config.ron").unwrap();
        file.write_fmt(format_args!("(exit_after: Some({frames}))"))
            .unwrap();
        ci_testing = vec!["--features", "ci_testing"];
    }

    // let examples = if cli.all {
    //     get_examples()
    // } else {
    //     cli.examples
    // };

    for example in cli.examples {
        let sh = Shell::new().unwrap();
        let ci_testing = ci_testing.clone();

        cmd!(
            sh,
            "cargo build {ci_testing...} --release --target wasm32-unknown-unknown --example {example}"
        )
        .run()
        .expect("Error building example");

        cmd!(
            sh,
            "wasm-bindgen --out-dir examples/wasm/target --out-name wasm_example --target web target/wasm32-unknown-unknown/release/examples/{example}.wasm"
        )
        .run()
        .expect("Error creating wasm binding");

        if cli.test {
            let _dir = sh.push_dir(".github/start-wasm-example");
            let mut browsers = cli.browsers.clone();
            if !browsers.is_empty() {
                browsers.insert(0, "--project".to_string());
            }
            cmd!(sh, "npx playwright test --headed {browsers...}")
                .env("SCREENSHOT_PREFIX", format!("screenshot-{example}"))
                .run()
                .expect("Error running playwright test");
        }
    }
}

// pub fn get_examples() -> Vec<String> {
//     let mut examples = vec![];
//     let mut it = WalkDir::new("examples").into_iter();
//     loop {
//         let entry = match it.next() {
//             None => break,
//             Some(Ok(entry)) => entry,
//             Some(Err(err)) => panic!("ERROR: {err}"),
//         };

//         // No hidden files or folders
//         if is_hidden(&entry) {
//             if entry.file_type().is_dir() {
//                 it.skip_current_dir();
//             }
//             continue;
//         }

//         if !entry.file_type().is_dir()
//             && !is_restricted_file(&entry)
//             && !is_restricted_folder(&entry)
//         {
//             let parent = entry
//                 .path()
//                 .parent()
//                 .unwrap()
//                 .strip_prefix("examples")
//                 .expect("Error stripping prefix");

//             examples.push(format!(
//                 "{}_{}",
//                 parent.display(),
//                 Path::new(entry.file_name().to_str().unwrap())
//                     .with_extension("")
//                     .to_str()
//                     .unwrap()
//             ));
//         }
//     }

//     examples
// }

// fn is_hidden(entry: &DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with('.'))
//         .unwrap_or(false)
// }

// fn is_restricted_file(entry: &DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with('.') || s.starts_with("README") || s.starts_with("LICENSE"))
//         .unwrap_or(false)
// }

// fn is_restricted_folder(entry: &DirEntry) -> bool {
//     entry
//         .path()
//         .to_str()
//         .map(|s| s.contains("wasm"))
//         .unwrap_or(false)
// }
