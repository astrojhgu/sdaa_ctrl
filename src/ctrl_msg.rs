use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    io::Cursor,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use binrw::{binrw, BinRead, BinWrite};
use chrono::Local;
use serde::{Deserialize, Serialize};

use rand::{rng, Rng};

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
        temperatures: [u32; 2],
    },

    #[brw(magic(0x78_56_34_12_u32))]
    TEHealth {
        nhealth: u32,
        #[br(count=nhealth)]
        payload: Vec<u32>,
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
    I2CScanReply {
        msg_id: u32,
        ndev: u32,
        #[br(count=ndev)]
        payload: Vec<u8>,
    },
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
    PwrCtrl { msg_id: u32, op_code: u32 },
    #[brw(magic(0xff_00_00_07_u32))]
    PwrCtrlReply { msg_id: u32 },

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
            CtrlMsg::I2CScanReply {
                msg_id,
                ndev,
                payload,
            } => {
                write!(f, "I2CScanReply{{msg_id: {msg_id}, ndev: {ndev}, payload: ")?;
                for &x in payload {
                    write!(f, "{x:02x} ")?;
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
                write!(
                    f,
                    "I2CReadReply{{ msg_id: {msg_id}, err_code: {err_code:x}, payload:"
                )?;
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
                len,
                payload,
            } => {
                write!(
                    f,
                    "I2CReadRegReply{{ msg_id: {msg_id}, len:{len}, err_code: {err_code:x} payload:"
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
            CtrlMsg::PwrCtrl { msg_id, op_code } => {
                writeln!(f, "PwrCtrl{{msg_id: {msg_id}, op_code: {op_code}}}")
            }
            CtrlMsg::PwrCtrlReply { msg_id } => {
                writeln!(f, "PwrCtrlReply{{msg_id: {msg_id}}}")
            }

            CtrlMsg::Init {
                msg_id,
                reserved_zeros: _,
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
            PwrCtrl { msg_id, .. } => *msg_id = mid,
            PwrCtrlReply { msg_id, .. } => *msg_id = mid,
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
            PwrCtrl { msg_id, .. } => *msg_id,
            PwrCtrlReply { msg_id } => *msg_id,
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

#[derive(Default, Debug)]
pub struct CmdReplySummary {
    pub no_reply: Vec<(Vec<SocketAddr>, u32)>,
    pub invalid_reply: Vec<(SocketAddr, CtrlMsg)>,
    pub normal_reply: Vec<(SocketAddr, CtrlMsg)>,
}

pub fn send_cmd<A, B>(
    mut cmd: CtrlMsg,
    targets: &[A],
    local_addr: B,
    timeout: Option<Duration>,
    debug_level: u32,
) -> CmdReplySummary
where
    A: ToSocketAddrs,
    B: ToSocketAddrs,
{
    let socket = UdpSocket::bind(local_addr).unwrap();
    socket.set_broadcast(true).expect("broadcast set failed");
    socket
        .set_nonblocking(true)
        .expect("nonblocking set failed");

    socket
        .set_read_timeout(timeout)
        .expect("failed to set timeout");

    let mut rng1 = rng();
    let mut msg_set = BTreeSet::new();
    let mut addr_msg_id_map = BTreeMap::<u32, Vec<SocketAddr>>::new();
    let mut reply_summary = CmdReplySummary::default();
    for (_i, addr) in targets.iter().enumerate() {
        let msg_id: u32 = rng1.random();
        cmd.set_msg_id(msg_id);
        msg_set.insert(msg_id);
        addr_msg_id_map.insert(msg_id, addr.to_socket_addrs().unwrap().collect::<Vec<_>>());
        let mut buf = Cursor::new(Vec::new());
        cmd.write(&mut buf).unwrap();
        let buf = buf.into_inner();
        socket.send_to(&buf, addr).expect("send error");

        println!(
            "{} msg with id={} sent",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            msg_id,
        );
        print_bytes(&buf);

        println!("{:?}", cmd);

        let mut buf = vec![0_u8; 9000];
        while let Ok((l, a)) = socket.recv_from(&mut buf) {
            //let (_s, _a)=socket.recv_from(&mut buf).unwrap();
            if debug_level >= 1 {
                println!(
                    "{} received {} bytes, {} words from {:?}:",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    l,
                    l / 4,
                    a
                );
                print_bytes(&buf[..l]);
            }
            let buf1 = std::mem::replace(&mut buf, vec![0_u8; 9000]);
            let mut cursor = Cursor::new(buf1);
            let reply = CtrlMsg::read(&mut cursor).unwrap();

            let msg_id = reply.get_msg_id();
            if let CtrlMsg::InvalidMsg { .. } = reply {
                println!(
                    "{} Invalid msg {:?}",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    reply
                );
                reply_summary.invalid_reply.push((a, reply));
            } else {
                reply_summary.normal_reply.push((a, reply));
            }

            println!(
                "{} msg with id={} replied from {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                msg_id,
                a
            );
            let x = msg_set.remove(&msg_id);
            assert!(x);
        }
    }

    println!("==waiting for the rest replies==");
    socket
        .set_nonblocking(false)
        .expect("nonblocking set failed");

    let mut buf = vec![0_u8; 9000];

    if !msg_set.is_empty() {
        while let Ok((l, a)) = socket.recv_from(&mut buf) {
            if debug_level >= 1 {
                println!(
                    "{} received {} bytes, {} words from {:?}:",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    l,
                    l / 4,
                    a
                );
                print_bytes(&buf[..l]);
            }

            let mut cursor = Cursor::new(buf.clone());
            let reply = CtrlMsg::read(&mut cursor).unwrap();
            println!(
                "{} \n{}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                reply
            );

            let msg_id = reply.get_msg_id();

            if let CtrlMsg::InvalidMsg { .. } = reply {
                println!("Invalid msg received");
                reply_summary.invalid_reply.push((a, reply));
            } else {
                reply_summary.normal_reply.push((a, reply));
            }

            println!(
                "{} msg with id={} replied from {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                msg_id,
                a
            );
            let x = msg_set.remove(&msg_id);
            assert!(x);
            if msg_set.is_empty() {
                break;
            }
        }
    }
    //reply_summary.no_reply = msg_set.into_iter().map(|i| i as usize).collect();
    reply_summary.no_reply = addr_msg_id_map.iter().filter(|&(k,v)|{
        msg_set.contains(k)
    }).map(|(&k,v)| (v.clone(), k)).collect();
    reply_summary
}

pub fn bcast_cmd<A, B>(
    mut cmd: CtrlMsg,
    baddr: A,
    local_addr: B,
    timeout: Option<Duration>,
    debug_level: u32,
) -> CmdReplySummary
where
    A: ToSocketAddrs,
    B: ToSocketAddrs,
{
    let socket = UdpSocket::bind(local_addr).unwrap();
    socket.set_broadcast(true).expect("broadcast set failed");
    socket
        .set_nonblocking(true)
        .expect("nonblocking set failed");

    socket
        .set_read_timeout(timeout)
        .expect("failed to set timeout");

    let mut reply_summary = CmdReplySummary::default();

    cmd.set_msg_id(0 as u32);

    let mut buf = Cursor::new(Vec::new());
    cmd.write(&mut buf).unwrap();
    let buf = buf.into_inner();
    socket.send_to(&buf, baddr).expect("send error");

    println!(
        "{} msg with id={} sent",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        0,
    );
    print_bytes(&buf);

    println!("{:?}", cmd);

    let mut buf = vec![0_u8; 9000];
    while let Ok((l, a)) = socket.recv_from(&mut buf) {
        //let (_s, _a)=socket.recv_from(&mut buf).unwrap();
        if debug_level >= 1 {
            println!(
                "{} received {} bytes, {} words from {:?}:",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                l,
                l / 4,
                a
            );
            print_bytes(&buf[..l]);
        }
        let buf1 = std::mem::replace(&mut buf, vec![0_u8; 9000]);
        let mut cursor = Cursor::new(buf1);
        let reply = CtrlMsg::read(&mut cursor).unwrap();

        let msg_id = reply.get_msg_id();
        if let CtrlMsg::InvalidMsg { .. } = reply {
            println!(
                "{} Invalid msg {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                reply
            );
            reply_summary.invalid_reply.push((a, reply));
        } else {
            reply_summary.normal_reply.push((a, reply));
        }

        println!(
            "{} msg with id={} replied from {:?}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            msg_id,
            a
        );
    }

    println!("==waiting for the rest replies==");
    socket
        .set_nonblocking(false)
        .expect("nonblocking set failed");

    let mut buf = vec![0_u8; 9000];

    while let Ok((l, a)) = socket.recv_from(&mut buf) {
        if debug_level >= 1 {
            println!(
                "{} received {} bytes, {} words from {:?}:",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                l,
                l / 4,
                a
            );
            print_bytes(&buf[..l]);
        }

        let mut cursor = Cursor::new(buf.clone());
        let reply = CtrlMsg::read(&mut cursor).unwrap();
        println!(
            "{} \n{}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            reply
        );

        let msg_id = reply.get_msg_id();

        if let CtrlMsg::InvalidMsg { .. } = reply {
            println!("Invalid msg received");
            reply_summary.invalid_reply.push((a, reply));
        } else {
            reply_summary.normal_reply.push((a, reply));
        }

        println!(
            "{} msg with id={} replied from {:?}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            msg_id,
            a
        );
    }
    reply_summary
}
