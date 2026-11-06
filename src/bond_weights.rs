//! Dimensionless 1-2 / 1-3 / … / 1-N scale weights: [`BondDistanceWeights`].
//!
//! molrs 0.20 removed this type. Its replacement is a 1-2/1-3/1-4 pairs
//! block plus a force-field special-bond triple, which cannot express a
//! variable-length 1-N tail or a fractional packing weight. The table stays
//! molpack's.

use std::fmt;

use molrs::op::F;

/// Bond-distance scale weights: slot 0 is 1-2, last slot is the 1-N tail.
///
/// Weights are dimensionless in `[0, 1]`: `0.0` fully exempts that distance,
/// `1.0` leaves it at full strength, and a fraction (Amber 1-4 `0.5`) is legal.
/// There is no [`Default`] — callers write the table;
/// [`from_exclusion_depth`](Self::from_exclusion_depth) is the named one-liner
/// for "exempt up to this hop, then 1".
///
/// Slot `k` (0-based) is the weight at bond distance `k + 1`, and the last
/// slot is the 1-N tail for every farther hop. A length-3 table is **not** a
/// LAMMPS `special_bonds` triple: LAMMPS charmm `0 0 0` is `[0, 0, 0, 1]`
/// here; `new(vec![0.0, 0.0, 0.0])` is a zero tail that exempts the rest of
/// the connected component.
///
/// # Examples
///
/// ```
/// use molpack::BondDistanceWeights;
///
/// let table = BondDistanceWeights::from_exclusion_depth(3);
/// assert_eq!(table.as_slice(), &[0.0, 0.0, 0.0, 1.0]);
/// assert_eq!(table.weight(0), 0.0);
/// assert_eq!(table.weight(3), 0.0);
/// assert_eq!(table.weight(4), 1.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BondDistanceWeights(Vec<F>);

/// Why [`BondDistanceWeights::new`] refused a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondWeightsError {
    detail: String,
}

impl fmt::Display for BondWeightsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.detail)
    }
}

impl std::error::Error for BondWeightsError {}

impl BondDistanceWeights {
    /// Construct a table from explicit per-distance weights.
    ///
    /// `weights[0]` is bond-distance 1 (1-2). The last entry is the 1-N tail.
    /// A zero tail is legal.
    ///
    /// # Errors
    ///
    /// [`BondWeightsError`] if `weights` is empty or any entry is outside
    /// `[0, 1]` or not finite.
    pub fn new(weights: Vec<F>) -> Result<Self, BondWeightsError> {
        if weights.is_empty() {
            return Err(BondWeightsError {
                detail: "bond-distance weights must be non-empty".to_string(),
            });
        }
        for (i, &w) in weights.iter().enumerate() {
            if !(0.0..=1.0).contains(&w) {
                return Err(BondWeightsError {
                    detail: format!(
                        "bond-distance weight[{i}] = {w} is outside [0, 1] or not finite"
                    ),
                });
            }
        }
        Ok(Self(weights))
    }

    /// Exempt bond distances `1..=depth` and set the 1-N tail to `1.0`.
    ///
    /// Equivalent to `vec![0.0; depth]` followed by `1.0`. Depth 3 is
    /// `[0, 0, 0, 1]` (1-2 / 1-3 / 1-4 exempt, 1-5+ full strength).
    pub fn from_exclusion_depth(depth: usize) -> Self {
        let mut weights = vec![0.0; depth];
        weights.push(1.0);
        Self(weights)
    }

    /// Scale weight at graph distance `distance`.
    ///
    /// Distance 0 is the atom itself and is always `0.0`. Distances past the
    /// last slot return the 1-N tail (the last entry).
    pub fn weight(&self, distance: usize) -> F {
        if distance == 0 {
            return 0.0;
        }
        self.0[(distance - 1).min(self.0.len() - 1)]
    }

    /// The stored table: index 0 is 1-2, last index is the 1-N tail.
    pub fn as_slice(&self) -> &[F] {
        &self.0
    }
}

/// Per-atom partners this table exempts, sorted, the atom itself included.
///
/// A partner is every atom whose [`BondDistanceWeights::weight`] is `0`.
/// [`Topology::distances`](molrs::core::Topology::distances) reports an
/// unreachable atom as `-1`; that hop is the 1-N tail, so a zero tail still
/// exempts the rest of the molecule.
pub(crate) fn excluded_partners(
    topo: &molrs::core::Topology,
    weights: &BondDistanceWeights,
) -> Vec<Vec<usize>> {
    let n = topo.n_atoms();
    (0..n)
        .map(|i| {
            let dist = topo.distances(i);
            let mut partners: Vec<usize> = (0..n)
                .filter(|&j| {
                    let hops = dist[j];
                    let hops = if hops < 0 { usize::MAX } else { hops as usize };
                    weights.weight(hops) == 0.0
                })
                .collect();
            partners.sort_unstable();
            partners
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_is_refused() {
        let err = BondDistanceWeights::new(vec![]).unwrap_err();
        assert!(err.to_string().contains("non-empty"));
    }

    #[test]
    fn new_out_of_range_is_refused() {
        assert!(BondDistanceWeights::new(vec![0.0, 1.5]).is_err());
    }

    #[test]
    fn new_nan_is_refused() {
        assert!(BondDistanceWeights::new(vec![0.0, f64::NAN]).is_err());
    }

    #[test]
    fn new_negative_is_refused() {
        assert!(BondDistanceWeights::new(vec![-0.1]).is_err());
    }

    #[test]
    fn new_infinity_is_refused() {
        assert!(BondDistanceWeights::new(vec![0.0, f64::INFINITY]).is_err());
    }

    #[test]
    fn new_single_zero_succeeds_with_zero_tail() {
        let table = BondDistanceWeights::new(vec![0.0]).expect("zero tail is legal");
        assert_eq!(table.weight(10), 0.0);
    }

    #[test]
    fn new_length_three_zeros_succeeds_with_zero_tail() {
        let table = BondDistanceWeights::new(vec![0.0, 0.0, 0.0]).expect("zero tail is legal");
        assert_eq!(table.weight(10), 0.0);
    }

    #[test]
    fn from_exclusion_depth_three_is_four_slots() {
        assert_eq!(
            BondDistanceWeights::from_exclusion_depth(3).as_slice(),
            &[0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn weight_on_exclusion_depth_three_table() {
        let table = BondDistanceWeights::from_exclusion_depth(3);
        assert_eq!(table.weight(0), 0.0);
        assert_eq!(table.weight(1), 0.0);
        assert_eq!(table.weight(3), 0.0);
        assert_eq!(table.weight(4), 1.0);
        assert_eq!(table.weight(97), 1.0);
    }

    #[test]
    fn weight_zero_is_self_not_first_slot() {
        let table = BondDistanceWeights::new(vec![1.0]).expect("unit weight is legal");
        assert_eq!(table.weight(0), 0.0);
        assert_eq!(table.weight(1), 1.0);
    }

    #[test]
    fn as_slice_matches_constructed_weights() {
        let table =
            BondDistanceWeights::new(vec![0.0, 0.5, 1.0]).expect("fractional weight is legal");
        assert_eq!(table.as_slice(), &[0.0, 0.5, 1.0]);
    }

    #[test]
    fn excluded_partners_on_a_chain_include_self_through_the_exempt_hops() {
        use molrs::core::Topology;

        let mut topo = Topology::with_atoms(5);
        topo.add_bonds(&[[0, 1], [1, 2], [2, 3], [3, 4]]);
        let depth3 = excluded_partners(&topo, &BondDistanceWeights::from_exclusion_depth(3));
        assert_eq!(depth3[0], vec![0, 1, 2, 3]);
        assert!(!depth3[0].contains(&4));
        let depth1 = excluded_partners(&topo, &BondDistanceWeights::from_exclusion_depth(1));
        assert_eq!(depth1[0], vec![0, 1]);
    }
}
