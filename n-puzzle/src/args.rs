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

    pub fn set_algorithm(&mut self, algorithm: &str) -> () {
        match algorithm {
            "astar" => self.algorithm = Algorithm::AStar,
            "uniformcost" => self.algorithm = Algorithm::UniformCost,
            "greedy" => self.algorithm = Algorithm::Greedy,
            _ => println!("error")
        }
    }

    pub fn set_heuristic(&mut self, heuristic: &str) {
        match heuristic {
            "manhattan" => self.heuristic = Heuristic::Manhattan,
            _ => println!("error")
        }
    }

    pub fn set_text_path(&mut self, text_path: &str) {
        self.puzzle_settings = PuzzleSettings::TextPath(text_path.to_string())
    }

    pub fn set_size(&mut self, size: &str) {
        let size: usize = match size.trim().parse() {
            Ok(num) => num,
            Err(_) => 0, //println!("error");
        };
        if size <= 0 {
            println!("error");
        }
        self.puzzle_settings = PuzzleSettings::Size(size)
    }
}

// TODO: change println!("error") into Err() format
// Expected format: executable [-a algorithm] [-h heuristic] [-f file | -n size]
pub fn parse_args() -> Result<Settings> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let len_args: usize = args.len();
    let mut settings: Settings = Settings::new(
        PuzzleSettings::Size(3),
        Algorithm::AStar,
        Heuristic::Manhattan
    );

    //println!("-- Expected Format --");
    //println!("usage: {} [-a algorithm] [-h heuristic] [-f file | -n size]", args[0]);

    if len_args % 2 == 0 {
        println!("error")
    }

    for i in 0..((len_args - 1) / 2) {
        let opt = args[2 * i + 1].as_str();
        let detail = args[2 * i + 2].as_str();

        match opt {
            "-a" => settings.set_algorithm(detail),
            "-h" => settings.set_heuristic(detail),
            "-f" => settings.set_text_path(detail),
            "-n" => settings.set_size(detail),
            _ => println!("error")
        }
    }

    println!("{:?}", settings);
    Ok(settings)
}
