use crate::arc::Arc;
use crate::fst_traits::{CoreFst, MutableFst};
use crate::semirings::Semiring;
use crate::Label;

use std::cmp;

/// Turns a list of input labels and output labels into a linear FST.
/// The only accepted path in the FST has for input `labels_input` and for output `labels_output`.
///
/// # Example
///
/// ```
/// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::semirings::{ProbabilityWeight, Semiring};
/// # use rustfst::utils::transducer;
/// # use rustfst::Arc;
/// let labels_input = vec![32, 43, 21];
/// let labels_output = vec![53, 18, 89];
///
/// let fst : VectorFst<ProbabilityWeight> = transducer(labels_input.clone().into_iter(), labels_output.clone().into_iter());
///
/// assert_eq!(fst.num_states(), 4);
///
/// // The transducer function produces the same FST as the following code
///
/// let mut fst_ref = VectorFst::new();
/// let s1 = fst_ref.add_state();
/// let s2 = fst_ref.add_state();
/// let s3 = fst_ref.add_state();
/// let s4 = fst_ref.add_state();
///
/// fst_ref.set_start(s1).unwrap();
/// fst_ref.set_final(s4, ProbabilityWeight::ONE).unwrap();
///
/// fst_ref.add_arc(s1, Arc::new(labels_input[0], labels_output[0], ProbabilityWeight::ONE, s2)).unwrap();
/// fst_ref.add_arc(s2, Arc::new(labels_input[1], labels_output[1], ProbabilityWeight::ONE, s3)).unwrap();
/// fst_ref.add_arc(s3, Arc::new(labels_input[2], labels_output[2], ProbabilityWeight::ONE, s4)).unwrap();
///
/// assert_eq!(fst, fst_ref);
/// ```
pub fn transducer<T: Iterator<Item = Label>, F: MutableFst>(
    labels_input: T,
    labels_output: T,
) -> F {
    let mut vec_labels_input: Vec<_> = labels_input.collect();
    let mut vec_labels_output: Vec<_> = labels_output.collect();

    let max_size = cmp::max(vec_labels_input.len(), vec_labels_output.len());

    vec_labels_input.resize(max_size, 0);
    vec_labels_output.resize(max_size, 0);

    let mut fst = F::new();
    let mut state_cour = fst.add_state();

    // Can't fail as the state has just been added
    fst.set_start(state_cour).unwrap();

    for (i, o) in vec_labels_input.iter().zip(vec_labels_output.iter()) {
        let new_state = fst.add_state();

        // Can't fail as the state has just been added
        fst.add_arc(
            state_cour,
            Arc::new(*i, *o, <F as CoreFst>::W::ONE, new_state),
        )
        .unwrap();

        state_cour = new_state;
    }

    // Can't fail as the state has just been added
    fst.set_final(state_cour, <F as CoreFst>::W::ONE).unwrap();

    fst
}

/// Turns a list of labels into a linear acceptor (FST with the same labels for both input and output).
/// The only accepted path in the acceptor will be `labels`.
///
/// # Example
///
/// ```
/// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// use rustfst::fst_impls::VectorFst;
/// use rustfst::semirings::{ProbabilityWeight, Semiring};
/// use rustfst::utils::acceptor;
/// use rustfst::Arc;
///
/// let labels = vec![32, 43, 21];
///
/// let fst : VectorFst<ProbabilityWeight> = acceptor(labels.clone().into_iter());
///
/// assert_eq!(fst.num_states(), 4);
///
/// // The acceptor function produces the same FST as the following code
///
/// let mut fst_ref = VectorFst::new();
/// let s1 = fst_ref.add_state();
/// let s2 = fst_ref.add_state();
/// let s3 = fst_ref.add_state();
/// let s4 = fst_ref.add_state();
///
/// fst_ref.set_start(s1).unwrap();
/// fst_ref.set_final(s4, ProbabilityWeight::ONE).unwrap();
///
/// fst_ref.add_arc(s1, Arc::new(labels[0], labels[0], ProbabilityWeight::ONE, s2)).unwrap();
/// fst_ref.add_arc(s2, Arc::new(labels[1], labels[1], ProbabilityWeight::ONE, s3)).unwrap();
/// fst_ref.add_arc(s3, Arc::new(labels[2], labels[2], ProbabilityWeight::ONE, s4)).unwrap();
///
/// assert_eq!(fst, fst_ref);
///
/// ```
pub fn acceptor<T: Iterator<Item = Label>, F: MutableFst>(labels: T) -> F {
    let vec_labels: Vec<_> = labels.collect();
    let mut fst = F::new();
    let mut state_cour = fst.add_state();

    // Can't fail as the state has just been added
    fst.set_start(state_cour).unwrap();

    for l in &vec_labels {
        let new_state = fst.add_state();

        // Can't fail as the state has just been added
        fst.add_arc(
            state_cour,
            Arc::new(*l, *l, <F as CoreFst>::W::ONE, new_state),
        )
        .unwrap();
        state_cour = new_state;
    }

    // Can't fail as the state has just been added
    fst.set_final(state_cour, <F as CoreFst>::W::ONE).unwrap();

    fst
}

/// Easily creates an acceptor from a list of labels.
///
/// This will return a linear FST with one arc for each label given
/// (same input and output, weight one).
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::utils;
/// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::semirings::{ProbabilityWeight, Semiring};
/// # use rustfst::utils::acceptor;
/// # use rustfst::Arc;
/// let fst : VectorFst<ProbabilityWeight> = acceptor![1,2,3];
/// # }
/// ```
#[macro_export]
macro_rules! acceptor {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = vec![$($x),*];
            acceptor(temp_vec.clone().into_iter())
        }
    };
}

/// Easily creates a transducer from two lists of labels.
///
/// This will return a linear FST. The only accepted path in the FST has for input the first
/// of labels and for output the second list of labels.
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::utils;
/// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::semirings::{ProbabilityWeight, Semiring};
/// # use rustfst::utils::transducer;
/// # use rustfst::Arc;
/// let fst : VectorFst<ProbabilityWeight> = transducer![1,2,3 => 1,2,4];
/// # }
/// ```
#[macro_export]
macro_rules! transducer {
    ( $( $x:expr ),* => $( $y:expr ),* ) => {
        {
            let mut temp_vec_input = vec![$($x),*];
            let mut temp_vec_output = vec![$($y),*];
            transducer(
                temp_vec_input.clone().into_iter(),
                temp_vec_output.clone().into_iter()
            )
        }
    };
}

/// Creates a Path containing the arguments.
///
/// There are multiple forms to this macro :
///
/// - Create an unweighted linear acceptor :
///
/// This will return a linear FST with one arc for each label given
/// (same input and output, weight one).
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::utils;
/// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst, PathsIterator};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::semirings::{ProbabilityWeight, Semiring};
/// # use rustfst::utils::acceptor;
/// # use rustfst::{Arc, Path};
/// let fst : VectorFst<ProbabilityWeight> = fst![1,2,3];
/// assert_eq!(fst.paths_iter().count(), 1);
/// assert_eq!(fst.paths_iter().next().unwrap(), fst_path![1,2,3]);
/// # }
/// ```
///
/// - Create an unweighted linear transducer from two list of labels :
///
/// The only accepted path in the FST has for input the first
/// list of labels and for output the second list of labels.
///
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::utils;
/// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst, PathsIterator};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::semirings::{ProbabilityWeight, Semiring};
/// # use rustfst::utils::transducer;
/// # use rustfst::{Arc, Path};
/// let fst : VectorFst<ProbabilityWeight> = fst![1,2,3 => 1,2,4];
/// assert_eq!(fst.paths_iter().count(), 1);
/// assert_eq!(fst.paths_iter().next().unwrap(), fst_path![1,2,3 => 1,2,4]);
/// # }
/// ```
///
#[macro_export]
macro_rules! fst {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = vec![$($x),*];
            acceptor(temp_vec.clone().into_iter())
        }
    };
    ( $( $x:expr ),* => $( $y:expr ),* ) => {
        {
            let mut temp_vec_input = vec![$($x),*];
            let mut temp_vec_output = vec![$($y),*];
            transducer(
                temp_vec_input.clone().into_iter(),
                temp_vec_output.clone().into_iter()
            )
        }
    };
}