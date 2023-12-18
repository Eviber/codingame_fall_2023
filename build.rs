/// run `python3 bundler.py -i src/main.rs -o output.rs`
fn main() {
    std::process::Command::new("python3")
        .arg("bundler.py")
        .arg("-i")
        .arg("src/main.rs")
        .arg("-o")
        .arg("output.rs")
        .status()
        .expect("running bundler.py should succeed");
}
