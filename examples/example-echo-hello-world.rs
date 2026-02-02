#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # Get the ouput from `echo Hello, world!`
//!
//! ```
//! use std::process::Command;
//! let output = Command::new("echo")
//!        .arg("Hello,")
//!        .arg("world!")
//!        .output()
//!        .expect("Failed to execute command");
//! let output_1 = String::from_utf8_lossy(&output.stdout).to_string();
//! assert_eq!(format!("{}", output_1), "Hello, world!\n");
//! let output_2 = String::from_utf8(output.stdout).expect("Format error");
//! assert_eq!(format!("{}", output_2), "Hello, world!\n");
//!
//! ```
//! [std::process::Command]
//!
//! [String::from_utf8]
//!
//! [String::from_utf8_lossy]

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
