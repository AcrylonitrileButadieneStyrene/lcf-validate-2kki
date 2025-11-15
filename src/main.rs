#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use lcf::ConvertExt;
use owo_colors::OwoColorize;

mod directory_browser;
mod lints;

pub use lints::{Diagnostic, Lint};

#[derive(Clone, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum LogLevel {
    #[default]
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
            check_map(&directory_browser::run(path), &args.level);
        }
    } else {
        match args.path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("ldb") | Some("lmt") => {
                check_map(
                    &directory_browser::run(args.path.parent().unwrap().to_owned()),
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
        .filter(|(_, diagnostic)| match diagnostic {
            Diagnostic::Normal => *level == LogLevel::All,
            Diagnostic::Warning(_) => *level != LogLevel::Error,
            Diagnostic::Error(_) => true,
        })
        .collect::<Vec<_>>();

    if !results.is_empty() {
        print_file();
        for (name, diagnostic) in results {
            match diagnostic {
                Diagnostic::Normal => println!("  {}", name.green()),
                Diagnostic::Warning(warning) => println!("  {}: {warning}", name.yellow()),
                Diagnostic::Error(err) => println!("  {}: {err}", name.red()),
            }
        }
    }

    return;
}

fn exit() -> ! {
    if atty::is(atty::Stream::Stdin) {
        eprint!("Press enter to exit...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        drop(std::io::stdin().read_line(&mut String::new()));
    }
    std::process::exit(0);
}
