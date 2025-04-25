use clap::Parser;
use sdaa_ctrl::ctrl_msg::{send_cmd, CtrlMsg};
use serde_yaml::from_reader;
use std::{fs::File, time::Duration};

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

    let cmds: Vec<CtrlMsg> = from_reader(File::open(&args.cmd).expect("file not open")).expect("failed to load cmd");
    for c in cmds {
        let summary = send_cmd(
            c,
            &args.addr,
            &args.local_addr,
            Some(Duration::from_secs(args.timeout)),
            debug_level,
        );
        if summary.no_reply.is_empty() {
            println!("all replied");
        } else {
            println!("not replied:");
            for (addr, msg_id) in &summary.no_reply {
                println!("{:?} {}", addr, msg_id);
            }
        }

        if !summary.invalid_reply.is_empty() {
            println!("Invalid reply:");
            for (a, r) in summary.invalid_reply {
                println!("{} {}", a, r);
            }
        }
    }
}
