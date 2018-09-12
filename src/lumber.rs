// - use scientific/botanical names?
// - density

use money_serde::MoneyDef;

use steel_cent::Money;

pub trait FobCostReader {
    fn fob_cost(&self, lumber_type: &LumberType) -> Money;
}

pub type LumberType = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lumber {
    type_name: String,
    //density
    #[serde(with = "MoneyDef")]
    fob_cost: Money,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Props {
    pub drying_method: DryingMethod,
    pub grade: Grade,
    pub spec: Specification,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DryingMethod {
    KilnDried,
    AirDried,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Grade {
    I,
    II,
    III,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Specification {
    FOHC,
    BoxedHeart,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            drying_method: DryingMethod::KilnDried,
            grade: Grade::I,
            spec: Specification::FOHC,
        }
    }
}
