// TODO - toml lookup instead?
// - use scientific/botanical names?
// - add some form of weight/mass for estimating

use steel_cent::currency::USD;
use steel_cent::Money;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LumberType {
    DouglasFir,
    RedPine,
}

impl LumberType {
    // TODO - this doesn't scale very well
    pub fn enumerate() -> &'static [LumberType] {
        &[LumberType::DouglasFir, LumberType::RedPine]
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            LumberType::DouglasFir => "Douglas Fir",
            LumberType::RedPine => "Red Pine",
        }
    }

    pub fn fob_price(&self) -> Money {
        match self {
            LumberType::DouglasFir => Money::of_major_minor(USD, 2, 60),
            LumberType::RedPine => Money::of_major_minor(USD, 1, 13),
        }
    }
}
