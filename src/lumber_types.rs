// TODO - toml lookup instead?
// - use scientific/botanical names?
// - add some form of weight/mass for estimating

use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn fob_price(&self) -> SmallMoney {
        match self {
            LumberType::DouglasFir => SmallMoney::of_major_minor(USD, 2, 60),
            LumberType::RedPine => SmallMoney::of_major_minor(USD, 1, 13),
        }
    }
}
