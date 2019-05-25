extern crate termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{self, Write, Read,stdout, stdin, Result, Stdout, Stdin};
use std::string::*;
use termion::event::Key;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use std::ops::Add;

fn check_unique_username(stream: &mut TcpStream, username: &[u8]) -> bool {
    let mut ans = vec![0];
    // let mut stream = stream.lock().unwrap();
    stream.write(username).unwrap();
    stream.read_exact(&mut ans);

    if ans[0] == 0 {
        false
    } else {
        true
    }
}

fn receive_message(stream: TcpStream, mut stdout: Stdout) {
    let mut stream = stream;
    thread::spawn(move || {
        let mut buf: [u8; 30] = [0; 30];
        loop {
            if let Ok(len) = stream.read(&mut buf) {
                if len > 0 {
                    let (line, _ ) = buf.split_at(len);
                    let incoming_msg = String::from_utf8_lossy(&line);
                    stdout.write(&incoming_msg.as_bytes()).unwrap();
                }
            }
        }
    });
}

fn main() {
    let mut stdin = stdin();
    // let mut stdout = Arc::new(Mutex::new(stdout()));
    let stdout = stdout();
    let mut output_stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    let mut incoming_stream = output_stream.try_clone().unwrap();

    println!("welcome to chat, please input your username..");
    let mut username = String::new();
    loop {
        if let Ok(size) = stdin.read_line(&mut username) {
            if (size-1) < 3 {
                println!("username should be more than three letters: {}.", size);
            } else {
                let username_buf = (&username[..username.len() - 1]).as_bytes();
                if check_unique_username(&mut output_stream, username_buf) {
                    break;
                } else {
                    println!("username is already taken.")
                }
            }
            username.clear();
        }
    }
    println!("You've joined the chat room :)");

    // aftering joining the chat room, spawn a new thread checking the broadcasting info
    // from other people
    // let stdout_copy = Arc::clone()
    receive_message(incoming_stream, stdout);

    let mut input = String::new();
    loop {
        // read_line read the line, including newline  symbol
        // on linux maybe \n -> 1
        // on windows maybe \r\n -> 2
        if let Ok(size) = stdin.read_line(&mut input) {
            if size - 1 < 0 {

            } else {
                // read everything, then append a new terminal symbol
                // for the server side to determine
                // but it needs to be some unique escape symbol
                // which I don't know how to do right now
                let content_to_send = (&input[..input.len()]).as_bytes();
                output_stream.write(content_to_send).unwrap();
                output_stream.flush().unwrap();
            }
        }
        input.clear();
    }

}
