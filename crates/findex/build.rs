use std::process::Command;

fn main() {
    let git_commit = match Command::new("git")
        .args(["describe", "--tags"])
        .output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::from("[git was unavailable in build time]")
            }
        },
        Err(_) => String::from("[git was unavailable in build time]"),
    };

    println!("cargo:rustc-env=GIT_COMMIT={git_commit}");
}