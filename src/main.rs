
extern crate colored;
extern crate libc;
extern crate byteorder;

mod terminal;
mod msg;

use msg::*;
use terminal::*;

use std::io::*;
use std::str;
use std::net::*;
use std::os::unix::io::AsRawFd;
use colored::*;

fn rpoll(fds: &mut [libc::pollfd], timeout: libc::c_int) -> libc::c_int {
    unsafe {
        libc::poll(&mut fds[0] as *mut libc::pollfd, fds.len() as libc::nfds_t, timeout)
    }
}

fn display_input(term: &mut Terminal, name: &str, inp: &str)
{
    term.add_bytes(name.as_bytes());
    term.add_bytes(b": ");
    term.add_bytes(inp.as_bytes());
}

fn main() {
    let mut stream: TcpStream;
    let mut name: String = String::new();

    print!("Name: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut name).unwrap();
    name.pop();

    let mut term = Terminal::init().unwrap();

    match TcpStream::connect("127.0.0.1:8123") {
        Ok(s) => stream = s,
        Err(e) => {
            panic!("TcpStream: {}", e);
        }
    };

    chat_send_name(&mut stream, &name);

    let mut fds: [libc::pollfd; 2] = [
        libc::pollfd { fd: libc::STDIN_FILENO, events: libc::POLLIN, revents: 0},
        libc::pollfd { fd: stream.as_raw_fd(), events: libc::POLLIN, revents: 0},
    ];

    let mut inp: String = String::new();

    display_input(&mut term, &name, &inp);

    loop {
        rpoll(&mut fds, -1);

        if fds[0].revents & libc::POLLIN != 0 {
            let mut buffer = [0;1];

            stdin().read_exact(&mut buffer).unwrap();

            match buffer[0] {
                b'\n' => {
                    chat_send_msg(&mut stream, &Msg { text: inp.clone(), flags: 0 });
                    term.del_bytes(inp.len());
                    inp.clear();
                },
                127 => {
                    if inp.len() > 0 {
                        inp.pop();
                        term.del_char();
                    }
                },
                c => {
                    inp.push(c as char);
                    term.add_char(c);
                }
            }
        }

        if fds[1].revents & libc::POLLIN != 0 {
            let msg: Msg = chat_recv_msg(&mut stream);

            term.erase_line();

            if msg.flags & 0x04 != 0 || msg.flags & 0x08 != 0 {
                println!("{}", msg.text.green().bold());
            } else if msg.flags & 0x02 != 0 {
                println!("{}", msg.text.red());
            } else {
                println!("{}", msg.text);
            }

            display_input(&mut term, &name, &inp);
        }
    }
}

