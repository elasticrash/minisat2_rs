use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  simplifyDB
|
|  Description:
|    Simplify the clause database according to the current top-level assigment. Currently, the only
|    thing done here is the removal of satisfied clauses, but more things can be put here.
|________________________________________________________________________________________________@*/
pub trait Simplify {
    fn simplify_db(&mut self);
}

impl Simplify for SolverState {
    fn simplify_db(&mut self) {
        trace!("{}|{}|{}", "simplify_db".to_string(), file!(), line!());
        if !self.ok {
            return;
        }

        assert!(self.decision_level() == 0);
        match self.propagate() {
            None => {
                trace!(
                    "{}|{}|{}",
                    "propagate match none".to_string(),
                    file!(),
                    line!()
                );
                if self.clone().n_assigns() == self.simp_db_assigns as usize
                    || self.simp_db_props > 0.0
                {
                    return;
                }

                for y in self.simp_db_assigns..self.clone().n_assigns() as i32 {
                    let _p: Lit = self.trail[y as usize];
                    self.watches[_p.x as usize].clear();
                    self.watches[(!_p).x as usize].clear();
                }

                // Compact the clause/learnt index lists in place: keep the
                // surviving refs at the front and truncate. `j <= k` always, so
                // the entry read at index `k` is never one we have overwritten.
                for t in 0..2 {
                    let clause_size: usize = if t != 0 {
                        self.learnts.len()
                    } else {
                        self.clauses.len()
                    };

                    let mut j: usize = 0;
                    for k in 0..clause_size {
                        let cref = if t != 0 {
                            self.learnts[k]
                        } else {
                            self.clauses[k]
                        };
                        let a = !self.locked(cref);
                        let b = self.simplify(k as i32, t);

                        if a && b {
                            self.remove(cref, false);
                        } else {
                            if t != 0 {
                                self.learnts[j] = cref;
                            } else {
                                self.clauses[j] = cref;
                            }
                            j += 1;
                        }
                    }
                    if t != 0 {
                        self.learnts.truncate(j);
                    } else {
                        self.clauses.truncate(j);
                    }
                }

                self.simp_db_assigns = self.clone().n_assigns() as i32;
                self.simp_db_props =
                    self.solver_stats.clauses_literals + self.solver_stats.learnts_literals;
            }
            _ => {
                trace!(
                    "{}|{}|{}",
                    "solver state false".to_string(),
                    file!(),
                    line!(),
                );
                self.ok = false;
            }
        }
    }
}
