use std::process::{Command, Output};

fn run_git(args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute git {}", args.join(" ")))
}

// Set rustc environment variable GIT_HASH_STATUS. This contains the short git sha id and, if
// the workspace is dirty, the string "-dirty" appended.
fn main() {
    let output = run_git(&["rev-parse", "--short", "HEAD"]);
    let short_hash = String::from_utf8(output.stdout).unwrap();

    let output = run_git(&["diff", "--quiet"]);
    let status = if output.status.code().unwrap_or(1) != 0 {
        "-dirty"
    } else {
        ""
    };

    let mut git_hash = String::new();
    git_hash.push_str(short_hash.trim());
    git_hash.push_str(status);

    println!("cargo:rustc-env=GIT_HASH_STATUS={}", git_hash);
}
