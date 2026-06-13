use trybuild::TestCases;

#[cfg(has_proc_macro_diagnostic)]
static PATH: &str = "./examples/nightly/examples/fail_*.rs";
#[cfg(not(has_proc_macro_diagnostic))]
static PATH: &str = "./examples/stable/examples/fail_*.rs";

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail(PATH);
}
