extern crate termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{self, Write, Read,stdout, stdin, Result};
use std::string::*;
use termion::event::Key;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn check_unique_username(stream: &mut TcpStream, username: &[u8]) -> bool {
    let mut ans = vec![0];
    stream.write(username).unwrap();
    stream.read_exact(&mut ans);

    if ans[0] == 0 {
        false
    } else {
        true
    }
}

fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    println!("welcome to chat, please input your username..");
    let mut username = String::new();
    loop {
        if let Ok(size) = stdin.read_line(&mut username) {
            if (size-1) < 3 {
                println!("username should be more than three letters: {}.", size);
            } else {
                let username_buf = (&username[..username.len() - 1]).as_bytes();
                if check_unique_username(&mut stream, username_buf) {
                    break;
                } else {
                    println!("username is already taken.")
                }
            }
            
            username.clear();
        }
    }

    // username and client is registered
    // the following is the actual tui

}
