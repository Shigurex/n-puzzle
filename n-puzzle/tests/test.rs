use anyhow::Result;

#[test]
fn test_one_move() -> Result<()> {
    let args: Vec<String> = [
        "n-puzzle",
        "-a",
        "astar",
        "-h",
        "linear_conflict",
        "-t",
        "10",
        "../puzzles/one_move_three.txt",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    n_puzzle::run(args)?;
    Ok(())
}

#[test]
fn test_error_size_puzzle() -> Result<()> {
    let args: Vec<String> = [
        "n-puzzle",
        "-a",
        "astar",
        "-h",
        "linear_conflict",
        "../puzzles/error_one_puzzle.txt",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert!(n_puzzle::run(args).is_err());
    Ok(())
}

#[test]
fn test_unsolvable_puzzle() -> Result<()> {
    let args: Vec<String> = ["n-puzzle", "../puzzles/unsolvable_puzzle.txt"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    match n_puzzle::run(args) {
        Ok(_) => panic!("Should return an error"),
        Err(e) => assert!(e.to_string().contains("No solution")),
    }
    Ok(())
}

#[test]
fn test_100_puzzle() -> Result<()> {
    let args: Vec<String> = ["n-puzzle", "../puzzles/max_100_puzzle.txt"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    n_puzzle::run(args)?;
    Ok(())
}
