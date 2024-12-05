//! Implement the simplest XYZ file format.
//!
//! Format:
//! ```plaintext
//! <number of atoms>
//! <title>
//! <atom_symbol> <x> <y> <z>
//! <atom_symbol> <x> <y> <z>
//! ...
//! ```

use core::fmt;

use nom::{
    character::complete::{alpha1, digit1, line_ending, not_line_ending, space0, space1},
    combinator::{all_consuming, map, map_res},
    error::{context, convert_error, ContextError, FromExternalError, ParseError, VerboseError},
    multi::count,
    number,
    sequence::{pair, preceded, terminated, tuple},
    Finish, IResult,
};

/// Represent a XYZ file.
pub struct Xyz {
    pub title: String,
    pub atoms: Vec<XyzAtom>,
}

impl Xyz {
    /// Create a new XYZ file.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            atoms: Vec::new(),
        }
    }

    fn parse_line<'a, O, E: ParseError<&'a str>>(
        parser: impl FnMut(&'a str) -> IResult<&'a str, O, E>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
        terminated(parser, pair(space0, line_ending))
    }

    fn parse_n_atoms<
        'a,
        E: ParseError<&'a str>
            + ContextError<&'a str>
            + FromExternalError<&'a str, std::num::ParseIntError>,
    >(
        input: &'a str,
    ) -> IResult<&'a str, usize, E> {
        context(
            "number of atoms",
            Self::parse_line(map_res(digit1, |s: &str| s.parse::<usize>())),
        )(input)
    }

    fn parse_title<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        context("title", Self::parse_line(not_line_ending))(input)
    }

    pub fn parse<
        'a,
        E: ParseError<&'a str>
            + ContextError<&'a str>
            + FromExternalError<&'a str, std::num::ParseIntError>,
    >(
        input: &'a str,
    ) -> IResult<&'a str, Xyz, E> {
        let (input, n_atoms) = Self::parse_n_atoms(input)?;
        let (input, title) = Self::parse_title(input)?;

        let (input, atoms) = context(
            "atoms",
            map(
                pair(
                    count(Self::parse_line(XyzAtom::parse), n_atoms - 1),
                    all_consuming(terminated(XyzAtom::parse, space0)),
                ),
                |(mut atoms, last)| {
                    atoms.push(last);
                    atoms
                },
            ),
        )(input)?;

        Ok((
            input,
            Xyz {
                title: title.to_string(),
                atoms,
            },
        ))
    }
}
impl std::str::FromStr for Xyz {
    type Err = ParseXyzError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, xyz) = Self::parse::<VerboseError<&str>>(s)
            .finish()
            .map_err(|e| ParseXyzError(convert_error(s, e)))?;
        Ok(xyz)
    }
}

#[derive(Debug)]
pub struct ParseXyzError(String);

impl fmt::Display for ParseXyzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse XYZ file: {}", self.0)
    }
}

/// Represent a XYZ record.
pub struct XyzAtom {
    pub symbol: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XyzAtom {
    /// Create a new record.
    pub fn new(symbol: &str, x: f64, y: f64, z: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            x,
            y,
            z,
        }
    }

    /// Parse a line of text into an `XyzAtom`.
    /// The line should contain an atom symbol followed by three floating-point numbers.
    /// Example: "C 0.0 0.0 0.0"
    fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, XyzAtom, E> {
        let parse_spaced_double = |input| preceded(space1, number::complete::double)(input);

        // equal to: std::cin >> symbol >> x >> y >> z;
        context(
            "atom",
            map(
                tuple((
                    alpha1,
                    parse_spaced_double,
                    parse_spaced_double,
                    parse_spaced_double,
                )),
                |(symbol, x, y, z)| XyzAtom::new(symbol, x, y, z),
            ),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz_parse() {
        // from https://en.wikipedia.org/wiki/XYZ_file_format
        let pyridine_xyz = r#"11
Pyridine
C       -0.180226841      0.360945118     -1.120304970
C       -0.180226841      1.559292118     -0.407860970
C       -0.180226841      1.503191118      0.986935030
N       -0.180226841      0.360945118      1.29018350
C       -0.180226841     -0.781300882      0.986935030
C       -0.180226841     -0.837401882     -0.407860970
H       -0.180226841      0.360945118     -2.206546970
H       -0.180226841      2.517950118     -0.917077970
H       -0.180226841      2.421289118      1.572099030
H       -0.180226841     -1.699398882      1.572099030
H       -0.180226841     -1.796059882     -0.917077970"#;

        let xyz: Xyz = pyridine_xyz.parse().unwrap();
        assert_eq!(xyz.atoms.len(), 11);
        assert_eq!(xyz.title, "Pyridine");
        assert_eq!(xyz.atoms[0].symbol, "C");
        assert_eq!(xyz.atoms[0].x, -0.180226841);
        assert_eq!(xyz.atoms[0].y, 0.360945118);
        assert_eq!(xyz.atoms[0].z, -1.120304970);

        assert_eq!(xyz.atoms[10].symbol, "H");
        assert_eq!(xyz.atoms[10].x, -0.180226841);
        assert_eq!(xyz.atoms[10].y, -1.796059882);
        assert_eq!(xyz.atoms[10].z, -0.917077970);
    }
}
