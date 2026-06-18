use crate::models::lit::Lit;

/// A lightweight handle to a [`Clause`] stored in the solver's clause arena.
///
/// Clauses live in a single owned arena (`SolverState::arena`) and are referred
/// to everywhere else (watch lists, reasons, the clause/learnt index lists) by
/// this `Copy` index instead of being cloned. Comparing two clauses for
/// identity is therefore a plain integer comparison.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClauseRef(pub u32);

impl ClauseRef {
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Debug)]
pub struct Clause {
    pub data: Vec<Lit>,
    pub is_learnt: bool,
    pub id: u32,
    pub activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: &[Lit], id: u32) -> Self;
    fn size(&self) -> i32;
    fn learnt(&self) -> bool;
}

impl IClause for Clause {
    fn new(_learnt: bool, _ps: &[Lit], id: u32) -> Self {
        Self {
            data: _ps.to_vec(),
            is_learnt: _learnt,
            activity: 0.0,
            id,
        }
    }
    fn size(&self) -> i32 {
        self.data.len() as i32
    }
    fn learnt(&self) -> bool {
        self.is_learnt
    }
}
