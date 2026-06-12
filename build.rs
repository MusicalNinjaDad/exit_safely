use ninja_build_rs::{Result, nightly::Nightly};

fn main() -> Result<()> {
    let mut ac = autocfg::new();
    ac.set_edition(Some("2024".to_string()));

    ac.emit_unstable_feature("try_trait_v2");

    Ok(())
}
