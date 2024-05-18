use anstream::println;
use cargo_depcheck::{Deps, MultiVerDepResults, MultiVerDeps, MultiVerParents};
use cargo_lock::Lockfile;
use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "cargo depcheck")]
#[command(
    version,
    about = "Check for duplicate dependencies in Cargo.lock",
    long_about = None,
    styles = clap_cargo::style::CLAP_STYLING
)]
struct CargoCli {
    /// Path to Cargo.lock
    #[arg(long, short)]
    lock_path: Option<std::path::PathBuf>,

    /// Display dependency multi version dependency stats
    #[arg(long)]
    deps: bool,

    /// Display duplicate dependencies and their versions
    #[arg(long)]
    dups: bool,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,
}

fn load_and_process_lock_file(
    cli: CargoCli,
) -> Result<MultiVerDepResults, Box<dyn std::error::Error>> {
    let lock_path = cli
        .lock_path
        .unwrap_or(std::path::PathBuf::from("Cargo.lock"));
    let lock_file = Lockfile::load(lock_path)?;

    let deps = Deps::from_lock_file(lock_file)?;
    let multi_ver_deps = MultiVerDeps::from_deps(&deps);
    let multi_ver_parents = MultiVerParents::from_deps(&deps, &multi_ver_deps)?;
    let results = MultiVerDepResults::build(
        &deps,
        &multi_ver_parents,
        multi_ver_deps,
        cli.deps,
        cli.dups,
        cli.verbose,
    )?;
    Ok(results)
}

fn main() {
    let cli = CargoCli::parse();

    match load_and_process_lock_file(cli) {
        Ok(dup_dep_results) => {
            println!("{dup_dep_results}");

            if dup_dep_results.has_dup_deps() {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
