#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # Get the ouput from `echo Hello, world!`
//!
//! [std::process::Command]
//!
//! ```
//! use std::process::Command;
//! let output = Command::new("echo")
//!        .arg("Hello,")
//!        .arg("world!")
//!        .output()
//!        .expect("Failed to execute command");
//! let output = String::from_utf8(output.stdout).expect("Format error");
//! assert_eq!(format!("{}", output), "Hello, world!\n");
//!
//! ```
use std::process::Command;

fn echo_hello_world() -> String {
    let output = Command::new("echo")
        .arg("Hello,")
        .arg("world!")
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).expect("Format error")
}

fn main() {
    println!("{}", echo_hello_world());
}

#[cfg(test)]
#[test]
fn test_echo_hello_world() {
    assert_eq!(format!("{}", echo_hello_world()), "Hello, world!\n");
}
