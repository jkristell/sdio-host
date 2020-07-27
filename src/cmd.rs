/// Host to Card commands
pub struct Cmd {
    pub cmd: u8,
    pub arg: u32,
    pub resp: Response,
}

/// Command Response type
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Response {
    None = 0,
    Short = 1,
    Long = 3,
}

/// Go to idle
pub const IDLE: (u8, Response)                  = (0, Response::None);
/// Ask all cards to send their Cids
pub const ALL_SEND_CID: (u8, Response)          = (2, Response::Long);
/// Send Rca
pub const SEND_REL_ADDR: (u8, Response)         = (3, Response::Short);
/// Select or deselect card
pub const SEL_DESEL_CARD: (u8, Response)        = (7, Response::Short);
///
pub const HS_SEND_EXT: (u8, Response)           = (8, Response::Short);
///
pub const HS_SEND_CSD: (u8, Response)           = (9, Response::Long);
///
pub const CMD13: (u8, Response)                 = (13, Response::Short);
///
pub const SET_BLOCKLEN: (u8, Response)          = (16, Response::Short);
///
pub const READ_SINGLE_BLOCK: (u8, Response)     = (17, Response::Short);
///
pub const WRITE_SINGLE_BLOCK: (u8, Response)    = (24, Response::Short);
///
pub const APP_OP_COMMAND: (u8, Response)        = (41, Response::Short);
///
pub const CMD51: (u8, Response)                 = (51, Response::Short);
///
pub const APP_CMD: (u8, Response)               = (55, Response::Short);

///
/// App Commands
///

///
pub const ACMD6: (u8, Response)                 = (6, Response::Short);
///
pub const ACMD13: (u8, Response)                = (13, Response::Short);

impl Cmd {
    const fn from_constant((cmd, resp): (u8, Response), arg: u32) -> Cmd {
        Cmd {cmd, arg, resp}
    }

    pub const fn idle() -> Cmd {
        Cmd::from_constant(IDLE, 0)
    }

    pub const fn all_send_cid() -> Cmd {
        Cmd::from_constant(ALL_SEND_CID, 0)
    }

    pub const fn send_rel_address() -> Cmd {
        Cmd::from_constant(SEND_REL_ADDR, 0)
    }

    pub const fn cmd51() -> Cmd {
        Cmd::from_constant(CMD51, 0)
    }

    pub const fn acmd6(arg: u32) -> Cmd {
        Cmd::from_constant(ACMD6, arg)
    }

    pub const fn acmd13() -> Cmd {
        Cmd::from_constant(ACMD13, 0)
    }

    pub const fn sel_desel_card(arg: u32) -> Cmd {
        Cmd::from_constant(SEL_DESEL_CARD, arg)
    }

    pub const fn hs_send_ext_csd(arg: u32) -> Cmd {
        Cmd::from_constant(HS_SEND_EXT, arg)
    }

    pub const fn send_csd(arg: u32) -> Cmd {
        Cmd::from_constant(HS_SEND_CSD, arg)
    }

    pub const fn cmd13(arg: u32) -> Cmd {
        Cmd::from_constant(CMD13, arg)
    }

    pub const fn set_blocklen(blocklen: u32) -> Cmd {
        Cmd::from_constant(SET_BLOCKLEN, blocklen)
    }

    pub const fn read_single_block(addr: u32) -> Cmd {
        Cmd::from_constant(READ_SINGLE_BLOCK, addr)
    }

    pub const fn write_single_block(addr: u32) -> Cmd {
        Cmd::from_constant(WRITE_SINGLE_BLOCK, addr)
    }

    pub const fn app_op_cmd(arg: u32) -> Cmd {
        Cmd::from_constant(APP_OP_COMMAND, arg)
    }

    /// App Command. Indicates that next command will be a app command
    pub const fn app_cmd(rca: u32) -> Cmd {
        Cmd::from_constant(APP_CMD, rca)
    }
}
