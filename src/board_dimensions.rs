// TODO - proper dimensioned type(s)
// BoardFoot<f64>
// constraints like len > 0?

use dim::{ucum, Dimensionless};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardDimensions {
    length: ucum::Meter<f64>,
    width: ucum::Meter<f64>,
    thickness: ucum::Meter<f64>,
}

impl BoardDimensions {
    pub fn new() -> Self {
        Self {
            // should be 40 board feet
            length: 1.0 * ucum::FT_I,
            width: 4.0 * ucum::IN_I,
            thickness: 4.0 * ucum::IN_I,
        }
    }

    pub fn volume(&self) -> ucum::Meter3<f64> {
        (self.length * self.width * self.thickness)
    }

    // should it be dimensionless?
    //pub fn board_feet(&self) -> BoardFoot<f64> ?
    pub fn board_feet(&self) -> f64 {
        *(self.volume() / ucum::BF_I).value()
    }
}

/// Displays 'T in X W in X L ft'
impl Display for BoardDimensions {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{} in X {} in X {} ft",
            self.thickness / ucum::IN_I,
            self.width / ucum::IN_I,
            self.length / ucum::FT_I
        )
    }
}
