use cargo_test_support::prelude::*;
use cargo_test_support::{file, registry::RegistryBuilder};

#[cargo_test]
fn case() {
    let _ = RegistryBuilder::new()
        .alternative()
        .no_configure_token()
        .build();
    cargo_test_support::registry::Package::new("my-package", "99999.0.0-alpha.1+my-package")
        .alternative(true)
        .publish();

    snapbox::cmd::Command::cargo_ui()
        .arg("info")
        .arg("https://crates.io")
        .arg("--registry=alternative")
        .assert()
        .failure()
        .stdout_eq("")
        .stderr_eq(file!["stderr.term.svg"]);
}