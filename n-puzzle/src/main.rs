fn main() {
    match n_puzzle::cui_run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
