use clap::Parser;
use log::{info, LevelFilter};
use sat_rs::solve_dimacs;
use simplelog::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[command(name = "sat_rs", about = "MiniSAT 2 based SAT solver")]
struct Cli {
    #[arg(default_value = "./default.cnf")]
    input: String,

    #[arg(short, long, default_value = "info")]
    log_level: LevelFilter,
}

fn main() {
    let cli = Cli::parse();

    CombinedLogger::init(vec![
        TermLogger::new(
            cli.log_level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            cli.log_level,
            Config::default(),
            File::create("sat.log").unwrap(),
        ),
    ])
    .unwrap();

    let mut file = File::open(&cli.input).unwrap_or_else(|e| {
        eprintln!("error opening '{}': {}", cli.input, e);
        std::process::exit(1);
    });
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let state = solve_dimacs(&buffer);

    let result = if state.ok {
        "SATISFIABLE"
    } else {
        "UNSATISFIABLE"
    };
    info!("{}|{}|{}|{}", result, file!(), line!(), 2);
    println!("{}", state.solver_stats);
}
