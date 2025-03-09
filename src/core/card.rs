use core::fmt;

#[derive(Debug, Clone)]
pub enum Suite {
    SPADE,
    HEART,
    DIAMOND,
    CLUB,
}
#[derive(Debug, Clone)]
pub enum Color {
    RED,
    BLACK,
}

#[derive(Debug, Clone)]
pub struct Card {
    rank: String,
    value: u8,
    suite: Suite,
    color: Color,
}

impl Card {
    pub fn new(rank: String, value: u8, suite: Suite, color: Color) -> Self {
        Self {
            rank,
            value,
            suite,
            color,
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_rank(&self) -> String {
        self.rank.clone()
    }
    pub fn get_suite(&self) -> Suite {
        self.suite.clone()
    }
    pub fn get_color(&self) -> Color {
        self.color.clone()
    }
}
impl fmt::Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CLUB => write!(f, "󰣎 CLUB"),
            Self::SPADE => write!(f, "󰣑 SPADE"),
            Self::HEART => write!(f, " HEART"),
            Self::DIAMOND => write!(f, " DIAMOND"),
        }
    }
}
