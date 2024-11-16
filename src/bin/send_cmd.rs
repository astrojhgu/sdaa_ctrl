use std::{collections::BTreeSet, fs::File, io::Cursor, net::UdpSocket};

use binrw::{BinRead, BinWrite};
use clap::Parser;
use sdand_ctrl::ctrl_msg::CtrlMsg;
use serde_yaml::from_reader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'a', long = "addr", num_args(1..), value_name = "<ip:port> ...")]
    addr: Vec<String>,

    #[clap(short = 'p', long = "port", value_name = "local port")]
    port: u16,

    #[clap(short = 'c', long = "cmd", value_name = "cmd.yaml")]
    cmd: String,
}

fn main() {
    let args = Args::parse();

    let cmds: Vec<CtrlMsg> = from_reader(File::open(&args.cmd).expect("file not open")).unwrap();

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", args.port)).unwrap();
    socket.set_broadcast(true).expect("broadcast set failed");
    socket
        .set_nonblocking(true)
        .expect("nonblocking set failed");

    let mut msg_id = 1;
    let mut msg_set = BTreeSet::new();
    for mut c in cmds {
        for addr in &args.addr {
            c.set_msg_id(msg_id);
            msg_set.insert(msg_id);
            println!("{} sent", msg_id);
            msg_id += 1;
            let mut buf = Cursor::new(Vec::new());
            c.write(&mut buf).unwrap();
            let buf = buf.into_inner();
            socket.send_to(&buf, addr).expect("send error");

            let mut buf = vec![0_u8; 9000];
            while let Ok((_s, _a)) = socket.recv_from(&mut buf) {
                //let (_s, _a)=socket.recv_from(&mut buf).unwrap();
                let buf1 = std::mem::replace(&mut buf, vec![0_u8; 9000]);
                let mut cursor = Cursor::new(buf1);
                let reply = CtrlMsg::read(&mut cursor).unwrap();
                let msg_id = reply.get_msg_id();
                println!("{} received", msg_id);
                assert!(msg_set.remove(&msg_id));
            }
        }
    }
    socket
        .set_nonblocking(false)
        .expect("nonblocking set failed");

    while !msg_set.is_empty() {
        let mut buf = vec![0_u8; 9000];
        socket.recv_from(&mut buf).unwrap();
        let mut cursor = Cursor::new(buf);
        let reply = CtrlMsg::read(&mut cursor).unwrap();
        let msg_id = reply.get_msg_id();
        println!("{} replied", msg_id);
        assert!(msg_set.remove(&msg_id));
    }
}
