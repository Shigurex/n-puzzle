use anyhow::{Result, anyhow};

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

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<()> {
        match algorithm {
            "astar" => self.algorithm = Algorithm::AStar,
            "uniformcost" => self.algorithm = Algorithm::UniformCost,
            "greedy" => self.algorithm = Algorithm::Greedy,
            _ => return Err(anyhow!("Not a valid algorithm: {}. Use astar, uniformcost, or greedy", algorithm))
        }
        Ok(())
    }

    pub fn set_heuristic(&mut self, heuristic: &str) -> Result<()> {
        match heuristic {
            "manhattan" => self.heuristic = Heuristic::Manhattan,
            _ => return Err(anyhow!("Not a valid heuristic: {}. Use manhattan", heuristic))
        }
        Ok(())
    }

    pub fn set_text_path(&mut self, text_path: &str) -> Result<()> {
        if !text_path.ends_with(".txt") {
            return Err(anyhow!("Not a valid file format: {}. File must be in .txt format", text_path))
        }
        self.puzzle_settings = PuzzleSettings::TextPath(text_path.to_string());
        Ok(())
    }

    pub fn set_size(&mut self, size: &str) -> Result<()> {
        let size: usize = match size.trim().parse() {
            Ok(num) if num > 1 => num,
            Ok(_) => return Err(anyhow!("Not a valid size: {}. Size must be more than 1", size)),
            Err(_) => return Err(anyhow!("Not a valid number: {}. Use numerical numbers", size))
        };
        self.puzzle_settings = PuzzleSettings::Size(size);
        Ok(())
    }
}

// Parse arguments
// Expected format: executable (file | size) [-a algorithm] [-h heuristic]
pub fn parse_args() -> Result<Option<Settings>> {
    let args: Vec<String> = env::args().collect();
    let len_args: usize = args.len();
    let mut settings: Settings = Settings::new(
        PuzzleSettings::Size(0),
        Algorithm::AStar,
        Heuristic::Manhattan
    );

    if len_args == 1 {
        println!("usage: {} (file | size) [-a algorithm] [-h heuristic]", args[0]);
        return Ok(None);
    }

    let mut i = 1;
    while i < len_args {
        let arg = args[i].as_str();
        match arg {
            "-a" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!("Need an algorithm: Use astar, uniformcost, or greedy"))
                }
                settings.set_algorithm(args[i].as_str())?
            },
            "-h" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!("Need a heuristic: Use manhattan"))
                }
                settings.set_heuristic(args[i].as_str())?
            },
            _ => match arg.trim().parse::<usize>() {
                Ok(_) => settings.set_size(arg)?,
                Err(_) => settings.set_text_path(arg)?,
            }
        }
        i += 1;
    }

    println!("{:?}", settings);
    Ok(Some(settings))
}
