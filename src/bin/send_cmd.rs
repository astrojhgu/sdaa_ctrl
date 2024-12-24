use std::{collections::BTreeSet, fs::File, io::Cursor, net::UdpSocket};

use binrw::{BinRead, BinWrite};
use clap::Parser;
use sdand_ctrl::ctrl_msg::{print_bytes, CtrlMsg};
use serde_yaml::from_reader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'a', long = "addr", num_args(1..), value_name = "<ip:port> ...")]
    addr: Vec<String>,

    #[clap(
        short = 'L',
        long = "local addr",
        value_name = "local addr and port, default: [::]:3001",
        default_value("[::]:3001")
    )]
    local_addr: String,

    #[clap(short = 'c', long = "cmd", value_name = "cmd.yaml")]
    cmd: String,
}

fn main() {
    let args = Args::parse();

    let cmds: Vec<CtrlMsg> = from_reader(File::open(&args.cmd).expect("file not open")).unwrap();

    let socket = UdpSocket::bind(args.local_addr).unwrap();
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

            let mut buf = Cursor::new(Vec::new());
            c.write(&mut buf).unwrap();
            let buf = buf.into_inner();
            socket.send_to(&buf, addr).expect("send error");

            println!("msg with id={} sent", msg_id);
            print_bytes(&buf);

            println!("{:?}", c);

            let mut buf = vec![0_u8; 9000];
            while let Ok((l, a)) = socket.recv_from(&mut buf) {
                //let (_s, _a)=socket.recv_from(&mut buf).unwrap();
                println!("received bytes from {:?}:", a);
                print_bytes(&buf[..l]);
                let buf1 = std::mem::replace(&mut buf, vec![0_u8; 9000]);
                let mut cursor = Cursor::new(buf1);
                let reply = CtrlMsg::read(&mut cursor).unwrap();
                if let CtrlMsg::InvalidMsg { .. } = reply {
                    println!("Invalid msg {:?}", reply);
                }
                let msg_id = reply.get_msg_id();
                println!("msg with id={} replied", msg_id);
                assert!(msg_set.remove(&msg_id));
            }
            msg_id += 1;
        }
    }

    println!("==waiting for the rest replies==");
    socket
        .set_nonblocking(false)
        .expect("nonblocking set failed");

    while !msg_set.is_empty() {
        let mut buf = vec![0_u8; 9000];
        let (l, a) = socket.recv_from(&mut buf).unwrap();
        println!("received bytes from {:?}:", a);
        print_bytes(&buf[..l]);
        let mut cursor = Cursor::new(buf);
        let reply = CtrlMsg::read(&mut cursor).unwrap();
        println!("{}", reply);
        if let CtrlMsg::InvalidMsg { .. } = reply {
            println!("Invalid msg received");
        }
        let msg_id = reply.get_msg_id();
        println!("msg with id={} replied", msg_id);
        assert!(msg_set.remove(&msg_id));
    }
    println!("==all replies have been received. Bye!==");
}
