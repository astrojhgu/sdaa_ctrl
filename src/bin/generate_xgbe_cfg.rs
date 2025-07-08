use std::{net::SocketAddrV4, fs::File};
use serde_yaml::{from_reader, to_writer};
use clap::Parser;
use sdaa_ctrl::ctrl_msg::CtrlMsg;

use pnet::datalink::{self, NetworkInterface};
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long = "dest", value_name = "<addr:port>")]
    dest_addr: String,

    #[clap(short = 's', long = "src", value_name = "<addr:port>")]
    src_addr: String,

    #[clap(short = 'p', long = "port", value_name = "{1,2,3,4}")]
    port_id: u32,

    #[clap(short = 'i', long = "in", value_name = "input name")]
    iname: String,

    #[clap(short = 'o', long = "out", value_name = "output name")]
    oname: String,
}

fn find_interface_by_ip(target_ip: IpAddr) -> Option<NetworkInterface> {
    for iface in datalink::interfaces() {
        for ip in iface.ips.iter().map(|p| p.ip()) {
            if ip == target_ip {
                return Some(iface);
            }
        }
    }
    None
}

pub fn main() {
    let args = Args::parse();


    //let socket_addr=SocketAddrV4::from(args.dest_addr);
    let dest_addr = args.dest_addr.parse::<SocketAddrV4>().unwrap();
    let src_addr = args.src_addr.parse::<SocketAddrV4>().unwrap();

    println!("dest addr: {dest_addr}");

    let cfg: Vec<CtrlMsg> = from_reader(File::open(&args.iname).expect("file not open")).expect("failed to load cfg");
    if let CtrlMsg::XGbeCfgSingle { msg_id, port_id, mut cfg }=cfg[0]{
        cfg.dst_ip.copy_from_slice(&dest_addr.ip().octets());
        cfg.dst_port=dest_addr.port();
        cfg.src_ip.copy_from_slice(&src_addr.ip().octets());
        cfg.src_port=src_addr.port();

        let iface=find_interface_by_ip(IpAddr::V4(dest_addr.ip().to_owned())).unwrap();
        let mac=iface.mac.unwrap();
        cfg.dst_mac.copy_from_slice(&mac.octets());
        println!("dest mac : {mac}");
        to_writer(File::create(&args.oname).expect("cannot create file"), 
        &CtrlMsg::XGbeCfgSingle{msg_id, port_id, cfg}
    ).expect("failed to write cfg")
    }else{
        panic!("Error the input file has to container only one XGbeCfg")
    }

    /*
    let cfg=CtrlMsg::XGbeCfgSingle{
        msg_id:0,
        port_id: args.port_id,
        cfg: XGbeCfg{

        }
    }*/
}
