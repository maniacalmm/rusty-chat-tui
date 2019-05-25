use std::io::{self, Write, Read,stdout, stdin, Result, Stdout, Stdin};
fn main() {
    let mut stdout = stdout();
    let hello = "hello".as_bytes();
    stdout.write(hello).unwrap();
}