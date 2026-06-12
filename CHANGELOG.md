# exit_safely changelog

## [v0.3.0] - Works on stable

### Technical changes

- Removed usage of nightly feature `iterator_try_collect`
- Bumped MSRV to 1.95.0 to support `if_let_guard` and `let_chains`
- Made `proc_macro2_diagnostic` work on stable, updated dependency here
- Updated docs, tests & examples to cover stable
