#![cfg(has_proc_macro_diagnostic)]
use trybuild::TestCases;

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail("examples/stable/examples/fail_*.rs");
}
