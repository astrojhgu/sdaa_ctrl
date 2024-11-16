use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'a', long = "addr", num_args(1..), value_name="ip:port")]
    addr: String,
}

use binrw::{BinRead, BinWrite};
use sdand_ctrl::ctrl_msg::{CtrlMsg, CtrlMsg::*};
use std::{io::Cursor, net::UdpSocket};
fn main() {
    let args = Args::parse();
    let socket = UdpSocket::bind(args.addr).unwrap();
    socket.set_nonblocking(false).unwrap();
    loop {
        let mut buf = vec![0_u8; 9000];
        let (sz, addr) = socket.recv_from(&mut buf).unwrap();
        println!("received {} Bytes from {}", sz, addr);
        let mut cursor = Cursor::new(buf);
        let msg = CtrlMsg::read(&mut cursor).unwrap();

        let reply = match msg {
            Query { msg_id } => QueryReply {
                msg_id,
                fm_ver: 1,
                tick_cnt1: 10,
                tick_cnt2: 10,
                locked: 0,
                nhealth: 10,
                values: vec![0; 10],
            },
            //QueryReply { msg_id } => *msg_id = mid,
            Sync { msg_id } => SyncReply { msg_id },
            //SyncReply { msg_id } => *msg_id = mid,
            XGbeCfg { msg_id, .. } => XgbeCfgReply { msg_id },
            //XgbeCfgReply { msg_id } => *msg_id = mid,
            I2CScan { msg_id } => I2CScanReply {
                msg_id,
                payload: [0_u8; 32],
            },
            //I2CScanReply { msg_id, .. } => *msg_id = mid,
            I2CWrite { msg_id, .. } => I2CWriteReply {
                msg_id,
                err_code: 0,
            },
            //I2CWriteReply { msg_id, .. } => *msg_id = mid,
            I2CWriteReg { msg_id, .. } => I2CWriteRegReply {
                msg_id,
                err_code: 0,
            },
            //I2CWriteRegReply { msg_id, .. } => *msg_id = mid,
            I2CRead { msg_id, .. } => I2CReadReply {
                msg_id,
                err_code: 0,
                len: 10,
                payload: vec![0; 10],
            },
            //I2CReadReply { msg_id, .. } => *msg_id = mid,
            I2CReadReg { msg_id, .. } => I2CReadRegReply {
                msg_id,
                err_code: 0,
                len: 10,
                payload: vec![0; 10],
            },
            //I2CReadRegReply { msg_id, .. } => *msg_id = mid,
            StreamStart { msg_id } => StreamStartReply { msg_id },
            //StreamStartReply { msg_id } => *msg_id = mid,
            StreamStop { msg_id } => StreamStopReply { msg_id },
            //StreamStopReply { msg_id } => *msg_id = mid,
            _ => panic!(),
        };

        let mut cursor = Cursor::new(Vec::new());
        reply.write(&mut cursor).unwrap();
        let buf = cursor.into_inner();
        socket.send_to(&buf, addr).unwrap();
    }
}
