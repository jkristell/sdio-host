//! SD-specific command definitions.

use crate::common_cmd::{cmd, Cmd, R1, R3, Resp};

/// R6: Published RCA response
pub struct R6;
/// R7: Card interface condition
pub struct R7;

impl Resp for R6 {}
impl Resp for R7 {}

/// CMD3: Send RCA
pub fn send_relative_address() -> Cmd<R6> {
    cmd(3, 0)
}

/// CMD6: Switch Function Command
pub fn cmd6(arg: u32) -> Cmd<R1> {
    cmd(6, arg)
}

/// CMD8: Sends memory card interface conditions
pub fn send_if_cond(voltage: u8, checkpattern: u8) -> Cmd<R7> {
    let arg = u32::from(voltage & 0xF) << 8 | u32::from(checkpattern);
    cmd(8, arg)
}

/// CMD11: Switch to 1.8V bus signaling level
pub fn voltage_switch() -> Cmd<R1> {
    cmd(11, 0)
}

/// CMD19: Send tuning pattern
pub fn send_tuning_block(addr: u32) -> Cmd<R1> {
    cmd(19, addr)
}

/// CMD20: Speed class control
pub fn speed_class_control(arg: u32) -> Cmd<R1> {
    cmd(20, arg)
}

/// CMD22: Address extension
pub fn address_extension(arg: u32) -> Cmd<R1> {
    cmd(22, arg)
}

/// CMD23: Address extension
pub fn set_block_count(blockcount: u32) -> Cmd<R1> {
    cmd(23, blockcount)
}

/// ACMD6: Bus Width
/// * `bw4bit` - Enable 4 bit bus width
pub fn set_bus_width(bw4bit: bool) -> Cmd<R1> {
    let arg = if bw4bit { 0b10 } else { 0b00 };
    cmd(6, arg)
}

/// ACMD13: SD Status
pub fn sd_status() -> Cmd<R1> {
    cmd(13, 0)
}

/// ACMD41: App Op Command
///
/// * `high_capacity` - Host supports high capacity cards
/// * `xpc` - Controls the maximum power and default speed mode of SDXC and SDUC cards.
/// * `s18r` - Switch to 1.8V signaling
/// * `voltage_window` - The voltage window the host supports.
pub fn sd_send_op_cond(high_capacity: bool, xpc: bool, s18r: bool, voltage_window: u32) -> Cmd<R3> {
    let arg = u32::from(high_capacity) << 30
        | u32::from(xpc) << 28
        | u32::from(s18r) << 24
        | voltage_window & 0x00FF_FFFF;
    cmd(41, arg)
}

/// ACMD51: Reads the SCR
pub fn send_scr() -> Cmd<R1> {
    cmd(51, 0)
}
