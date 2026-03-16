#![feature(assert_matches)]
#![feature(never_type)]
#![feature(try_trait_v2)]

use std::{assert_matches::assert_matches, path::PathBuf, process::Command};

use escargot::CargoBuild;
use exit_safely::Termination;
use try_v2::Try;

use std::sync::LazyLock;
static APP: LazyLock<PathBuf> = LazyLock::new( ||
    CargoBuild::new()
        .bin("fixture_app")
        .manifest_path("./tests/fixture_app/Cargo.toml")
        .current_release()
        .run()
        .unwrap()
        .path()
        .to_owned(),
);

#[test]
fn exit_ok() {
    let mut cmd = Command::new(APP.as_os_str());
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!("Hello, world!\n", stdout);
    assert_eq!("", stderr);
    assert_eq!(0, output.status.code().unwrap());
}
