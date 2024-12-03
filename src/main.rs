use std::fs::File;

use binrw::BinWrite;
use sdand_ctrl::ctrl_msg::{CtrlMsg, XGbeCfg};
use serde_yaml::to_writer;
fn main() {
    let xgbecfg = XGbeCfg {
        dst_mac: [0x00, 0x00, 0x00, 0x00, 0x00, 0xee],
        src_mac: [0x00, 0x00, 0x00, 0x00, 0x00, 0xff],
        dst_ip: [192, 168, 1, 100],
        src_ip: [192, 168, 1, 101],
        dst_port: 3000,
        src_port: 3001,
    };

    let msg = CtrlMsg::XGbeCfg {
        msg_id: 0x11223344,
        cfg: [xgbecfg, xgbecfg, xgbecfg, xgbecfg],
    };

    let mut f = File::create("a.bin").unwrap();
    msg.write(&mut f).unwrap();

    let msg1 = vec![msg];
    to_writer(File::create("cmd.yaml").unwrap(), &msg1).unwrap();
}
