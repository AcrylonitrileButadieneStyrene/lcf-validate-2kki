#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::missing_panics_doc)]

use indicatif::ParallelProgressIterator as _;
use lcf::ConvertExt;
pub use lints::{Diagnostic, DiagnosticEvent, DiagnosticLevel, DiagnosticPage, Lint};
use owo_colors::OwoColorize;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod lints;

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
    /// Minimum level for logging, everything lower will be ignored.
    #[arg(long, default_value = "all")]
    level: LogLevel,
    /// Ignored lints
    #[arg(long, value_delimiter = ',')]
    ignore: Vec<usize>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let target = if std::fs::metadata(&args.path).unwrap().is_dir() {
        let tree = read_tree(&args.path);
        either::Either::Right((args.path, tree))
    } else {
        match args.path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("lmt" | "ldb") => either::Either::Right((
                args.path.parent().unwrap().to_owned(),
                read_tree(&args.path.with_extension("lmt")),
            )),
            Some("lmu") => either::Either::Left(args.path),
            x => {
                println!(
                    "Unrecognized extension {} is not supported.",
                    x.unwrap_or("<none>")
                );
                exit();
            }
        }
    };

    match target {
        either::Either::Left(single) => analyze(
            read_map(&single).unwrap().unwrap(),
            &args.level,
            &args.ignore,
        )
        .for_each(|line| println!("{}", line)),
        either::Either::Right((base, tree)) => {
            let mut maps = tree.maps[1..]
                .into_par_iter()
                .progress_with(
                    indicatif::ProgressBar::new(tree.maps.len() as u64 - 1).with_style(
                        indicatif::ProgressStyle::default_bar()
                            .template(
                                "[{elapsed} / {duration}] {bar:40.cyan/blue} {pos:>4}/{len:4} ({percent}%) {per_sec:>0}",
                            )
                            .unwrap(),
                    ),
                )
                .map(
                    |(id, _)| match read_map(&base.join(format!("Map{id:04}.lmu"))) {
                        Ok(Ok(map)) => (
                            id,
                            either::Right(
                                analyze(map, &args.level, &args.ignore).collect::<Vec<_>>(),
                            ),
                        ),
                        Ok(Err(err)) => (
                            id,
                            either::Left(format!(
                                "Map{id:04}.lmu\n  {}: {}",
                                "Invalid map file".on_red(),
                                err.red()
                            )),
                        ),
                        Err(err) => (id, either::Left(format!("Map{id:04}.lmu\n  {}", err.red()))),
                    },
                )
                .collect::<Vec<_>>();
            maps.sort_by_key(|item| *item.0);
            maps.iter().for_each(|(id, result)| match result {
                either::Left(err) => println!("{}", err),
                either::Right(output) => {
                    println!("Map{id:04}.lmu:");
                    for line in output {
                        println!("  {}", line);
                    }
                }
            });
        }
    };

    exit();
}

fn read_tree(path: &std::path::Path) -> lcf::lmt::LcfMapTree {
    let bytes = std::fs::read(path.join("RPG_RT.lmt")).unwrap();
    lcf::lmt::LcfMapTree::read(&mut std::io::Cursor::new(bytes)).unwrap()
}

fn analyze(
    map: lcf::lmu::LcfMapUnit,
    level: &LogLevel,
    ignored: &[usize],
) -> impl Iterator<Item = String> {
    lints::ALL
        .iter()
        .enumerate()
        .filter(|(index, _)| !ignored.iter().any(|ignore| *ignore == index + 1))
        .map(move |(index, lint)| (index + 1, lint.name(), lint.test(&map)))
        .filter_map(move |(index, name, diagnostics)| match level {
            LogLevel::All => Some((index, name, diagnostics)),
            LogLevel::Warn => {
                if diagnostics.is_empty() {
                    None
                } else {
                    Some((index, name, diagnostics))
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
                    Some((index, name, diagnostics))
                }
            }
        })
        .flat_map(|(index, name, diagnostics)| {
            if diagnostics.is_empty() {
                vec![format!("L{index:04}: {}", name.green())]
            } else {
                let mut items = Vec::with_capacity(diagnostics.len() + 1);
                items.push(format!("L{index:04}: {name}:"));
                items.extend(
                    diagnostics
                        .into_iter()
                        .map(|diagnostic| match diagnostic.level {
                            DiagnosticLevel::Warning => format!("  {}", diagnostic.yellow()),
                            DiagnosticLevel::Error => format!("  {}", diagnostic.red()),
                        }),
                );
                items
            }
        })
}

fn read_map(
    path: &std::path::Path,
) -> Result<Result<lcf::lmu::LcfMapUnit, lcf::lmu::LcfMapUnitReadError>, std::io::Error> {
    let bytes = std::fs::read(path)?;
    let mut cursor = std::io::Cursor::new(bytes);
    let map = lcf::lmu::LcfMapUnit::read(&mut cursor);
    Ok(map)
}

fn exit() -> ! {
    if atty::is(atty::Stream::Stdin) && atty::is(atty::Stream::Stdout) {
        eprint!("Press enter to exit...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        drop(std::io::stdin().read_line(&mut String::new()));
    }
    std::process::exit(0);
}
