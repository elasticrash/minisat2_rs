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

        self.learnts.sort_by(|x, y| {
            if x.size() > 2 && (y.size() == 2 || x.activity < y.activity) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let half = self.learnts.len() / 2;
        let mut j: usize = 0;
        for i in 0..self.learnts.len() {
            let removable = self.learnts[i].data.len() > 2
                && !self.locked(&self.learnts[i].clone())
                && (i < half || self.learnts[i].activity < extra_lim);

            if removable {
                self.remove(self.learnts[i].clone(), false);
            } else {
                self.learnts[j] = self.learnts[i].clone();
                j += 1;
            }
        }
        self.learnts.truncate(j);
    }
}
