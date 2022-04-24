use std::io::Read;

use rand;
use rand::Rng;

fn random_number_vector() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut s = Vec::new();
    for _ in 0..4 {
        s.push(rng.gen_range(1..128));
    }
    s
}
fn random_string2() -> String {
    let str = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(7)
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&str).to_string()
}
fn random_string3() -> String {
    let str = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(7)
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&str).to_string()
}

#[test]
fn test_random_string() {
    println!("{:?}", random_number_vector());
    println!("{}", random_string2());
    println!("{}", random_string3());
}

#[test]
fn test_bad_bytes() {
    let bad_bytes = random_string2();
    assert_cmd::Command::cargo_bin("headr")
        .unwrap()
        .args(&["-c", &bad_bytes, "../tests/inputs/empty.txt"])
        .assert()
        .failure();
}
#[test]
fn test_bad_lines() {
    let bad_lines = random_string2();
    assert_cmd::Command::cargo_bin("headr")
        .unwrap()
        .args(&["-c", &bad_lines, "../tests/inputs/empty.txt"])
        .assert()
        .failure();
}

fn run_read_file(args: &[&str], expected_file: &str) {
    let mut file = std::fs::File::open(expected_file).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    assert_cmd::Command::cargo_bin("headr")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(buf);
}
#[test]
fn test_empty() {
    run_read_file(
        &["-n=2", "tests/inputs/empty.txt"],
        "tests/expected/empty.txt",
    );
}
#[test]
fn read_lines_from_files() {
    run_read_file(
        &["tests/inputs/three-lines.txt"],
        "tests/expected/three-lines.txt",
    );
    run_read_file(
        &["-n=2", "tests/inputs/three-lines.txt"],
        "tests/expected/three-lines-n2.txt",
    );
}
#[test]
fn read_lines() {
    run_read_file(
        &["-n=3", "tests/inputs/ten.txt"],
        "tests/expected/ten_n3.txt",
    );
    run_read_file(
        &["-n", "3", "tests/inputs/ten.txt"],
        "tests/expected/ten_n3.txt",
    );
    run_read_file(
        &["tests/inputs/ten.txt", "-n", "3"],
        "tests/expected/ten_n3.txt",
    );
    run_read_file(
        &["tests/inputs/ten.txt", "--lines", "3"],
        "tests/expected/ten_n3.txt",
    );
    // last n line
    run_read_file(
        &["-n", "-3", "tests/inputs/ten.txt"],
        "tests/expected/ten_n-3.txt",
    );
    run_read_file(
        &["-n=-3", "tests/inputs/ten.txt"],
        "tests/expected/ten_n-3.txt",
    );
    run_read_file(
        &["tests/inputs/ten.txt", "-n", "-3"],
        "tests/expected/ten_n-3.txt",
    );
    run_read_file(
        &["tests/inputs/ten.txt", "--lines", "-3"],
        "tests/expected/ten_n-3.txt",
    );
    run_read_file(
        &["tests/inputs/ten.txt", "--lines=-3"],
        "tests/expected/ten_n-3.txt",
    );
    run_read_file(
        &["--lines", "-3", "tests/inputs/ten.txt"],
        "tests/expected/ten_n-3.txt",
    );
}
#[test]
fn long_flag() {}
#[test]
fn test_bytes() {
    run_read_file(
        &["-c=3", "tests/inputs/unicodes.txt"],
        "tests/expected/unicodes-c3.txt",
    );
    run_read_file(
        &["-c", "3", "tests/inputs/unicodes.txt"],
        "tests/expected/unicodes-c3.txt",
    );
    run_read_file(
        &["-c=-3", "tests/inputs/unicodes.txt"],
        "tests/expected/unicodes_c-3.txt",
    );
    run_read_file(
        &["-c", "-3", "tests/inputs/unicodes.txt"],
        "tests/expected/unicodes_c-3.txt",
    );
}
