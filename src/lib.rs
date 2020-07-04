#![no_std]

use bitfield::bitfield;

#[derive(Debug, Copy, Clone)]
pub enum CardCapacity {
    /// Standard Capacity (< 2Gb)
    SDSC,
    /// High capacity (< 32Gb)
    SDHC,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SdSpecVersion {
    /// Version 1.0 and and 1.0.1
    V1_0,
    /// Version 1.10
    V1_10,
    /// Version 2.0
    V2,
    /// Version 3.0
    V3,
    /// Version 4.0
    V4,
    Unsupported,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Card power status
    pub struct Ocr(u32);
    impl Debug;
    /// Voltage range 2.7 - 2.8 supported
    pub v27_28, _: 15;
    /// Voltage range 2.8 - 2.9 supported
    pub v28_29, _: 16;
    /// Voltage range 2.9 - 3.0 supported
    pub v29_30, _: 17;
    /// Voltage range 3.0 - 3.1 supported
    pub v30_31, _: 18;
    /// Voltage range 3.1 - 3.2 supported
    pub v31_32, _: 19;
    /// Voltage range 3.2 - 3.3 supported
    pub v32_33, _: 20;
    /// Voltage range 3.3 - 3.4 supported
    pub v33_34, _: 21;
    /// Voltage range 3.4 - 3.5 supported
    pub v34_35, _: 22;
    /// Voltage range 3.5 - 3.6 supported
    pub v35_36, _: 23;
    /// Switching to 1.8V Accepted (Only UHS-I card supports this bit)
    pub v18_allowed, _: 24;
    /// Over 2TB support Status (Only SDUC card supports this bit)
    pub over_2tb, _: 27;
    pub uhs2_card_status, _: 29;
    /// Card capacity, valid after power up
    /// True if SDHC or SDXC card is found, false for SDSC
    pub capacity, _: 30;
    /// Set to true when card has finished the power up routine
    pub powered, _: 31;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Card identification
    pub struct Cid([u32]);
    impl Debug;
    pub u8, crc7, _: 7, 1;
    pub u16, date, _: 19, 8;
    pub u32, serial, _: 55, 24;
    pub u64, name, _: 103, 64;
    pub u16, oid, _: 119, 104;
    pub u8, mid, _: 127, 120;
}

impl Cid<[u32; 4]> {
    pub fn name_bytes(&self) -> [u8; 5] {
        let bytes = self.name().to_be_bytes();
        let mut ret = [0u8; 5];
        ret.copy_from_slice(&bytes[3..]);
        ret
    }

    pub fn oem(&self) -> [u8; 2] {
        self.oid().to_be_bytes()
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Sd memory card configuration
    pub struct Scr([u32]);
    impl Debug;
    pub u8, sd_spec, _: 59, 56;
    /// Note: Memory cards should support both 1 and 4 wide bus
    pub u8, bus_widths, _: 51, 48;
    pub bool, sd_spec3, _: 47;
    pub bool, sd_spec4, _: 42;
    pub u8, sd_spec5, _: 41, 38;
}

impl Scr<[u32; 2]> {
    pub fn version(&self) -> SdSpecVersion {
        match (self.sd_spec(), self.sd_spec3(), self.sd_spec4()) {
            (0, false, false) => SdSpecVersion::V1_0,
            (1, false, false) => SdSpecVersion::V1_10,
            (2, false, false) => SdSpecVersion::V2,
            (2, true, false) => SdSpecVersion::V3,
            (2, true, true) => SdSpecVersion::V4,
            _ => SdSpecVersion::Unsupported,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Csd {
    V1(CsdV1<[u32; 4]>),
    V2(CsdV2<[u32; 4]>),
}

impl Csd {
    pub fn parse(buf: [u32; 4]) -> Option<Csd> {
        match buf[3] >> 30 {
            0b00 => Some(Csd::V1(CsdV1(buf))),
            0b01 => Some(Csd::V2(CsdV2(buf))),
            0b10 => unimplemented!("V3"),
            _ => None,
        }
    }

    pub fn card_size(&self) -> u64 {
        match self {
            Csd::V1(csd) => csd.card_size(),
            Csd::V2(csd) => csd.card_size(),
        }
    }

    pub fn blocks(&self) -> u32 {
        match self {
            Csd::V1(csd) => csd.blocks(),
            Csd::V2(csd) => csd.blocks(),
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Card identification (Version 1)
    pub struct CsdV1([u32]);
    impl Debug;

    ///  (C_SIZE)
    pub u16, device_size, _: 73, 62;
    /// (C_SIZE_MULT)
    pub u8, device_size_multiplier, _: 49, 47;
    /// (READ_BL_LEN)
    pub u8, read_block_len, _: 83, 80;
    pub u8, version, _: 127, 126;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Card identification (Version 2)
    pub struct CsdV2([u32]);
    impl Debug;
    pub u32, device_size, _: 69, 48;
    pub u8, version, _: 127, 126;
}

impl CsdV1<[u32; 4]> {
    pub fn card_size(&self) -> u64 {
        let blocks = self.blocks();
        let blk_len: u64 = 1u64 << self.read_block_len() as u64;
        (blocks as u64 * blk_len) as u64
    }

    pub fn blocks(&self) -> u32 {
        let blocks: u32 = 1u32 + (self.device_size() as u32);
        let multiplier: u32 = 1 << (2u32 + (self.device_size_multiplier() as u32));
        blocks * multiplier
    }
}

impl CsdV2<[u32; 4]> {
    /// Card size in bytes
    /// The memory capacity is calculated as (device_size (A.k.a C_SIZE) + 1) * 512kb
    pub fn card_size(&self) -> u64 {
        self.blocks() as u64 * 512
    }

    /// Number of blocks
    /// Block size is fixed at 512
    pub fn blocks(&self) -> u32 {
        (self.device_size() + 1) * 1024
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// Sd Card Status
    pub struct SdStatus([u32]);
    impl Debug;
    /// bus_width
    pub u8, bus_width, _: 511, 510;
    /// Secure mode
    pub bool, secure_mode, _: 509;
    /// Card type
    pub u16, sd_card_type, _: 495, 480;
    pub u32, protected_area_size, _: 479, 448;
    pub u8, speed_class, _: 447, 440;
    pub u8, app_perf_class, _: 339, 336;
    pub bool, discard_support, _: 313;
}
