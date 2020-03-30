use std::cell::RefCell;
use std::rc::Rc;

use failure::Fallible;

use crate::algorithms::compose_filters::ComposeFilter;
use crate::algorithms::matchers::Matcher;
use crate::semirings::Semiring;
use crate::{Arc, NO_LABEL};

#[derive(Debug)]
pub struct MultiEpsFilter<F> {
    filter: F,
    keep_multi_eps: bool,
}

impl<'fst1, 'fst2, W: Semiring + 'fst1 + 'fst2, F: ComposeFilter<'fst1, 'fst2, W>>
    ComposeFilter<'fst1, 'fst2, W> for MultiEpsFilter<F>
{
    type M1 = F::M1;
    type M2 = F::M2;
    type FS = F::FS;

    fn new<IM1: Into<Option<Rc<RefCell<Self::M1>>>>, IM2: Into<Option<Rc<RefCell<Self::M2>>>>>(
        fst1: &'fst1 <Self::M1 as Matcher<'fst1, W>>::F,
        fst2: &'fst2 <Self::M2 as Matcher<'fst2, W>>::F,
        m1: IM1,
        m2: IM2,
    ) -> Fallible<Self> {
        Ok(Self {
            filter: F::new(fst1, fst2, m1, m2)?,
            keep_multi_eps: false,
        })
    }

    fn start(&self) -> Self::FS {
        self.filter.start()
    }

    fn set_state(&mut self, s1: usize, s2: usize, filter_state: &Self::FS) -> Fallible<()> {
        self.filter.set_state(s1, s2, filter_state)
    }

    fn filter_arc(&mut self, arc1: &mut Arc<W>, arc2: &mut Arc<W>) -> Fallible<Self::FS> {
        let opt_fs = self.filter.filter_arc(arc1, arc2)?;
        if self.keep_multi_eps {
            if arc1.olabel == NO_LABEL {
                arc1.ilabel = arc2.ilabel;
            }

            if arc2.ilabel == NO_LABEL {
                arc2.olabel = arc1.olabel;
            }
        }
        Ok(opt_fs)
    }

    fn filter_final(&self, w1: &mut W, w2: &mut W) -> Fallible<()> {
        self.filter.filter_final(w1, w2)
    }

    fn matcher1(&self) -> Rc<RefCell<Self::M1>> {
        Rc::clone(&self.filter.matcher1())
    }

    fn matcher2(&self) -> Rc<RefCell<Self::M2>> {
        Rc::clone(&self.filter.matcher2())
    }
}
