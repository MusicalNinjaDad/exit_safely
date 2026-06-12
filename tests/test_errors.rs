#![cfg(has_proc_macro_diagnostic)]
use trybuild::TestCases;

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail("tests/fixtures/try/examples/fail_*.rs");
}
