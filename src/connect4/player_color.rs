/// Self explanatory :)
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum PlayerColor {
    Yellow,
    Red,
}

impl PlayerColor {
    pub fn next(&self) -> Self {
        match self {
            PlayerColor::Yellow => PlayerColor::Red,
            PlayerColor::Red => PlayerColor::Yellow,
        }
    }
}
