use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    slice::from_raw_parts_mut,
    time::Duration,
};

use crate::ctrl_msg::{bcast_cmd, send_cmd, CtrlMsg};


/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_device(
    addr: u32,
    result: *mut u32,
    max_n: usize,
    local_port: u16,
) -> usize {
    let result = unsafe{from_raw_parts_mut(result, max_n)};
    let ip = Ipv4Addr::from(addr);

    let addr = SocketAddrV4::new(ip, 3000);

    let query = CtrlMsg::Query { msg_id: 0 };

    let summary = bcast_cmd(
        query,
        addr,
        format!("0.0.0.0:{local_port}"),
        Some(Duration::from_secs(1)),
        1,
    );

    let mut nresult = 0;
    for (a, _r) in summary.normal_reply {
        if let SocketAddr::V4(x) = a {
            let ip = x.ip();
            let mut r: u32 = 0;
            for (i, &o) in ip.octets().iter().enumerate() {
                //(i as u32)
                r += (o as u32) << (8 * (3 - i));
            }

            nresult += 1;
            if nresult >= max_n {
                break;
            }
            result[nresult - 1] = r;
        }
    }
    nresult
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_device(ip: u32, local_port: u16) -> bool {
    let ip = Ipv4Addr::from(ip);
    let addr = SocketAddrV4::new(ip, 3000);

    let local_addr = format!("0.0.0.0:{local_port}");

    let cmd = CtrlMsg::Init {
        msg_id: 0,
        reserved_zeros: 0,
    };
    let summary = send_cmd(cmd, &[addr], &local_addr, Some(Duration::from_secs(5)), 1);

    println!("{summary:?}");

    // if summary.normal_reply.len() != 1 {
    //     return false;
    // }
    let cmd = CtrlMsg::Sync { msg_id: 0 };
    let _summary = send_cmd(cmd, &[addr], local_addr, Some(Duration::from_secs(5)), 1);

    // if summary.normal_reply.len() != 1 {
    //     return false;
    // }

    true
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unmake_device(ip: u32, local_port: u16) -> bool {
    let ip = Ipv4Addr::from(ip);
    let addr = SocketAddrV4::new(ip, 3000);

    let local_addr = format!("0.0.0.0:{local_port}");

    let cmd = CtrlMsg::StreamStop { msg_id: 0 };

    let summary = send_cmd(cmd, &[addr], &local_addr, Some(Duration::from_secs(5)), 1);
    if summary.normal_reply.len() != 1 {
        return false;
    }

    true
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_stream(ip: u32, local_port: u16) -> bool {
    let ip = Ipv4Addr::from(ip);
    let addr = SocketAddrV4::new(ip, 3000);

    let local_addr = format!("0.0.0.0:{local_port}");

    let cmd = CtrlMsg::StreamStart { msg_id: 0 };

    let summary = send_cmd(cmd, &[addr], &local_addr, Some(Duration::from_secs(5)), 1);
    if summary.normal_reply.len() != 1 {
        return false;
    }

    true
}
