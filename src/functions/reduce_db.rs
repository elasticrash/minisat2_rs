use crate::functions::new_clause::*;
use crate::models::clause::*;
use crate::models::solverstate::*;
use std::cmp::Ordering;

/*_________________________________________________________________________________________________
|
|  reduceDB
|
|  Description:
|    Remove half of the learnt clauses, minus the clauses locked by the current assignment. Locked
|    clauses are clauses that are reason to some assignment. Binary clauses are never removed.
|________________________________________________________________________________________________@*/
pub trait Reduce {
    fn reduce_db(&mut self);
}

impl Reduce for SolverState {
    fn reduce_db(&mut self) {
        trace!("{}|{}|{}", "reduce_db".to_string(), file!(), line!());

        let extra_lim: f64 = self.cla_inc / self.learnts.len() as f64;

        let mut learnts = std::mem::take(&mut self.learnts);
        learnts.sort_by(|&x, &y| {
            let cx = self.clause(x);
            let cy = self.clause(y);
            if cx.size() > 2 && (cy.size() == 2 || cx.activity < cy.activity) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        self.learnts = learnts;

        let half = self.learnts.len() / 2;
        let mut j: usize = 0;
        for i in 0..self.learnts.len() {
            let cref = self.learnts[i];
            let removable = self.clause(cref).data.len() > 2
                && !self.locked(cref)
                && (i < half || self.clause(cref).activity < extra_lim);

            if removable {
                self.remove(cref, false);
            } else {
                self.learnts[j] = self.learnts[i];
                j += 1;
            }
        }
        self.learnts.truncate(j);
    }
}
