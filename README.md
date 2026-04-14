# Rust Port of MiniSAT 2

## Build

```
cargo build --release
```

## Usage

```
sat_rs [OPTIONS] [input]
```

| Argument | Description | Default |
|---|---|---|
| `[input]` | CNF file in DIMACS format | `./input.txt` |
| `-l, --log-level` | Log level: `error`, `warn`, `info`, `debug`, `trace` | `info` |

Examples:

```
sat_rs                          # reads ./input.txt
sat_rs problem.cnf
sat_rs problem.cnf --log-level trace
```
Output is written to stdout and `sat.log`.

## Tests

```
cargo test
```
