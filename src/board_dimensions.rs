// TODO - proper dimensioned type(s)
// BoardFoot<f64>
// constraints like len > 0?

use dim::{ucum, Dimensionless};
use nom::types::CompleteStr;
use nom::{digit, float_s, IResult};
use std::fmt::{self, Display, Formatter};
use std::str;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardDimensions {
    length: ucum::Meter<f64>,
    width: ucum::Meter<f64>,
    thickness: ucum::Meter<f64>,
}

impl BoardDimensions {
    pub fn new() -> Self {
        Self::default()
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

impl Default for BoardDimensions {
    fn default() -> BoardDimensions {
        BoardDimensions {
            length: 1.0 * ucum::FT_I,
            width: 4.0 * ucum::IN_I,
            thickness: 4.0 * ucum::IN_I,
        }
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

#[derive(Debug)]
struct IR {
    pub t: f64,
    pub w: f64,
    pub l: f64,
}

fn parse_double(input: CompleteStr) -> ::nom::IResult<CompleteStr, f64> {
    flat_map!(input, call!(::nom::recognize_float), parse_to!(f64))
}

named!(parse_dimensions<CompleteStr, IR>,
    do_parse!(
        m_t: parse_double >>
        tag!(" in X ") >>
        m_w: parse_double >>
        tag!(" in X ") >>
        m_l: parse_double >>
        tag!(" ft") >>
        (IR { t: m_t, w: m_w, l: m_l })
    )
);

// T in X W in X L ft
impl FromStr for BoardDimensions {
    type Err = ();

    fn from_str(_s: &str) -> Result<BoardDimensions, ()> {
        Err(())
    }
}

#[test]
fn board_dimensions_from_str_test() {
    let res = parse_double(CompleteStr("123.234"));

    println!("{:#?}", res);

    let res = parse_dimensions(CompleteStr("12.3 in X 5.8 in X 88.12 ft"));

    println!("{:#?}", res);
}
