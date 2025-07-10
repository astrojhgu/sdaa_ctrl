use std::{fs::File, net::SocketAddrV4};
use serde_yaml::{from_reader, to_writer};
use clap::Parser;
use sdaa_ctrl::ctrl_msg::CtrlMsg;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long = "dest", value_name = "<addr:port>")]
    dest_addr: Option<String>,

    #[clap(short = 's', long = "src", value_name = "<addr:port>")]
    src_addr: Option<String>,

    #[clap(short = 'p', long = "port", value_name = "{1,2,3,4}")]
    port_id: Option<u32>,

    #[clap(short = 'i', long = "in", value_name = "input name")]
    iname: String,

    #[clap(short = 'o', long = "out", value_name = "output name")]
    oname: String,
}


fn multicast_mac_from_socketaddr(addr: &SocketAddrV4) -> Option<[u8; 6]> {
    let ip = addr.ip().octets();

    // 检查是否是合法的组播地址（224.0.0.0 ~ 239.255.255.255）
    if ip[0] < 224 || ip[0] > 239 {
        return None;
    }

    // 取 IP 地址的低 23 位（保留顺序）
    let ip_u32 = u32::from(*addr.ip());
    let low_23 = ip_u32 & 0x7FFFFF;

    let mac = [
        0x01,
        0x00,
        0x5e,
        ((low_23 >> 16) & 0x7F) as u8, // 只取低 7 位
        ((low_23 >> 8) & 0xFF) as u8,
        (low_23 & 0xFF) as u8,
    ];

    Some(mac)
}

pub fn main() {
    let args = Args::parse();


    //let socket_addr=SocketAddrV4::from(args.dest_addr);
    let dest_addr = args.dest_addr.map(|s| s.parse::<SocketAddrV4>().unwrap());
    let src_addr = args.src_addr.map(|s| s.parse::<SocketAddrV4>().unwrap());

    println!("dest addr: {dest_addr:?}");

    let cfg: Vec<CtrlMsg> = from_reader(File::open(&args.iname).expect("file not open")).expect("failed to load cfg");
    if let CtrlMsg::XGbeCfgSingle { msg_id, port_id, mut cfg }=cfg[0]{
                if let Some(addr)=dest_addr{
            cfg.dst_ip.copy_from_slice(&addr.ip().octets());
            cfg.dst_port=addr.port();
            let mac=multicast_mac_from_socketaddr(&addr).unwrap();
            println!("dest mac : {mac:?}");
            cfg.dst_mac.copy_from_slice(&mac);
        }
        if let Some(addr)=src_addr{
            cfg.src_ip.copy_from_slice(&addr.ip().octets());
            cfg.src_port=addr.port();
        }
        let port_id=args.port_id.unwrap_or(port_id);
        to_writer(File::create(&args.oname).expect("cannot create file"), 
        &vec![CtrlMsg::XGbeCfgSingle{msg_id, port_id, cfg}]
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
