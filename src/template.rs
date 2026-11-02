//! What molpack reads off a template: Cartesian coordinate rows, and the
//! rotatable bonds of its bond graph.
//!
//! Crate-root leaf over molrs only. Coordinates are in Å (the crate's length
//! unit, as carried by the frame — nothing is converted).

use std::collections::{HashSet, VecDeque};

use molrs::core::Atomistic;
use molrs::core::BondOrder;
use molrs::core::Frame;
use molrs::core::Topology;
use molrs::core::keys;
use molrs::op::{F, Fnx3};
use molrs::perceive::{RotatableBond, UnknownBondPolicy, perceive_rotatable_bonds};

/// Coordinate rows of an `N × 3` array (as `molrs::core::Frame::coords` returns it), in
/// the array's row order — the `[x, y, z]` triples molpack's placement code
/// works in.
pub(crate) fn coord_rows(xyz: &Fnx3) -> Vec<[F; 3]> {
    xyz.rows().into_iter().map(|r| [r[0], r[1], r[2]]).collect()
}

/// The rotatable bonds of a template graph, each with its downstream atoms.
///
/// Formats that carry connectivity without orders — PDB `CONECT`, GROMACS
/// `.top`, XYZ `Connct`, a hand-built coarse-grain frame — read back
/// `BondOrder::Unknown`, because molrs reports what the file said rather than
/// guessing. molpack's policy, held here once for every solver: an unclassed
/// bond is a rotatable single bond ([`UnknownBondPolicy::AsSingle`]). A class
/// the input did state is kept.
pub(crate) fn rotatable_bonds(graph: &Atomistic) -> Vec<RotatableBond> {
    perceive_rotatable_bonds(graph, UnknownBondPolicy::AsSingle)
}

/// Rotatable bonds of a template frame, as unordered atom-index pairs.
///
/// Two readings of the same frame:
///
/// * **Atomistic.** `atoms.atomic_number` is present, or `atoms.element` can
///   be converted to it. The graph is [`Atomistic::from_frame`] (symbols go
///   through [`Element::fill_atomic_numbers`](molrs::core::Element::fill_atomic_numbers)
///   first) and the bonds come from [`rotatable_bonds`].
/// * **Frame.** Neither species column is present. A coarse-grain bead has no
///   atomic number, and packing does not invent one. The bond list on the
///   frame is read directly. `bond_type` is used when that column exists.
///
/// Both use the same class rule: an unstated class is a rotatable single, a
/// stated double, triple or aromatic bond is not, both ends must have degree
/// greater than 1, and a bond that closes a ring is not.
///
/// # Errors
///
/// The atomistic reading returns the [`Atomistic::from_frame`] error. A
/// number outside `0..=118`, or an element symbol that is not an element, is
/// not treated as a coarse-grain frame.
pub(crate) fn rotatable_pairs(
    frame: &Frame,
    topo: &Topology,
) -> Result<HashSet<(usize, usize)>, molrs::core::MolRsError> {
    if has_species(frame) {
        let graph = atomistic_from_frame(frame)?;
        return Ok(index_pairs(&rotatable_bonds(&graph)));
    }
    Ok(pairs_from_bond_table(frame, topo))
}

/// A frame molrs can read as an [`Atomistic`]: Z is already there, or the
/// element symbols can be converted to it.
fn has_species(frame: &Frame) -> bool {
    let Some(atoms) = frame.get("atoms") else {
        return false;
    };
    atoms.get(keys::ATOMIC_NUMBER).is_some() || atoms.get(keys::ELEMENT).is_some()
}

/// [`Atomistic::from_frame`], after turning an `element` column into
/// `atomic_number` when that is the species the frame actually carries.
fn atomistic_from_frame(frame: &Frame) -> Result<Atomistic, molrs::core::MolRsError> {
    let atoms = frame.get("atoms");
    let has_number = atoms
        .and_then(|block| block.get(keys::ATOMIC_NUMBER))
        .is_some();
    let has_element = atoms.and_then(|block| block.get(keys::ELEMENT)).is_some();
    if has_number || !has_element {
        return Atomistic::from_frame(frame);
    }
    let mut owned = frame.deep_copy();
    molrs::core::Element::fill_atomic_numbers(&mut owned)?;
    Atomistic::from_frame(&owned)
}

fn index_pairs(bonds: &[RotatableBond]) -> HashSet<(usize, usize)> {
    bonds
        .iter()
        .map(|bond| (bond.j.min(bond.k), bond.j.max(bond.k)))
        .collect()
}

fn pairs_from_bond_table(frame: &Frame, topo: &Topology) -> HashSet<(usize, usize)> {
    let Some(bonds) = frame.get("bonds") else {
        return HashSet::new();
    };
    let Some(atomi) = bonds.get(keys::ATOMI).and_then(|c| c.get::<u64>()) else {
        return HashSet::new();
    };
    let Some(atomj) = bonds.get(keys::ATOMJ).and_then(|c| c.get::<u64>()) else {
        return HashSet::new();
    };
    let mut pairs = HashSet::new();
    for (row, (&a, &b)) in atomi.iter().zip(atomj.iter()).enumerate() {
        let (a, b) = (a as usize, b as usize);
        if a == b || a >= topo.n_atoms() || b >= topo.n_atoms() {
            continue;
        }
        if !unstated_is_single(bond_class(bonds, row)) {
            continue;
        }
        if topo.degree(a) <= 1 || topo.degree(b) <= 1 {
            continue;
        }
        if bond_closes_cycle(topo, a, b) {
            continue;
        }
        pairs.insert((a.min(b), b.max(a)));
    }
    pairs
}

/// molpack's one reading of a bond class: missing, null, or `Unknown` counts
/// as a single bond. A class the frame did write is kept.
fn unstated_is_single(class: BondOrder) -> bool {
    matches!(class, BondOrder::Single | BondOrder::Unknown)
}

fn bond_class(bonds: &molrs::core::Block, row: usize) -> BondOrder {
    if bonds
        .validity(keys::BOND_TYPE)
        .is_some_and(|mask| mask.get(row) == Some(&false))
    {
        return BondOrder::Unknown;
    }
    let Some(column) = bonds.get(keys::BOND_TYPE) else {
        return BondOrder::Unknown;
    };
    let code = column
        .get::<u32>()
        .and_then(|values| values.get(ndarray::IxDyn(&[row])).copied())
        .or_else(|| {
            column
                .get::<u64>()
                .and_then(|values| values.get(ndarray::IxDyn(&[row])).copied())
                .map(|value| value as u32)
        })
        .or_else(|| {
            column
                .get::<i64>()
                .and_then(|values| values.get(ndarray::IxDyn(&[row])).copied())
                .map(|value| value as u32)
        })
        .or_else(|| {
            column
                .get::<f64>()
                .and_then(|values| values.get(ndarray::IxDyn(&[row])).copied())
                .map(|value| value.round() as u32)
        });
    match code {
        Some(code) => BondOrder::from_code(code),
        None => BondOrder::Unknown,
    }
}

/// True when `a—b` is not a bridge: `b` is still reachable from `a` without
/// that edge. A tree has no such bond. Growth refuses rings before it asks,
/// and this keeps the same answer on a frame that does contain one.
fn bond_closes_cycle(topo: &Topology, a: usize, b: usize) -> bool {
    let mut seen = vec![false; topo.n_atoms()];
    let mut queue = VecDeque::new();
    seen[a] = true;
    queue.push_back(a);
    while let Some(atom) = queue.pop_front() {
        for neighbor in topo.neighbors(atom) {
            if (atom == a && neighbor == b) || (atom == b && neighbor == a) {
                continue;
            }
            if neighbor == b {
                return true;
            }
            if !seen[neighbor] {
                seen[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    false
}

#[cfg(test)]
mod rotatable_tests {
    //! The single-bond fallback: a bond whose class the input left unstated is
    //! treated as rotatable, and one the input did state is not overridden.
    //! Raw molrs perception supplies neither — this is molpack's own policy,
    //! so it is pinned here.

    use super::rotatable_bonds;
    use crate::test_fixtures::{chain_frame, chain_graph, frame_from_parts, zigzag_coords};
    use molrs::core::Topology;
    use molrs::core::keys;
    use ndarray::Array1;
    use std::collections::HashSet;

    /// A frame carrying connectivity but no `bond_type` column — a PDB `CONECT`
    /// list, a hand-built coarse-grain chain — reads back with every bond
    /// `BondOrder::Unknown`, and molrs keeps it that way on purpose. Raw perception
    /// therefore finds nothing; `rotatable_bonds` supplies the single-bond
    /// fallback so it finds the same bonds a graph-built chain has.
    #[test]
    fn unclassed_bonds_fall_back_to_single() {
        let frame = chain_frame(10, 1.54);
        assert!(
            frame
                .get("atoms")
                .and_then(|atoms| atoms.get(keys::ATOMIC_NUMBER))
                .is_none(),
            "a coarse-grain fixture must not invent an atomic number"
        );
        let topo = Topology::from_frame(&frame).expect("chain topology");
        let from_frame = super::rotatable_pairs(&frame, &topo).expect("coarse-grain bonds");
        let from_graph: HashSet<(usize, usize)> = rotatable_bonds(&chain_graph(10))
            .iter()
            .map(|bond| (bond.j.min(bond.k), bond.j.max(bond.k)))
            .collect();

        // No `bond_type` on the frame, and no atomic number either. The
        // packer's fallback still recovers the same free bonds a graph-built
        // chain has.
        assert_eq!(
            from_frame, from_graph,
            "the fallback must recover the graph-built chain's bonds"
        );
        assert!(!from_frame.is_empty());
    }

    /// The fallback fills in only what was unstated. A bond the input explicitly
    /// classes as double must stay non-rotatable.
    #[test]
    fn fallback_does_not_override_a_stated_class() {
        let frame = chain_frame(6, 1.54);
        let mut with_double = frame.clone();
        {
            let bonds = with_double
                .get_mut("bonds")
                .expect("chain frame has a bonds block");
            let n = bonds.n_rows().expect("bond rows");
            let mut classes = vec![1u32; n];
            classes[2] = 2; // one stated double bond
            bonds
                .insert("bond_type", Array1::from_vec(classes).into_dyn(), None)
                .expect("bond_type column");
        }

        let topo = Topology::from_frame(&frame).expect("chain topology");
        let all_unclassed = super::rotatable_pairs(&frame, &topo).expect("unclassed");
        let one_double = super::rotatable_pairs(&with_double, &topo).expect("double");

        assert_eq!(
            one_double.len() + 1,
            all_unclassed.len(),
            "the stated double bond must stay out of the rotatable set"
        );
        let stated = (2, 3); // rotatable pairs are keyed (low, high)
        assert!(
            !one_double.contains(&stated),
            "bond row 2 is the stated double bond"
        );
    }

    /// A ring bond is not a free torsion, whether or not the frame names an
    /// element. Growth refuses the template before this runs; the predicate
    /// itself still has to agree.
    #[test]
    fn a_ring_bond_is_not_rotatable_without_an_atomic_number() {
        let frame = frame_from_parts(&zigzag_coords(4, 1.54), &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let topo = Topology::from_frame(&frame).expect("ring");
        assert!(
            super::rotatable_pairs(&frame, &topo)
                .expect("ring")
                .is_empty(),
            "every bond of a 4-ring is a ring bond"
        );
    }

    /// A frame that already carries `atomic_number` is an atomistic graph.
    /// `Atomistic::from_frame` is the reader; the answer matches perception.
    #[test]
    fn atomic_number_uses_atomistic_from_frame() {
        use molrs::core::Atomistic;

        let mut frame = chain_frame(10, 1.54);
        {
            let atoms = frame.get_mut("atoms").expect("atoms");
            atoms
                .insert(
                    keys::ATOMIC_NUMBER,
                    Array1::from_vec(vec![6u8; 10]).into_dyn(),
                    None,
                )
                .expect("atomic_number");
        }
        let topo = Topology::from_frame(&frame).expect("chain topology");
        let from_dispatch = super::rotatable_pairs(&frame, &topo).expect("atomistic");
        let from_graph: HashSet<(usize, usize)> =
            rotatable_bonds(&Atomistic::from_frame(&frame).expect("from_frame"))
                .iter()
                .map(|bond| (bond.j.min(bond.k), bond.j.max(bond.k)))
                .collect();
        assert_eq!(from_dispatch, from_graph);
        assert!(!from_dispatch.is_empty());
    }

    /// Element symbols are atomistic too: convert, then `Atomistic::from_frame`.
    /// `from_frame` itself still refuses a frame that has not been converted.
    #[test]
    fn element_symbols_use_atomistic_from_frame() {
        use molrs::core::Atomistic;

        let mut frame = chain_frame(6, 1.54);
        {
            let atoms = frame.get_mut("atoms").expect("atoms");
            atoms
                .insert(
                    keys::ELEMENT,
                    Array1::from_vec(vec!["C".to_string(); 6]).into_dyn(),
                    None,
                )
                .expect("element");
        }
        assert!(
            Atomistic::from_frame(&frame).is_err(),
            "from_frame does not read element symbols itself"
        );
        let topo = Topology::from_frame(&frame).expect("chain topology");
        let pairs = super::rotatable_pairs(&frame, &topo).expect("symbols converted");
        assert_eq!(pairs.len(), 3, "a 6-bead chain has 3 non-terminal bonds");
    }
}
