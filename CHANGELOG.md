# exit_safely changelog

## [v0.3.2] - Stable examples

### Bugfix

- Fixed broken compiler errors on stable. Required not joining spans, even on nightly.

### Technical changes

- Moved test fixture apps to `examples` directory
- Added `trybuild` tests for stable channel

## [v0.3.1] - Immutable releases

## Bugfix

- Create immutable release on github via workflow

## [v0.3.0] - Works on stable

### Technical changes

- Removed usage of nightly feature `iterator_try_collect`
- Bumped MSRV to 1.95.0 to support `if_let_guard` and `let_chains`
- Made `proc_macro2_diagnostic` work on stable, updated dependency here
- Updated docs, tests & examples to cover stable
