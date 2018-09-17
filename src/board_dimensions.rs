// TODO - proper dimensioned type(s)
// BoardFoot<f64>
// constraints like len > 0?

use dim::{ucum, Dimensionless};
use nom::types::CompleteStr;
use nom::{space, IResult};
use std::fmt::{self, Display, Formatter};
use std::str;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
            "{:.1} in X {:.1} in X {:.1} ft",
            self.thickness / ucum::IN_I,
            self.width / ucum::IN_I,
            self.length / ucum::FT_I
        )
    }
}

#[derive(Debug, PartialEq)]
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
        space >>
        opt!(tag!("in ")) >>
        alt!(tag!("X") | tag!("x") | tag!("*")) >>
        space >>
        m_w: parse_double >>
        space >>
        opt!(tag!("in ")) >>
        alt!(tag!("X") | tag!("x") | tag!("*")) >>
        space >>
        m_l: parse_double >>
        space >>
        opt!(tag!("ft")) >>
        (IR { t: m_t, w: m_w, l: m_l })
    )
);

// T in X W in X L ft
// TODO - needs error handling
impl FromStr for BoardDimensions {
    type Err = ();

    fn from_str(s: &str) -> Result<BoardDimensions, ()> {
        let res: IResult<CompleteStr, IR> = parse_dimensions(CompleteStr(s));

        if let Ok((_, ir)) = res {
            let length = if ir.l.abs() <= 0.0 { 0.1 } else { ir.l.abs() };
            let width = if ir.w.abs() <= 0.0 { 0.1 } else { ir.w.abs() };
            let thickness = if ir.t.abs() <= 0.0 { 0.1 } else { ir.t.abs() };

            Ok(BoardDimensions {
                length: length * ucum::FT_I,
                width: width * ucum::IN_I,
                thickness: thickness * ucum::IN_I,
            })
        } else {
            Err(())
        }
    }
}

// TODO - just some examples to get by with, needs filled out
#[test]
fn parse_double_test() {
    assert_eq!(
        parse_double(CompleteStr("123.234")),
        Ok((CompleteStr(""), 123.234_f64))
    );
}

#[test]
fn parse_dimensions_test() {
    assert_eq!(
        parse_dimensions(CompleteStr("12.3 in X 5.8 in X 88.12 ft")),
        Ok((
            CompleteStr(""),
            IR {
                t: 12.3,
                w: 5.8,
                l: 88.12,
            }
        ))
    );
}

#[test]
fn board_dimensions_from_str_test() {
    let bd = BoardDimensions::from_str("1.8 in X 66.3 in X 384.22346 ft").unwrap();

    assert_eq!(
        bd,
        BoardDimensions {
            length: 384.22346 * ucum::FT_I,
            width: 66.3 * ucum::IN_I,
            thickness: 1.8 * ucum::IN_I,
        }
    );
}
