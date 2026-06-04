use ninja_build_rs::{Result, nightly::Nightly};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("let_chains");
    ac.emit_unstable_feature("if_let_guard");
    Ok(())
}
