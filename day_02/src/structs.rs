#[derive(Debug)]
pub struct Cubes {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<Cubes>,
}
