use std::hash::{Hash, Hasher};

use crate::semirings::Semiring;
use crate::{Label, EPS_LABEL};

/// Structure representing a path in a FST
/// (list of input labels, list of output labels and total weight).
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct Path<W: Semiring> {
    /// List of input labels.
    pub ilabels: Vec<Label>,
    /// List of output labels.
    pub olabels: Vec<Label>,
    /// Total weight of the path computed by multiplying the weight of each transition.
    pub weight: W,
}

impl<W: Semiring> Path<W> {

    /// Creates a new Path.
    pub fn new(ilabels: Vec<Label>, olabels: Vec<Label>, weight: W) -> Self {
        Path {
            ilabels,
            olabels,
            weight,
        }
    }

    /// Adds the content of an FST transition to the Path.
    /// Labels are added at the end of the corresponding vectors and the weight
    /// is multiplied by the total weight already stored in the Path.
    pub fn add_to_path(&mut self, ilabel: Label, olabel: Label, weight: W) {
        if ilabel != EPS_LABEL {
            self.ilabels.push(ilabel);
        }

        if olabel != EPS_LABEL {
            self.olabels.push(olabel);
        }

        self.weight *= weight
    }

    /// Add a single weight to the Path by multiplying the weight by the total weight of the path.
    pub fn add_weight(&mut self, weight: W) {
        self.weight *= weight
    }

    /// Append a Path to the current Path. Labels are appended and weights multiplied.
    pub fn concat(&mut self, other: Path<W>) {
        self.ilabels.extend(other.ilabels);
        self.olabels.extend(other.olabels);
        self.weight *= other.weight;
    }
}

impl<W: Semiring> Default for Path<W> {

    /// Creates an empty path with a weight one.
    fn default() -> Self {
        Path {
            ilabels: vec![],
            olabels: vec![],
            weight: W::ONE,
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl<W: Semiring + Hash + Eq> Hash for Path<W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ilabels.hash(state);
        self.olabels.hash(state);
        self.weight.hash(state);
    }
}

impl<W: Semiring + Hash + Eq> Eq for Path<W> {}

/// Creates a Path containing the arguments.
///
/// There are multiple forms to this macro :
///
/// - Create an unweighted acceptor path :
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::semirings::{IntegerWeight, Semiring};
/// # use rustfst::Path;
/// let path : Path<IntegerWeight> = fst_path![1,2,3];
/// assert_eq!(path.ilabels, vec![1,2,3]);
/// assert_eq!(path.olabels, vec![1,2,3]);
/// assert_eq!(path.weight, IntegerWeight::ONE);
/// # }
/// ```
///
/// - Create an unweighted transducer path :
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::semirings::{IntegerWeight, Semiring};
/// # use rustfst::Path;
/// let path : Path<IntegerWeight> = fst_path![1,2,3 => 1,2,4];
/// assert_eq!(path.ilabels, vec![1,2,3]);
/// assert_eq!(path.olabels, vec![1,2,4]);
/// assert_eq!(path.weight, IntegerWeight::ONE);
/// # }
/// ```
///
/// - Create a weighted acceptor path :
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::semirings::{IntegerWeight, Semiring};
/// # use rustfst::Path;
/// let path : Path<IntegerWeight> = fst_path![1,2,3; 18];
/// assert_eq!(path.ilabels, vec![1,2,3]);
/// assert_eq!(path.olabels, vec![1,2,3]);
/// assert_eq!(path.weight, IntegerWeight::new(18));
/// # }
/// ```
///
/// - Create a weighted transducer path :
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::semirings::{IntegerWeight, Semiring};
/// # use rustfst::Path;
/// let path : Path<IntegerWeight> = fst_path![1,2,3 => 1,2,4; 18];
/// assert_eq!(path.ilabels, vec![1,2,3]);
/// assert_eq!(path.olabels, vec![1,2,4]);
/// assert_eq!(path.weight, IntegerWeight::new(18));
/// # }
/// ```
///
#[macro_export]
macro_rules! fst_path {
    ( $( $x:expr ),*) => {
        {
            Path::new(
                vec![$($x),*],
                vec![$($x),*],
                Semiring::ONE
            )
        }
    };
    ( $( $x:expr ),* => $( $y:expr ),* ) => {
        {
            Path::new(
                vec![$($x),*],
                vec![$($y),*],
                Semiring::ONE
            )
        }
    };
    ( $( $x:expr ),* ; $weight:expr) => {
        {
            Path::new(
                vec![$($x),*],
                vec![$($x),*],
                Semiring::new($weight)
            )
        }
    };
    ( $( $x:expr ),* => $( $y:expr ),* ; $weight:expr) => {
        {
            Path::new(
                vec![$($x),*],
                vec![$($y),*],
                Semiring::new($weight)
            )
        }
    };
}
