#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use lcf::ConvertExt;
use owo_colors::OwoColorize;

mod directory_browser;
mod lints;

pub use lints::{Diagnostic, Lint};

#[derive(clap::Parser)]
struct Args {
    #[arg(index = 1, default_value = ".")]
    path: std::path::PathBuf,
    /// Check every map file in a game.
    #[arg(long)]
    all: bool,
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
                check_map(&path.join(format!("Map{id:04}.lmu")));
            }
        } else {
            check_map(&directory_browser::run(path));
        }
    } else {
        match args.path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("ldb") | Some("lmt") => {
                check_map(&directory_browser::run(
                    args.path.parent().unwrap().to_owned(),
                ));
            }
            Some("lmu") => check_map(&args.path),
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

fn check_map(map: &std::path::Path) {
    println!("{}:", map.file_name().unwrap().to_str().unwrap());

    let data = match std::fs::read(map) {
        Ok(data) => data,
        Err(err) => {
            println!("  {}", err.red());
            return;
        }
    };

    let map = match lcf::lmu::LcfMapUnit::read(&mut std::io::Cursor::new(data)) {
        Ok(map) => map,
        Err(err) => {
            println!("  {}: {}", "Invalid map file".on_red(), err.red());
            return;
        }
    };

    for lint in lints::ALL {
        let name = lint.name();
        match lint.test(&map) {
            Diagnostic::Normal => println!("  {}", name.green()),
            Diagnostic::Warning(warning) => println!("  {}: {warning}", name.yellow()),
            Diagnostic::Error(err) => println!("  {}: {err}", name.red()),
        }
    }
}

fn exit() -> ! {
    if atty::is(atty::Stream::Stdin) {
        print!("Press enter to exit...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        drop(std::io::stdin().read_line(&mut String::new()));
    }
    std::process::exit(0);
}
