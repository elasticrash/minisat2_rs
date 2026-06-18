use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  propagate
|
|  Description:
|    Propagates all enqueued facts. If a conflict arises, the conflicting clause is returned,
|    otherwise null. NOTE! This method has been optimized for speed rather than readability.
|
|    Post-conditions:
|      * the propagation queue is empty, even if there was a conflict.
|________________________________________________________________________________________________@*/
pub trait Prop {
    fn propagate(&mut self) -> Option<ClauseRef>;
}

impl Prop for SolverState {
    fn propagate(&mut self) -> Option<ClauseRef> {
        trace!("{}|{}|{}", "propagate".to_string(), file!(), line!(),);

        let mut confl: Option<ClauseRef> = None;

        while self.qhead < self.trail.len() as i32 {
            self.solver_stats.propagations += 1.0;
            self.simp_db_props -= 1.0;

            let p: Lit = self.trail[self.qhead as usize];
            self.qhead += 1;
            let mut ws: Vec<ClauseRef> = std::mem::take(&mut self.watches[p.x as usize]);

            //log p
            let mut i: usize = 0;
            let mut j: usize = 0;
            let end: usize = i + ws.len();
            let false_lit: Lit = !p;
            while i != end {
                let cref: ClauseRef = ws[i];
                i += 1;

                {
                    let c = self.clause_mut(cref);
                    if c.data[0] == false_lit {
                        c.data[0] = c.data[1];
                        c.data[1] = false_lit;
                    }
                }

                assert!(self.clause(cref).data[1] == false_lit);

                let first: Lit = self.clause(cref).data[0];
                let val: Lbool = self.value_by_lit(first);
                if val == L_TRUE {
                    ws[j] = cref;
                    j += 1;
                } else {
                    let mut foundwatch: bool = false;
                    let len = self.clause(cref).data.len();
                    for k in 2..len {
                        if self.value_by_lit(self.clause(cref).data[k]) != L_FALSE {
                            let new_watch = {
                                let c = self.clause_mut(cref);
                                c.data[1] = c.data[k];
                                c.data[k] = false_lit;
                                (!c.data[1]).x as usize
                            };
                            self.watches[new_watch].push(cref);
                            foundwatch = true;
                            break;
                        }
                    }
                    if !foundwatch {
                        ws[j] = cref;
                        j += 1;
                        if !self.enqueue(&first, Some(cref)) {
                            if self.decision_level() == 0 {
                                self.ok = false;
                            }
                            confl = Some(cref);
                            self.qhead = self.trail.len() as i32;

                            while i < end {
                                ws[j] = ws[i];
                                j += 1;
                                i += 1;
                            }
                        }
                    }
                }
            }
            ws.truncate(ws.len() - (i - j));
            self.watches[p.x as usize] = ws;
        }
        confl
    }
}
