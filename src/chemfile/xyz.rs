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

use winnow::{
    ascii::{alphanumeric1, dec_uint, float, line_ending, space0, space1, till_line_ending},
    combinator::{preceded, repeat, seq, terminated},
    error::{StrContext, StrContextValue},
    PResult, Parser,
};

use super::ParseChemfileError;

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

    /// Parse a part of text into an `Xyz` according to the XYZ file format.
    /// It allows the text to have extra spaces at the end of the line.
    /// It will not consume all the text, allowing multiple XYZ records in the same text.
    pub fn parser(input: &mut &str) -> PResult<Self> {
        let loose_line_ending = (space0, line_ending);
        let n_atoms: usize = terminated(dec_uint, loose_line_ending)
            .context(StrContext::Label("number of atoms"))
            .parse_next(input)?;
        seq! {
            Xyz {
                title: till_line_ending.parse_to().context(StrContext::Label("title")),
                atoms: repeat(n_atoms, preceded(loose_line_ending, XyzAtom::parser)).context(StrContext::Label("atoms list")),
                _: space0 // Consume extra spaces but preserve line endings
            }
        }
        .parse_next(input)
    }
}
impl std::str::FromStr for Xyz {
    type Err = ParseChemfileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser
            .context(StrContext::Label("XYZ file"))
            .parse(s)
            .map_err(|e| ParseChemfileError::from_parse(e, s))
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
    fn parser(input: &mut &str) -> PResult<Self> {
        let mut spaced_double = preceded(space1, float);
        seq! {
            XyzAtom {
                symbol: alphanumeric1.parse_to(),
                x: spaced_double,
                y: spaced_double,
                z: spaced_double,
            }
        }
        .context(StrContext::Label("xyz atom"))
        .context(StrContext::Expected(StrContextValue::Description(
            "<atom_symbol> <x> <y> <z>",
        )))
        .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // from https://en.wikipedia.org/wiki/XYZ_file_format
    const PYRIDINE_XYZ: &str = r#"11
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

    #[test]
    fn test_xyz_parse() {
        let xyz: Xyz = PYRIDINE_XYZ.parse().unwrap();
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

    #[test]
    fn test_xyz_parse_extra_line() {
        let xyz_str = format!("{}\n", PYRIDINE_XYZ);
        assert!(Xyz::parser(&mut xyz_str.as_str()).is_ok());
        assert!(xyz_str.parse::<Xyz>().is_err());
    }

    #[test]
    fn test_xyz_atom_parse() {
        let atom = "C 0.0 0.0 0.0";
        let atom = XyzAtom::parser.parse(atom).unwrap();
        assert_eq!(atom.symbol, "C");
        assert_eq!(atom.x, 0.0);
        assert_eq!(atom.y, 0.0);
        assert_eq!(atom.z, 0.0);

        let atom = "H 1.0 2.0 3.0";
        let atom = XyzAtom::parser.parse(atom).unwrap();
        assert_eq!(atom.symbol, "H");
        assert_eq!(atom.x, 1.0);
        assert_eq!(atom.y, 2.0);
        assert_eq!(atom.z, 3.0);

        let atom = "H       -0.180226841     -1.699398882      1.572099030";
        let atom = XyzAtom::parser.parse(atom).unwrap();
        assert_eq!(atom.symbol, "H");
        assert_eq!(atom.x, -0.180226841);
        assert_eq!(atom.y, -1.699398882);
        assert_eq!(atom.z, 1.572099030);
    }
}
