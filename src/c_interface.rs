use std::{
    ffi::{c_char, CStr},
    net::SocketAddr,
    slice::from_raw_parts_mut,
    time::Duration,
};

use crate::ctrl_msg::{bcast_cmd, send_cmd, CtrlMsg};

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[no_mangle]
pub unsafe extern "C" fn find_device(
    addr: *const c_char,
    result: *mut u32,
    max_n: usize,
    local_port: u16,
) -> usize {
    let buf = from_raw_parts_mut(result, max_n);
    let c_str = CStr::from_ptr(addr);
    let addr = if let Ok(s) = c_str.to_str() {
        format!("{}:3000", s)
    } else {
        return 0;
    };

    let query = CtrlMsg::Query { msg_id: 0 };

    let summary = bcast_cmd(
        query,
        addr,
        format!("0.0.0.0:{}", local_port),
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
            buf[nresult - 1] = r;
        }
    }
    nresult
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[no_mangle]
pub unsafe extern "C" fn make_device(addr: *const c_char, local_port: u16) -> bool {
    let c_str = CStr::from_ptr(addr);
    let addr = vec![if let Ok(s) = c_str.to_str() {
        format!("{}:3000", s)
    } else {
        return false;
    }];

    let local_addr = format!("0.0.0.0:{}", local_port);

    let cmd = CtrlMsg::Init {
        msg_id: 0,
        reserved_zeros: 0,
    };
    let summary = send_cmd(cmd, &addr, &local_addr, Some(Duration::from_secs(5)), 1);

    println!("{:?}", summary);

    // if summary.normal_reply.len() != 1 {
    //     return false;
    // }
    let cmd = CtrlMsg::Sync { msg_id: 0 };
    let _summary = send_cmd(cmd, &addr, local_addr, Some(Duration::from_secs(5)), 1);

    // if summary.normal_reply.len() != 1 {
    //     return false;
    // }

    true
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[no_mangle]
pub unsafe extern "C" fn unmake_device(addr: *const c_char, local_port: u16) -> bool {
    let c_str = CStr::from_ptr(addr);
    let addr = vec![if let Ok(s) = c_str.to_str() {
        format!("{}:3000", s)
    } else {
        return false;
    }];

    let local_addr = format!("0.0.0.0:{}", local_port);

    let cmd = CtrlMsg::StreamStop { msg_id: 0 };

    let summary = send_cmd(cmd, &addr, &local_addr, Some(Duration::from_secs(5)), 1);
    if summary.normal_reply.len() != 1 {
        return false;
    }

    true
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[no_mangle]
pub unsafe extern "C" fn start_stream(addr: *const c_char, local_port: u16) -> bool {
    let c_str = CStr::from_ptr(addr);
    let addr = vec![if let Ok(s) = c_str.to_str() {
        format!("{}:3000", s)
    } else {
        return false;
    }];

    let local_addr = format!("0.0.0.0:{}", local_port);

    let cmd = CtrlMsg::StreamStart { msg_id: 0 };

    let summary = send_cmd(cmd, &addr, &local_addr, Some(Duration::from_secs(5)), 1);
    if summary.normal_reply.len() != 1 {
        return false;
    }

    true
}
