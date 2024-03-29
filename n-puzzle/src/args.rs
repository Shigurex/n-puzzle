use anyhow::{anyhow, Result};

use std::env;

use super::{Algorithm, Heuristic, PuzzleSettings, MAX_PUZZLE_SIZE};

#[derive(Debug, PartialEq)]
pub struct Settings {
    pub puzzle_settings: PuzzleSettings,
    pub algorithm: Option<Algorithm>,
    pub heuristic: Heuristic,
    pub timeout: Option<u64>,
    pub verbose: bool,
}

impl Settings {
    pub fn new(
        puzzle_settings: PuzzleSettings,
        algorithm: Option<Algorithm>,
        heuristic: Heuristic,
        timeout: Option<u64>,
        verbose: bool,
    ) -> Self {
        Self {
            puzzle_settings,
            algorithm,
            heuristic,
            timeout,
            verbose,
        }
    }

    pub fn new_default() -> Self {
        Self::new(PuzzleSettings::Size(0), None, Heuristic::None, None, false)
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<()> {
        match self.algorithm {
            None => {}
            Some(_) => return Err(anyhow!("Duplicate algorithm defined.")),
        }
        match algorithm {
            "astar" => self.algorithm = Some(Algorithm::AStar),
            "uniform_cost" => self.algorithm = Some(Algorithm::UniformCost),
            "greedy" => self.algorithm = Some(Algorithm::Greedy),
            _ => {
                return Err(anyhow!(
                    "Not a valid algorithm: {}. Use astar, uniform_cost, or greedy",
                    algorithm
                ))
            }
        }
        Ok(())
    }

    pub fn set_heuristic(&mut self, heuristic: &str) -> Result<()> {
        match self.heuristic {
            Heuristic::None => {}
            _ => return Err(anyhow!("Duplicate heuristic defined.")),
        }
        match heuristic {
            "manhattan" => self.heuristic = Heuristic::Manhattan,
            "hamming" => self.heuristic = Heuristic::Hamming,
            "linear_conflict" => self.heuristic = Heuristic::LinearConflict,
            "inversion_distance" => self.heuristic = Heuristic::InversionDistance,
            _ => {
                return Err(anyhow!(
                    "Not a valid heuristic: {}. Use manhattan, hamming, linear_conflict or inversion_distance",
                    heuristic
                ))
            }
        }
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: &str) -> Result<()> {
        match self.timeout {
            None => {}
            Some(_) => return Err(anyhow!("Duplicate time defined.")),
        }
        let timeout: u64 = match timeout.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(_) => {
                return Err(anyhow!(
                    "Not a valid time: {}. Time must be more than 0",
                    timeout
                ))
            }
            Err(_) => {
                return Err(anyhow!(
                    "Not a valid number: {}. Use numerical numbers",
                    timeout
                ))
            }
        };
        self.timeout = Some(timeout);
        Ok(())
    }

    pub fn set_text_path(&mut self, text_path: &str) -> Result<()> {
        match self.puzzle_settings {
            PuzzleSettings::Size(0) => {}
            _ => return Err(anyhow!("Duplicate size or text_path defined.")),
        }
        if !text_path.ends_with(".txt") {
            return Err(anyhow!(
                "Not a valid file format: {}. File must be in .txt format",
                text_path
            ));
        }
        self.puzzle_settings = PuzzleSettings::TextPath(text_path.to_string());
        Ok(())
    }

    pub fn set_size(&mut self, size: &str) -> Result<()> {
        match self.puzzle_settings {
            PuzzleSettings::Size(0) => {}
            _ => return Err(anyhow!("Duplicate size or text_path defined.")),
        }
        let size: usize = match size.trim().parse() {
            Ok(num) if num > 1 && num < MAX_PUZZLE_SIZE => num,
            Ok(_) => {
                return Err(anyhow!(
                    "Not a valid size: {}. Must be between 2 and {}.",
                    size,
                    MAX_PUZZLE_SIZE
                ))
            }
            Err(_) => {
                return Err(anyhow!(
                    "Not a valid number: {}. Use numerical numbers",
                    size
                ))
            }
        };
        self.puzzle_settings = PuzzleSettings::Size(size);
        Ok(())
    }

    pub fn apply_default_setting(&mut self) -> Result<()> {
        if let PuzzleSettings::Size(0) = self.puzzle_settings {
            return Err(anyhow!("Need size or text_path."));
        }
        if self.algorithm.is_none() {
            self.algorithm = Some(Algorithm::AStar);
        }
        let is_huristic = self.algorithm.unwrap().is_heuristic();
        match self.heuristic {
            Heuristic::None => {
                if is_huristic {
                    self.heuristic = Heuristic::Manhattan;
                }
            }
            _ => {
                if !is_huristic {
                    return Err(anyhow!(
                        "Heuristic specified for algorithm that doesn't need heuristic."
                    ));
                }
            }
        }
        Ok(())
    }
}

// Get arguments
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

// Parse arguments
// Expected format: executable (file | size) [-a algorithm] [-h heuristic] [-t timeout] [--verbose]
pub fn parse_args(args: Vec<String>) -> Result<Option<Settings>> {
    let len_args: usize = args.len();
    let mut settings: Settings = Settings::new_default();

    if len_args == 1 {
        println!(
            "usage: {} (file | size) [-a algorithm] [-h heuristic] [-t timeout] [--verbose]",
            args[0]
        );
        return Ok(None);
    }

    let mut i = 1;
    while i < len_args {
        let arg = args[i].as_str();
        match arg {
            "-a" | "--algorithm" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!(
                        "Need an algorithm: Use astar, uniform_cost, or greedy"
                    ));
                }
                settings.set_algorithm(args[i].as_str())?
            }
            "-h" | "--heuristic" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!(
                        "Need a heuristic: Use manhattan, hamming, linear_conflict or inversion_distance"
                    ));
                }
                settings.set_heuristic(args[i].as_str())?
            }
            "-t" | "--timeout" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!("Need a time: Use numerical numbers"));
                }
                settings.set_timeout(args[i].as_str())?
            }
            "--verbose" => settings.verbose = true,
            _ => match arg.trim().parse::<usize>() {
                Ok(_) => settings.set_size(arg)?,
                Err(_) => settings.set_text_path(arg)?,
            },
        }
        i += 1;
    }

    settings.apply_default_setting()?;
    Ok(Some(settings))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_normal() -> Result<()> {
        let args: Vec<String> = vec!["target/debug/n-puzzle".into(), "2".into()];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(2),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_size_one() {
        let args: Vec<String> = vec!["target/debug/n-puzzle".into(), "1".into()];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_size_zero() {
        let args: Vec<String> = vec!["target/debug/n-puzzle".into(), "0".into()];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_text_path_valid() -> Result<()> {
        let args: Vec<String> = vec!["target/debug/n-puzzle".into(), "test.txt".into()];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::TextPath("test.txt".into()),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_text_path_invalid() {
        let args: Vec<String> = vec!["target/debug/n-puzzle".into(), "test.py".into()];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_algorithm_astar() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "astar".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_algorithm_uniform_cost() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "uniform_cost".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::UniformCost),
            Heuristic::None,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_algorithm_greedy() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "greedy".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::Greedy),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_algorithm_invalid() {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "invalid".into(),
        ];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_heuristic_manhattan() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-h".into(),
            "manhattan".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_heuristic_invalid() {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-h".into(),
            "invalid".into(),
        ];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_astar_with_heuristic() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "astar".into(),
            "-h".into(),
            "manhattan".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_greedy_with_heuristic() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "greedy".into(),
            "-h".into(),
            "manhattan".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::Size(3),
            Some(Algorithm::Greedy),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_uniform_cost_with_heuristic() {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "3".into(),
            "-a".into(),
            "uniform_cost".into(),
            "-h".into(),
            "manhattan".into(),
        ];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_complex_valid_middle() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "-a".into(),
            "astar".into(),
            "test.txt".into(),
            "-h".into(),
            "manhattan".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::TextPath("test.txt".into()),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_complex_valid_end() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "-a".into(),
            "astar".into(),
            "-h".into(),
            "manhattan".into(),
            "test.txt".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::TextPath("test.txt".into()),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            false,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }

    #[test]
    fn test_complex_invalid() {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "-a".into(),
            "-h".into(),
            "manhattan".into(),
            "test.txt".into(),
        ];
        let settings = parse_args(args);
        assert!(settings.is_err());
    }

    #[test]
    fn test_verbose() -> Result<()> {
        let args: Vec<String> = vec![
            "target/debug/n-puzzle".into(),
            "-a".into(),
            "astar".into(),
            "-h".into(),
            "manhattan".into(),
            "test.txt".into(),
            "--verbose".into(),
        ];
        let settings = parse_args(args)?.unwrap();
        let answer_settings = Settings::new(
            PuzzleSettings::TextPath("test.txt".into()),
            Some(Algorithm::AStar),
            Heuristic::Manhattan,
            None,
            true,
        );
        assert_eq!(settings, answer_settings);
        Ok(())
    }
}
