use anyhow::Result;

use std::env;

use super::{
    PuzzleSettings, Algorithm, Heuristic,
};

#[derive(Debug)]
pub struct Settings {
    pub puzzle_settings: PuzzleSettings,
    pub algorithm: Algorithm,
    pub heuristic: Heuristic,
}

impl Settings {
    pub fn new(puzzle_settings: PuzzleSettings, algorithm: Algorithm, heuristic: Heuristic) -> Self {
        Self {
            puzzle_settings,
            algorithm,
            heuristic,
        }
    }
}

pub fn parse_args() -> Result<Settings> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let mut settings: Settings = Settings::new(
        PuzzleSettings::Size(3),
        Algorithm::AStar,
        Heuristic::Manhattan
    );

    if args.len() % 2 == 0 {
        println!("usage: {} [-a algorithm] [-h heuristic] [-f file | -n size]", args[0]);
        println!("error")
    }

    for i in 0..((args.len() - 1) / 2) {
        let opt = args[2 * i + 1].as_str();
        let detail = args[2 * i + 2].as_str();

        match opt {
            "-a" => match detail {
                "astar" => settings.algorithm = Algorithm::AStar,
                "uniformcost" => settings.algorithm = Algorithm::UniformCost,
                "greedy" => settings.algorithm = Algorithm::Greedy,
                _ => println!("error")
            },
            "-h" => match detail {
                "manhattan" => settings.heuristic = Heuristic::Manhattan,
                _ => println!("error")
            },
            "-f" => settings.puzzle_settings = PuzzleSettings::TextPath(detail.to_string()),
            "-n" => {
                let size: usize = match detail.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                if size <= 0 {
                    println!("error");
                }
                settings.puzzle_settings = PuzzleSettings::Size(size)
            },
            _ => println!("error")
        }
    }

    println!("{:?}", settings);
    Ok(settings)
}
