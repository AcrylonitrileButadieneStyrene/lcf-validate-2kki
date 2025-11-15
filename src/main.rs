#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::missing_panics_doc)]

use lcf::ConvertExt;
use owo_colors::OwoColorize;

mod directory_browser;
mod lints;

pub use lints::{Diagnostic, DiagnosticEvent, DiagnosticLevel, DiagnosticPage, Lint};

#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum LogLevel {
    All,
    Warn,
    Error,
}

#[derive(clap::Parser)]
struct Args {
    #[arg(index = 1, default_value = ".")]
    path: std::path::PathBuf,
    /// Check every map file in a game.
    #[arg(long)]
    all: bool,
    /// Minimum level for logging, everything lower will be ignored.
    #[arg(long, default_value = "all")]
    level: LogLevel,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    if std::fs::metadata(&args.path).unwrap().is_dir() {
        let Some(path) = find_game_dir(args.path) else {
            println!("Failed to find a game from the given directory");
            exit();
        };

        if args.all {
            let tree = lcf::lmt::LcfMapTree::read(&mut std::io::Cursor::new(
                std::fs::read(path.join("RPG_RT.lmt")).unwrap(),
            ))
            .unwrap();

            for (id, _) in &tree.maps[1..] {
                check_map(&path.join(format!("Map{id:04}.lmu")), &args.level);
            }
        } else {
            check_map(&directory_browser::run(&path), &args.level);
        }
    } else {
        match args.path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("ldb" | "lmt") => {
                check_map(
                    &directory_browser::run(args.path.parent().unwrap()),
                    &args.level,
                );
            }
            Some("lmu") => check_map(&args.path, &args.level),
            x => {
                println!(
                    "Unrecognized extension {} is not supported.",
                    x.unwrap_or("<none>")
                );
                exit();
            }
        }
    }

    exit();
}

fn find_game_dir(base: std::path::PathBuf) -> Option<std::path::PathBuf> {
    if base.join("RPG_RT.ldb").exists() {
        Some(base)
    } else {
        let parent = base.parent()?;
        if parent.join("RPG_RT.ldb").exists() {
            Some(parent.to_owned())
        } else {
            None
        }
    }
}

fn check_map(map: &std::path::Path, level: &LogLevel) {
    let print_file = || println!("{}:", map.file_name().unwrap().to_str().unwrap());

    let data = match std::fs::read(map) {
        Ok(data) => data,
        Err(err) => {
            print_file();
            println!("  {}", err.red());
            return;
        }
    };

    let map = match lcf::lmu::LcfMapUnit::read(&mut std::io::Cursor::new(data)) {
        Ok(map) => map,
        Err(err) => {
            print_file();
            println!("  {}: {}", "Invalid map file".on_red(), err.red());
            return;
        }
    };

    let results = lints::ALL
        .iter()
        .map(|lint| (lint.name(), lint.test(&map)))
        .filter_map(|(name, diagnostics)| match level {
            LogLevel::All => Some((name, diagnostics)),
            LogLevel::Warn => {
                if diagnostics.is_empty() {
                    None
                } else {
                    Some((name, diagnostics))
                }
            }
            LogLevel::Error => {
                let diagnostics = diagnostics
                    .into_iter()
                    .filter(|diagnostic| matches!(diagnostic.level, DiagnosticLevel::Error))
                    .collect::<Vec<_>>();
                if diagnostics.is_empty() {
                    None
                } else {
                    Some((name, diagnostics))
                }
            }
        })
        .collect::<Vec<_>>();

    if !results.is_empty() {
        print_file();
        for (name, diagnostics) in results {
            if diagnostics.is_empty() {
                println!("  {}", name.green());
            } else {
                println!("  {name}:");
                for diagnostic in diagnostics {
                    match diagnostic.level {
                        DiagnosticLevel::Warning => println!("    {}", diagnostic.yellow()),
                        DiagnosticLevel::Error => println!("    {}", diagnostic.red()),
                    }
                }
            }
        }
    }
}

fn exit() -> ! {
    if atty::is(atty::Stream::Stdin) {
        eprint!("Press enter to exit...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        drop(std::io::stdin().read_line(&mut String::new()));
    }
    std::process::exit(0);
}
