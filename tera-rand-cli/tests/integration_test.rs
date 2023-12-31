use assert_cmd::output::OutputError;
use assert_cmd::Command;
use regex::Regex;
use std::process::Output;
use tracing::trace;
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_simple_output_with_record_limit() {
    let mut cmd: Command = Command::cargo_bin("tera-rand-cli").unwrap();
    cmd.args(&["-f", "resources/test/cpu_util.json", "--record-limit", "1"]);

    let output: Output = cmd.unwrap();
    let stdout: String = String::from_utf8(output.stdout).unwrap();
    trace!(stdout);

    let expected_regex: Regex =
        Regex::new(r#"\{"hostname": "[\w\d]{8}", "cpu_util": \d+}"#).unwrap();
    assert!(expected_regex.is_match(stdout.as_str()));
}

#[test]
#[traced_test]
fn test_error_when_file_not_passed_in() {
    let mut cmd: Command = Command::cargo_bin("tera-rand-cli").unwrap();

    let output_error: OutputError = cmd.unwrap_err();
    let output: &Output = output_error.as_output().unwrap();
    let stderr: String = String::from_utf8(output.stderr.clone()).unwrap();
    trace!(stderr);

    assert!(stderr.contains("the following required arguments were not provided:\n  --file <FILE>"));
}

#[test]
#[traced_test]
fn test_error_when_file_does_not_exist() {
    let mut cmd: Command = Command::cargo_bin("tera-rand-cli").unwrap();
    cmd.args(&["-f", "this-file-does-not-exist.json"]);

    let output_error: OutputError = cmd.unwrap_err();
    let output: &Output = output_error.as_output().unwrap();
    let stderr: String = String::from_utf8(output.stderr.clone()).unwrap();
    trace!(stderr);

    assert!(stderr.contains("Couldn't open template '\"this-file-does-not-exist.json\""));
}
