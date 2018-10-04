use arc::Arc;
use fst_traits::{CoreFst, MutableFst};
use semirings::Semiring;
use std::cmp;
use Label;
use Result;

/// Turns a list of input labels and output labels to a linear FST.
///
/// # Example
///
/// ```
/// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// use rustfst::fst_impls::VectorFst;
/// use rustfst::semirings::{ProbabilityWeight, Semiring};
/// use rustfst::utils::transducer;
/// use rustfst::arc::Arc;
///
/// let labels_input = vec![32, 43, 21];
/// let labels_output = vec![53, 18, 89];
///
/// let fst : VectorFst<ProbabilityWeight> = transducer(labels_input.clone().into_iter(), labels_output.clone().into_iter()).unwrap();
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
/// fst_ref.set_start(&s1).unwrap();
/// fst_ref.set_final(&s4, ProbabilityWeight::one()).unwrap();
///
/// fst_ref.add_arc(&s1, Arc::new(labels_input[0], labels_output[0], ProbabilityWeight::one(), s2)).unwrap();
/// fst_ref.add_arc(&s2, Arc::new(labels_input[1], labels_output[1], ProbabilityWeight::one(), s3)).unwrap();
/// fst_ref.add_arc(&s3, Arc::new(labels_input[2], labels_output[2], ProbabilityWeight::one(), s4)).unwrap();
///
/// assert_eq!(fst, fst_ref);
/// ```
pub fn transducer<T: Iterator<Item = Label>, F: MutableFst>(
    labels_input: T,
    labels_output: T,
) -> Result<F> {
    let mut vec_labels_input: Vec<_> = labels_input.collect();
    let mut vec_labels_output: Vec<_> = labels_output.collect();

    let max_size = cmp::max(vec_labels_input.len(), vec_labels_output.len());

    vec_labels_input.resize(max_size, 0);
    vec_labels_output.resize(max_size, 0);

    let mut fst = F::new();
    let mut state_cour = fst.add_state();
    fst.set_start(&state_cour)?;

    for (i, o) in vec_labels_input.iter().zip(vec_labels_output.iter()) {
        let new_state = fst.add_state();
        fst.add_arc(
            &state_cour,
            Arc::new(*i, *o, <F as CoreFst>::W::one(), new_state),
        )?;
        state_cour = new_state;
    }

    fst.set_final(&state_cour, <F as CoreFst>::W::one())?;

    Ok(fst)
}

/// Turns a list of labels into an acceptor (FST with the same labels for both input and output).
///
/// # Example
///
/// ```
/// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
/// use rustfst::fst_impls::VectorFst;
/// use rustfst::semirings::{ProbabilityWeight, Semiring};
/// use rustfst::utils::acceptor;
/// use rustfst::arc::Arc;
///
/// let labels = vec![32, 43, 21];
///
/// let fst : VectorFst<ProbabilityWeight> = acceptor(labels.clone().into_iter()).unwrap();
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
/// fst_ref.set_start(&s1).unwrap();
/// fst_ref.set_final(&s4, ProbabilityWeight::one()).unwrap();
///
/// fst_ref.add_arc(&s1, Arc::new(labels[0], labels[0], ProbabilityWeight::one(), s2)).unwrap();
/// fst_ref.add_arc(&s2, Arc::new(labels[1], labels[1], ProbabilityWeight::one(), s3)).unwrap();
/// fst_ref.add_arc(&s3, Arc::new(labels[2], labels[2], ProbabilityWeight::one(), s4)).unwrap();
///
/// assert_eq!(fst, fst_ref);
///
//§ ```
pub fn acceptor<T: Iterator<Item = Label>, F: MutableFst>(labels: T) -> Result<F> {
    let vec_labels: Vec<_> = labels.collect();
    let mut fst = F::new();
    let mut state_cour = fst.add_state();
    fst.set_start(&state_cour)?;

    for l in &vec_labels {
        let new_state = fst.add_state();
        fst.add_arc(
            &state_cour,
            Arc::new(*l, *l, <F as CoreFst>::W::one(), new_state),
        )?;
        state_cour = new_state;
    }

    fst.set_final(&state_cour, <F as CoreFst>::W::one())?;

    Ok(fst)
}