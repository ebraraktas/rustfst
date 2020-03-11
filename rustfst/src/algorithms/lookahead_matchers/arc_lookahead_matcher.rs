use failure::Fallible;

use crate::algorithms::lookahead_matchers::LookaheadMatcher;
use crate::algorithms::matchers::{MatchType, Matcher, MatcherFlags, IterItemMatcher};
use crate::fst_traits::{CoreFst, Fst};
use crate::semirings::Semiring;
use crate::{Arc, Label, StateId, NO_LABEL, NO_STATE_ID, EPS_LABEL};
use unsafe_unwrap::UnsafeUnwrap;

#[derive(Debug)]
struct ArcLookAheadMatcher<'fst, W: Semiring, M: Matcher<'fst, W>> {
    // matcher fst
    fst: &'fst M::F,
    matcher: M,
    weight: W,
    prefix_arc: Arc<W>,

    // Flags to customize the behaviour
    flags: MatcherFlags,
}

impl<'fst, W: Semiring + 'fst, M: Matcher<'fst, W>> Matcher<'fst, W>
    for ArcLookAheadMatcher<'fst, W, M>
{
    type F = M::F;
    type Iter = M::Iter;

    fn new(fst: &'fst Self::F, match_type: MatchType) -> Fallible<Self> {
        Ok(Self {
            fst,
            matcher: M::new(fst, match_type)?,
            flags: MatcherFlags::LOOKAHEAD_NON_EPSILONS
                | MatcherFlags::LOOKAHEAD_EPSILONS
                | MatcherFlags::LOOKAHEAD_WEIGHT
                | MatcherFlags::LOOKAHEAD_PREFIX,
            prefix_arc: Arc::new(0, 0, W::one(), NO_STATE_ID),
            weight: W::one(),
        })
    }

    fn iter(&self, state: usize, label: usize) -> Fallible<Self::Iter> {
        self.matcher.iter(state, label)
    }

    fn final_weight(&self, state: usize) -> Fallible<Option<&'fst W>> {
        self.matcher.final_weight(state)
    }

    fn match_type(&self) -> MatchType {
        self.matcher.match_type()
    }

    fn flags(&self) -> MatcherFlags {
        self.matcher.flags()
            | MatcherFlags::INPUT_LOOKAHEAD_MATCHER
            | MatcherFlags::OUTPUT_LOOKAHEAD_MATCHER
            | self.flags
    }
}

impl<'fst, W: Semiring + 'fst, M: Matcher<'fst, W>> LookaheadMatcher<'fst, W>
    for ArcLookAheadMatcher<'fst, W, M>
{
    fn lookahead_fst<LF: Fst<W = W>>(
        &mut self,
        matcher_state: StateId,
        lfst: &LF,
        lfst_state: StateId,
    ) -> Fallible<bool> {
        let mut result = false;
        let mut nprefix = 0;
        if self.flags.contains(MatcherFlags::LOOKAHEAD_WEIGHT) {
            self.clear_lookahead_weight();
        }
        if self.flags.contains(MatcherFlags::LOOKAHEAD_PREFIX) {
            self.clear_lookahead_prefix();
        }
        if self.fst.is_final(matcher_state)? && lfst.is_final(lfst_state)? {
            if !self
                .flags
                .contains(MatcherFlags::LOOKAHEAD_WEIGHT | MatcherFlags::LOOKAHEAD_PREFIX)
            {
                return Ok(true);
            }
            nprefix += 1;
            if self.flags.contains(MatcherFlags::LOOKAHEAD_WEIGHT) {
                unsafe {
                    let fw_matcher_state = self
                        .fst
                        .final_weight_unchecked(matcher_state)
                        .unsafe_unwrap();
                    let fw_lfst_state = lfst.final_weight_unchecked(lfst_state).unsafe_unwrap();
                    self.weight
                        .plus_assign(fw_matcher_state.times(fw_lfst_state)?)?;
                }
            }
            result = true;
        }
        let mut iter = self.iter(matcher_state, NO_LABEL)?.peekable();
        if iter.peek().is_some() {
            if !self
                .flags
                .contains(MatcherFlags::LOOKAHEAD_WEIGHT | MatcherFlags::LOOKAHEAD_PREFIX)
            {
                return Ok(true);
            }
            nprefix += 1;
            if self.flags.contains(MatcherFlags::LOOKAHEAD_WEIGHT) {
                for arc in iter {
                    match arc {
                        IterItemMatcher::Arc(a) => self.weight.plus_assign(&a.weight)?,
                        IterItemMatcher::EpsLoop => self.weight.plus_assign(W::one())?
                    };
                }
            }
            result = true;
        }

        let match_type = self.match_type();
        for arc in lfst.arcs_iter(lfst_state)? {
            match match_type {
                MatchType::MatchInput => arc.olabel,
                MatchType::MatchOutput => arc.ilabel,
                _ => bail!("Bad match type")
            }
            if label == EPS_LABEL {
                unimplemented!()
            } else {
                unimplemented!()
            }
        }

        // for arc in self.iter(matcher_state, NO_LABEL) {

        //
        // }

        unimplemented!()
    }

    fn lookahead_label(&self, state: StateId, label: Label) -> Fallible<bool> {
        let mut it = self.matcher.iter(state, label)?;
        Ok(it.next().is_some())
    }

    fn lookahead_prefix(&self) -> bool {
        unimplemented!()
    }

    fn lookahead_weight(&self) -> W {
        unimplemented!()
    }

    fn prefix_arc_mut(&mut self) -> &mut Arc<W> {
        &mut self.prefix_arc
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}
