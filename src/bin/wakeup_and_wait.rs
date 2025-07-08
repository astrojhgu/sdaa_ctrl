use clap::Parser;
use sdaa_ctrl::ctrl_msg::{send_cmd, CtrlMsg};
use std::time::Duration;

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

    #[clap(short = 't', value_name = "timeout in sec", default_value = "1")]
    timeout: u64,

    #[clap(
        short = 'd',
        long = "debug",
        value_name = "debug level",
        default_value("0")
    )]
    debug_level: u32,
}

fn main() {
    let args = Args::parse();
    let debug_level = args.debug_level;

    let cmd = CtrlMsg::PwrCtrl {
        msg_id: 0,
        op_code: 1,
    };
    let mut summary = send_cmd(
        cmd,
        &args.addr,
        &args.local_addr,
        Some(Duration::from_secs(args.timeout)),
        debug_level,
    );
    while !summary.no_reply.is_empty() {
        let addr = summary
            .no_reply
            .iter()
            .map(|(a, _)| a[0])
            .collect::<Vec<_>>();
        let cmd = CtrlMsg::PwrCtrl {
            msg_id: 0,
            op_code: 1,
        };
        eprintln!("addrs {addr:?} not reply, retrying");
        summary = send_cmd(
            cmd,
            &addr,
            &args.local_addr,
            Some(Duration::from_secs(args.timeout)),
            debug_level,
        );
    }
    eprintln!("all have replied");
    std::thread::sleep(Duration::from_secs(5));

    let cmd = CtrlMsg::Query { msg_id: 0 };
    let mut summary = send_cmd(
        cmd,
        &args.addr,
        &args.local_addr,
        Some(Duration::from_secs(args.timeout)),
        debug_level,
    );
    if summary.normal_reply.len() != args.addr.len() {
        println!("some one abnormal, please check");
        println!("{summary:?}");
        std::process::exit(1);
    }
    loop {
        let addr = summary
            .normal_reply
            .iter()
            .filter(|&(_a, m)| {
                if let CtrlMsg::QueryReply {
                    msg_id: _,
                    fm_ver: _,
                    tick_cnt1: _,
                    tick_cnt2: _,
                    trans_state: _,
                    locked,
                    health: _,
                } = m
                {
                    !(*locked == 0x3f || *locked == 0x2f)
                } else {
                    false
                }
            })
            .map(|&(a, _)| a)
            .collect::<Vec<_>>();

        if addr.is_empty() {
            break;
        }
        std::thread::sleep(Duration::from_secs(1));
        let cmd = CtrlMsg::Query { msg_id: 0 };
        summary = send_cmd(
            cmd,
            &addr,
            &args.local_addr,
            Some(Duration::from_secs(args.timeout)),
            debug_level,
        );
    }

    let cmd = CtrlMsg::Query { msg_id: 0 };
    let _ = send_cmd(
        cmd,
        &args.addr,
        &args.local_addr,
        Some(Duration::from_secs(args.timeout)),
        debug_level,
    );
}
