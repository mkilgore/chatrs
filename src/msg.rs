
use std::net::*;
use std::io::*;
use std::str;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Msg {
    pub text: String,
    pub flags: u32,
}

pub fn chat_send_name(stream: &mut TcpStream, name: &str) {
    stream.write_u32::<LittleEndian>(name.len() as u32).unwrap();
    stream.write_all(name.as_bytes()).unwrap();
}

pub fn chat_send_msg(stream: &mut TcpStream, msg: &Msg) {
    if msg.text.len() > 0 {
        stream.write_u32::<LittleEndian>(msg.flags).unwrap();
        stream.write_u32::<LittleEndian>(msg.text.len() as u32).unwrap();
        stream.write(msg.text.as_bytes()).unwrap();
    }
}

pub fn chat_recv_msg(stream: &mut TcpStream) -> Msg {
    let mut msg: Msg = Msg { text: String::new(), flags: 0 };

    let flags: u32  = stream.read_u32::<LittleEndian>().unwrap();
    let length: u32 = stream.read_u32::<LittleEndian>().unwrap();

    let mut v: Vec<u8> = vec![0 as u8; length as usize];
    stream.read_exact(&mut v).unwrap();

    msg.flags = flags;
    msg.text = str::from_utf8(&mut v).unwrap().to_string();

    msg
}

