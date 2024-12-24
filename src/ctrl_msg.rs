use std::fmt::Display;

use binrw::binrw;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[binrw]
#[brw(little)]
pub struct XGbeCfg {
    #[brw(pad_after(2))]
    pub dst_mac: [u8; 6],
    #[brw(pad_after(2))]
    pub src_mac: [u8; 6],

    pub dst_ip: [u8; 4], //20
    pub src_ip: [u8; 4], //24

    #[brw(pad_after(2))]
    pub dst_port: u16, //26
    #[brw(pad_after(2))]
    pub src_port: u16, //30
}

impl Display for XGbeCfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}:{}",
            self.src_ip[0], self.src_ip[1], self.src_ip[2], self.src_ip[3], self.src_port
        )?;
        write!(f, "(")?;
        for x in self.src_mac {
            write!(f, " {x:02x}")?
        }
        write!(f, ") -> ")?;
        write!(
            f,
            "{}.{}.{}.{}:{}",
            self.dst_ip[0], self.dst_ip[1], self.dst_ip[2], self.dst_ip[3], self.dst_port
        )?;
        write!(f, "(")?;
        for x in self.dst_mac {
            write!(f, " {x:02x}")?
        }
        write!(f, ")")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[binrw]
#[brw(little)]
pub enum Health {
    #[brw(magic(0x31_76_6c_68_u32))]
    HLHealth {
        nhealth: u32,
        xgbe_state: [u32; 4],
        pkt_sent: [u64; 4],
        volt12_inner: u32,
        volt12_input: u32,
        vcc1v0: u32,
        vcc1v8: u32,
        mgtavtt1v2: u32,
        mgtavtt1v0: u32,
        temperature: u32,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[binrw]
#[brw(little)]
pub enum CtrlMsg {
    #[brw(magic(0xff_ff_ff_ff_u32))]
    InvalidMsg {
        msg_id: u32,
        err_code: u32,
        len: u32,
        #[br(count=len)]
        description: Vec<u8>,
    },
    #[brw(magic(0x01_u32))]
    Query { msg_id: u32 },
    #[brw(magic(0xff_00_00_01_u32))]
    QueryReply {
        msg_id: u32,
        fm_ver: u32,
        tick_cnt1: u32,
        tick_cnt2: u32,
        trans_state: u32,
        locked: u32,
        health: Health,
    },
    #[brw(magic(0x02_u32))]
    Sync { msg_id: u32 },
    #[brw(magic(0xff_00_00_02_u32))]
    SyncReply { msg_id: u32 },
    #[brw(magic(0x03_u32))]
    XGbeCfg { msg_id: u32, cfg: [XGbeCfg; 4] },
    #[brw(magic(0xff_00_00_03_u32))]
    XgbeCfgReply { msg_id: u32 },
    #[brw(magic(0x04_u32))]
    I2CScan { msg_id: u32 },
    #[brw(magic(0xff_00_00_04_u32))]
    I2CScanReply { msg_id: u32, payload: [u8; 32] },
    #[brw(magic(0x01_04_u32))]
    I2CWrite {
        msg_id: u32,
        dev_addr: u32,
        len: u32,
        #[br(count = len)]
        payload: Vec<u8>,
    },
    #[brw(magic(0xff_00_01_04_u32))]
    I2CWriteReply { msg_id: u32, err_code: u32 },
    #[brw(magic(0x02_04_u32))]
    I2CWriteReg {
        msg_id: u32,
        dev_addr: u32,
        reg_addr: u32,
        len: u32,
        #[br(count=len)]
        payload: Vec<u8>,
    },
    #[brw(magic(0xff_00_02_04_u32))]
    I2CWriteRegReply { msg_id: u32, err_code: u32 },
    #[brw(magic(0x03_04_u32))]
    I2CRead {
        msg_id: u32,
        dev_addr: u32,
        nbytes: u32,
    },
    #[brw(magic(0xff_00_03_04_u32))]
    I2CReadReply {
        msg_id: u32,
        err_code: u32,
        len: u32,
        #[br(count=len)]
        payload: Vec<u8>,
    },
    #[brw(magic(0x04_04_u32))]
    I2CReadReg {
        msg_id: u32,
        dev_addr: u32,
        reg_addr: u32,
        nbytes: u32,
    },
    #[brw(magic(0xff_00_04_04_u32))]
    I2CReadRegReply {
        msg_id: u32,
        err_code: u32,
        len: u32,
        #[br(count=len)]
        payload: Vec<u8>,
    },
    #[brw(magic(0x01_05_u32))]
    StreamStart { msg_id: u32 },
    #[brw(magic(0xff_00_01_05_u32))]
    StreamStartReply { msg_id: u32 },
    #[brw(magic(0x02_05_u32))]
    StreamStop { msg_id: u32 },
    #[brw(magic(0xff_00_02_05_u32))]
    StreamStopReply { msg_id: u32 },
    #[brw(magic(0x06_u32))]
    VGACtrl {
        msg_id: u32,
        nvga: u32,
        #[br(count=nvga)]
        gains: Vec<u32>,
    },
    #[brw(magic(0xff_00_00_06_u32))]
    VGACtrlReply { msg_id: u32, err_code: u32 },
    #[brw(magic(0x07_u32))]
    Suspend { msg_id: u32, reserved_zeros: u32 },
    #[brw(magic(0xff_00_00_07_u32))]
    SuspendReply { msg_id: u32 },

    #[brw(magic(0x08_u32))]
    Init { msg_id: u32, reserved_zeros: u32 },
    #[brw(magic(0xff_00_00_08_u32))]
    InitReply { msg_id: u32 },
}

impl Display for CtrlMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=====================")?;
        match self {
            CtrlMsg::InvalidMsg {
                msg_id,
                err_code,
                len: _,
                description,
            } => {
                let desc = String::from_utf8(description.clone()).unwrap();
                writeln!(
                    f,
                    "InvalidMsg:{{ msg_id: {msg_id}, err_code: {err_code}, desc: {desc} }}"
                )
            }
            CtrlMsg::Query { msg_id } => {
                writeln!(f, "Query{{ msg_id: {msg_id} }}")
            }
            CtrlMsg::QueryReply {
                msg_id,
                fm_ver,
                tick_cnt1,
                tick_cnt2,
                trans_state,
                locked,
                health,
            } => {
                write!(f, "QueryReply{{msg_id: {msg_id}, fm_ver: 0x{fm_ver:x}, tick_cnt1: {tick_cnt1}, tick_cnt2: {tick_cnt2}, trans_state: 0x{trans_state:x}, locked: 0x{locked:x}, Health: {health:?}")?;
                writeln!(f, "}}")
            }
            CtrlMsg::Sync { msg_id } => {
                writeln!(f, "Sync {{msg_id: {msg_id}}}")
            }
            CtrlMsg::SyncReply { msg_id } => {
                writeln!(f, "SyncReply{{msg_id: {msg_id}}}")
            }
            CtrlMsg::XGbeCfg { msg_id, cfg } => {
                writeln!(f, "XGbeCfg{{msg_id: {msg_id}")?;
                for x in cfg {
                    writeln!(f, "{}", x)?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::XgbeCfgReply { msg_id } => {
                writeln!(f, "XgbeCfgReply{{msg_id: {msg_id}}}")
            }
            CtrlMsg::I2CScan { msg_id } => {
                writeln!(f, "I2CScan{{msg_id: {msg_id}}}")
            }
            CtrlMsg::I2CScanReply { msg_id, payload } => {
                write!(f, "I2CScanReply{{msg_id: {msg_id}")?;
                for &x in payload {
                    write!(f, "{x:02x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::I2CWrite {
                msg_id,
                dev_addr,
                len: _,
                payload,
            } => {
                write!(f, "I2CWrite{{ msg_id: {msg_id}, dev_addr: 0x{dev_addr:x}, ")?;
                for &x in payload {
                    write!(f, " {x:02x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::I2CWriteReply { msg_id, err_code } => {
                writeln!(
                    f,
                    "I2CWriteReply{{msg_id: {msg_id}, err_code: 0x{err_code:x}}}"
                )
            }
            CtrlMsg::I2CWriteReg {
                msg_id,
                dev_addr,
                reg_addr,
                len: _,
                payload,
            } => {
                write!(f, "I2CWriteReg{{ msg_id: {msg_id}, dev_addr: 0x{dev_addr:x}, reg_addr: {reg_addr:x}")?;
                for &x in payload {
                    write!(f, " {x:02x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::I2CWriteRegReply { msg_id, err_code } => {
                writeln!(
                    f,
                    "I2CWriteRegReply{{msg_id: {msg_id}, err_code: 0x{err_code:x}}}"
                )
            }
            CtrlMsg::I2CRead {
                msg_id,
                dev_addr,
                nbytes,
            } => {
                writeln!(
                    f,
                    "I2CRead{{msg_id: {msg_id}, dev_addr: 0x{dev_addr:x}, nbytes:{nbytes}}}"
                )
            }
            CtrlMsg::I2CReadReply {
                msg_id,
                err_code,
                len: _,
                payload,
            } => {
                write!(f, "I2CReadReply{{ msg_id: {msg_id}, err_code: {err_code:x}")?;
                for &x in payload {
                    write!(f, " {x:02x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::I2CReadReg {
                msg_id,
                dev_addr,
                reg_addr,
                nbytes,
            } => {
                writeln!(f, "I2CReadReg{{msg_id: {msg_id}, dev_addr: 0x{dev_addr:x}, reg_addr: {reg_addr:x} nbytes:{nbytes}}}")
            }
            CtrlMsg::I2CReadRegReply {
                msg_id,
                err_code,
                len: _,
                payload,
            } => {
                write!(
                    f,
                    "I2CReadRegReply{{ msg_id: {msg_id}, err_code: {err_code:x}"
                )?;
                for &x in payload {
                    write!(f, " {x:02x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::StreamStart { msg_id } => {
                writeln!(f, "StreamStart{{msg_id: {msg_id}}}")
            }
            CtrlMsg::StreamStartReply { msg_id } => {
                writeln!(f, "StreamStartReply{{msg_id: {msg_id}}}")
            }
            CtrlMsg::StreamStop { msg_id } => {
                writeln!(f, "StreamStop{{msg_id: {msg_id}}}")
            }
            CtrlMsg::StreamStopReply { msg_id } => {
                writeln!(f, "StreamStopReply{{msg_id: {msg_id}}}")
            }
            CtrlMsg::VGACtrl {
                msg_id,
                nvga: _,
                gains,
            } => {
                write!(f, "VGACtrl{{ msg_id: {msg_id},")?;
                for &x in gains {
                    write!(f, "{x}")?;
                }
                writeln!(f, "}}")
            }
            CtrlMsg::VGACtrlReply { msg_id, err_code } => {
                writeln!(
                    f,
                    "VGACtrlReply{{msg_id: {msg_id}, err_code: 0x{err_code:x}}}"
                )
            }
            CtrlMsg::Suspend {
                msg_id,
                reserved_zeros: _,
            } => {
                writeln!(f, "Suspend{{msg_id: {msg_id}}}")
            }
            CtrlMsg::SuspendReply { msg_id } => {
                writeln!(f, "SuspendReply{{msg_id: {msg_id}}}")
            }

            CtrlMsg::Init {
                msg_id,
                reserved_zeros:_,
            } => {
                writeln!(f, "Init {{msg_id: {msg_id}}}")
            }

            CtrlMsg::InitReply { msg_id } => {
                writeln!(f, "InitReply {{msg_id: {msg_id}}}")
            }
        }?;
        writeln!(f, "=====================")
    }
}

impl CtrlMsg {
    pub fn set_msg_id(&mut self, mid: u32) {
        use CtrlMsg::*;
        match self {
            InvalidMsg { msg_id, .. } => *msg_id = mid,
            Query { msg_id } => *msg_id = mid,
            QueryReply { msg_id, .. } => *msg_id = mid,
            Sync { msg_id } => *msg_id = mid,
            SyncReply { msg_id } => *msg_id = mid,
            XGbeCfg { msg_id, .. } => *msg_id = mid,
            XgbeCfgReply { msg_id } => *msg_id = mid,
            I2CScan { msg_id } => *msg_id = mid,
            I2CScanReply { msg_id, .. } => *msg_id = mid,
            I2CWrite { msg_id, .. } => *msg_id = mid,
            I2CWriteReply { msg_id, .. } => *msg_id = mid,
            I2CWriteReg { msg_id, .. } => *msg_id = mid,
            I2CWriteRegReply { msg_id, .. } => *msg_id = mid,
            I2CRead { msg_id, .. } => *msg_id = mid,
            I2CReadReply { msg_id, .. } => *msg_id = mid,
            I2CReadReg { msg_id, .. } => *msg_id = mid,
            I2CReadRegReply { msg_id, .. } => *msg_id = mid,
            StreamStart { msg_id } => *msg_id = mid,
            StreamStartReply { msg_id } => *msg_id = mid,
            StreamStop { msg_id } => *msg_id = mid,
            StreamStopReply { msg_id } => *msg_id = mid,
            VGACtrl { msg_id, .. } => *msg_id = mid,
            VGACtrlReply { msg_id, .. } => *msg_id = mid,
            Suspend { msg_id, .. } => *msg_id = mid,
            SuspendReply { msg_id, .. } => *msg_id = mid,
            Init { msg_id, .. } => *msg_id = mid,
            InitReply { msg_id, .. } => *msg_id = mid,
        }
    }

    pub fn get_msg_id(&self) -> u32 {
        use CtrlMsg::*;
        match self {
            InvalidMsg { msg_id, .. } => *msg_id,
            Query { msg_id } => *msg_id,
            QueryReply { msg_id, .. } => *msg_id,
            Sync { msg_id } => *msg_id,
            SyncReply { msg_id } => *msg_id,
            XGbeCfg { msg_id, .. } => *msg_id,
            XgbeCfgReply { msg_id } => *msg_id,
            I2CScan { msg_id } => *msg_id,
            I2CScanReply { msg_id, .. } => *msg_id,
            I2CWrite { msg_id, .. } => *msg_id,
            I2CWriteReply { msg_id, .. } => *msg_id,
            I2CWriteReg { msg_id, .. } => *msg_id,
            I2CWriteRegReply { msg_id, .. } => *msg_id,
            I2CRead { msg_id, .. } => *msg_id,
            I2CReadReply { msg_id, .. } => *msg_id,
            I2CReadReg { msg_id, .. } => *msg_id,
            I2CReadRegReply { msg_id, .. } => *msg_id,
            StreamStart { msg_id } => *msg_id,
            StreamStartReply { msg_id } => *msg_id,
            StreamStop { msg_id } => *msg_id,
            StreamStopReply { msg_id } => *msg_id,
            VGACtrl { msg_id, .. } => *msg_id,
            VGACtrlReply { msg_id, .. } => *msg_id,
            Suspend { msg_id, .. } => *msg_id,
            SuspendReply { msg_id } => *msg_id,
            Init {
                msg_id,
                reserved_zeros: _,
            } => *msg_id,
            InitReply { msg_id } => *msg_id,
        }
    }
}

pub fn print_bytes(x: &[u8]) {
    for (i, w) in x.chunks(4).enumerate() {
        for &b in w {
            print!("{b:02x} ");
        }
        print!("| {i:02} {}:{}", i * 4, i * 4 + 3);
        println!();
    }
}
