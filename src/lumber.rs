// - use scientific/botanical names?
// - density

use money_serde::MoneyDef;

use steel_cent::Money;

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

impl DryingMethod {
    pub fn to_str(&self) -> &str {
        match self {
            DryingMethod::KilnDried => "KilnDried",
            DryingMethod::AirDried => "AirDried",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Grade {
    I,
    II,
    III,
}

impl Grade {
    pub fn to_str(&self) -> &str {
        match self {
            Grade::I => "I",
            Grade::II => "II",
            Grade::III => "III",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Specification {
    FOHC,
    BoxedHeart,
}

impl Specification {
    pub fn to_str(&self) -> &str {
        match self {
            Specification::FOHC => "FOHC",
            Specification::BoxedHeart => "BoxedHeart",
        }
    }
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

impl Lumber {
    pub fn new(type_name: String, fob_cost: Money) -> Self {
        Self {
            type_name,
            fob_cost,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn fob_cost(&self) -> &Money {
        &self.fob_cost
    }
}
