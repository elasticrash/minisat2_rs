# A MiniSAT solver in Rust based on MiniSAT 2

`mini_sat_rs` ships as both a command-line tool and a reusable library, so you can
solve CNF files from the shell or embed the solver in your own application. It is not
yet optimised for efficiency so use with care. 

## Build

```
cargo build --release
```

## Command-line usage

```
mini_sat_rs [OPTIONS] [input]
```

| Argument | Description | Default |
|---|---|---|
| `[input]` | CNF file in DIMACS format | `./default.cnf` |
| `-l, --log-level` | Log level: `error`, `warn`, `info`, `debug`, `trace` | `info` |

Examples:

```
mini_sat_rs                          # reads ./default.cnf
mini_sat_rs problem.cnf
mini_sat_rs problem.cnf --log-level trace
```

Output is written to stdout and `sat.log`.

## Library usage

Add `sat_rs` as a dependency:

```toml
[dependencies]
mini_sat_rs = "0.2.3"
```

### Solve a DIMACS CNF string

The simplest entry point parses a problem in DIMACS CNF format and solves it:

```rust
use mini_sat_rs::solve_dimacs;

let problem = "p cnf 3 2\n1 -3 0\n2 3 -1 0\n";
let state = solve_dimacs(problem);

if state.ok {
    println!("SATISFIABLE");
} else {
    println!("UNSATISFIABLE");
}
println!("{}", state.solver_stats); // solver statistics
```

### Build a problem programmatically

For finer-grained control, construct a `SolverState` and add clauses directly.
The `prelude` brings the core types and the extension traits you need into
scope in one import:

```rust
use mini_sat_rs::prelude::*;

let mut state = SolverState::new();

// Allocate variables 0..=2.
while 2 >= state.n_vars() {
    state.new_var();
}

// Add the clause (x0 OR NOT x2).
let mut clause = vec![Lit::simple(0), !Lit::simple(2)];
state.add_clause(&mut clause);

state.solve_no_assumptions();
assert!(state.ok);
```

Variables are zero-based: `Lit::simple(0)` is the positive literal of the first
variable, and `!Lit::simple(0)` is its negation. Inspect `state.ok` for
satisfiability and `state.solver_stats` for solver statistics.

## Tests

```
cargo test
```
