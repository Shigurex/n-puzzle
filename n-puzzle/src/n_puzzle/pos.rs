#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        let x: Vec<String> = vec!["x", "y"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        println!("{}: {}", x[0], self.x);

        (self.x, self.y)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
