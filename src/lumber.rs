// - use scientific/botanical names?
// - density

use money_serde::MoneyDef;

use std::str::FromStr;
use steel_cent::Money;

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

    pub fn enumerate() -> &'static [DryingMethod] {
        &[DryingMethod::KilnDried, DryingMethod::AirDried]
    }
}

// TODO - how to use a single source for string?
impl FromStr for DryingMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<DryingMethod, ()> {
        match s {
            "KilnDried" => Ok(DryingMethod::KilnDried),
            "AirDried" => Ok(DryingMethod::AirDried),
            _ => Err(()),
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

    pub fn enumerate() -> &'static [Grade] {
        &[Grade::I, Grade::II, Grade::III]
    }
}

impl FromStr for Grade {
    type Err = ();

    fn from_str(s: &str) -> Result<Grade, ()> {
        match s {
            "I" => Ok(Grade::I),
            "II" => Ok(Grade::II),
            "III" => Ok(Grade::III),
            _ => Err(()),
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

    pub fn enumerate() -> &'static [Specification] {
        &[Specification::FOHC, Specification::BoxedHeart]
    }
}

impl FromStr for Specification {
    type Err = ();

    fn from_str(s: &str) -> Result<Specification, ()> {
        match s {
            "FOHC" => Ok(Specification::FOHC),
            "BoxedHeart" => Ok(Specification::BoxedHeart),
            _ => Err(()),
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
