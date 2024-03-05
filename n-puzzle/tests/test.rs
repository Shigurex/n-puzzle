use anyhow::Result;

#[test]
fn test_one_move() -> Result<()> {
    let args: Vec<String> = [
        "n-puzzle",
        "-a",
        "astar",
        "-h",
        "linearconflict",
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
