use steel_cent::currency::USD;
use steel_cent::Money;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Money")]
pub struct MoneyDef {
    #[serde(getter = "Money::major_part")]
    major_part: i64,
    #[serde(getter = "Money::minor_part")]
    minor_part: i64,
}

impl From<MoneyDef> for Money {
    fn from(def: MoneyDef) -> Money {
        Money::of_major_minor(USD, def.major_part, def.minor_part)
    }
}
