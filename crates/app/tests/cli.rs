//! `pulse --help` prints the usage, and an unknown flag is an error.

use std::process::Command;

#[test]
fn pulse_help_prints_the_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_pulse"))
        .arg("--help")
        .output()
        .expect("pulse --help");
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("Uso: pulse"), "{stdout}");
    assert!(stdout.contains("--render"), "{stdout}");
    assert!(stdout.contains("--version"), "{stdout}");
}

#[test]
fn pulse_short_help_prints_the_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_pulse"))
        .arg("-h")
        .output()
        .expect("pulse -h");
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("Uso: pulse"), "{stdout}");
}

#[test]
fn pulse_rejects_an_unknown_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_pulse"))
        .arg("--no-such-flag")
        .output()
        .expect("pulse --no-such-flag");
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--no-such-flag"), "{stderr}");
    assert!(stderr.contains("Uso: pulse"), "{stderr}");
}
