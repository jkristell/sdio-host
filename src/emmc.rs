//! eMMC-specific extensions to the core SDMMC protocol.

pub use crate::common::*;

pub use core::str;

/// Type marker for eMMC-specific extensions.
pub struct EMMC;

impl OCR<EMMC> {
    /// OCR \[7\]. Valid for eMMC. False for High Voltage, true for Dual voltage.
    pub fn is_dual_voltage_card(&self) -> bool {
        self.0 & 0x0100_0000 != 0
    }
}

/// All possible values of the CBX field of the CID register on eMMC devices.
pub enum DeviceType {
    RemovableDevice = 0b00,
    BGA = 0b01,
    POP = 0b10,
    Unknown = 0b11,
}

impl CID<EMMC> {
    /// CBX field, indicating device type.
    pub fn device_type(&self) -> DeviceType {
        match self.bytes[1] & 0x3 {
            0b00 => DeviceType::RemovableDevice,
            0b01 => DeviceType::BGA,
            0b10 => DeviceType::POP,
            _ => DeviceType::POP,
        }
    }

    /// OID field, indicating OEM/Application ID.
    ///
    /// The OID number is controlled, defined and allocated to an eMMC manufacturer by JEDEC.
    pub fn oem_application_id(&self) -> u8 {
        self.bytes[2]
    }

    /// PNM field, indicating product name.
    pub fn product_name(&self) -> &str {
        str::from_utf8(&self.bytes[3..9]).unwrap_or(&"<ERR>")
    }

    /// PRV field, indicating product revision.
    ///
    /// The return value is a (major, minor) version tuple.
    pub fn product_revision(&self) -> (u8, u8) {
        let major = (self.bytes[9] & 0xF0) >> 4;
        let minor = self.bytes[9] & 0x0F;
        (major, minor)
    }

    /// PSN field, indicating product serial number.
    pub fn serial(&self) -> u32 {
        (self.inner >> 16) as u32
    }

    /// MDT field, indicating manufacturing date.
    ///
    /// The return value is a (month, year) tuple where the month code has 1 = January and the year
    /// is an offset from either 1997 or 2013 depending on the value of `EXT_CSD_REV`.
    pub fn manufacturing_date(&self) -> (u8, u8) {
        let month = (self.inner >> 8) as u8 & 0xF0;
        let year = (self.inner >> 8) as u8 & 0x0F;
        (
            month,
            year,
        )
    }
}

impl CSD<EMMC> {
}

impl CardStatus<EMMC> {
    /// If set, the Device did not switch to the expected mode as requested by the SWITCH command
    pub fn switch_error(&self) -> bool {
        self.0 & 0x80 != 0
    }
    /// If set, one of the exception bits in field EXCEPTION_EVENTS_STATUS was set to indicate some
    /// exception has occurred. Host should check that field to discover the exception that has
    /// occurred to understand what further actions are needed in order to clear this bit.
    pub fn exception_event(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

/// eMMC hosts need to be able to create relative card addresses so that they can be assigned to
/// devices. SD hosts only ever retrieve RCAs from 32-bit card responses.
impl From<u16> for RCA<EMMC> {
    fn from(address: u16) -> Self {
        Self::from((address as u32) << 16)
    }
}
