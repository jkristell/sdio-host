use sdio_host::{BusWidth, SDSpecVersion};
use sdio_host::{SDStatus, CID, CSD, OCR, SCR};

struct TestCard {
    cid: [u32; 4],
    cidr: CidRes,
    csd: [u32; 4],
    csdr: CsdRes,
    ocr: u32,
    ocrr: OcrRes,
    status: [u32; 16],
    statusr: StatusRes,
    scr: [u32; 2],
    scrr: ScrRes,
}

struct CidRes {
    mid: u8,
    serial: u32,
    name: &'static str,
    oem: &'static str,
}

struct CsdRes {
    version: u8,
    device_size: u32,
    blocks: u32,
    size_bytes: u64,
}

struct OcrRes {
    voltage_window_mv: (u16, u16),
    v18_allowed: bool,
    over_2tb: bool,
    uhs2_card_status: bool,
    high_capacity: bool,
    powered: bool,
}

struct StatusRes {
    bus_width: BusWidth,
    secure_mode: bool,
    sd_card_type: u16,
    protected_area_size: u32,
    speed_class: u8,
    app_perf_class: u8,
    discard_support: bool,
}

struct ScrRes {
    sd_spec: u8,
    bus_widths: u8,
    sd_spec3: bool,
    sd_spec4: bool,
    sd_spec5: u8,
    version: SDSpecVersion,
}

static CARDS: &[TestCard] = &[
    // Panasonic 8 Gb Class 4
    TestCard {
        cid: [4093715758, 333095359, 808993095, 22036825],
        cidr: CidRes {
            mid: 1,
            serial: 3668033524,
            name: "Y08AG",
            oem: "PA",
        },
        csd: [171966712, 968064896, 1532559360, 1074659378],
        csdr: CsdRes {
            version: 1,
            device_size: 14771,
            blocks: 15126528,
            size_bytes: 7744782336,
        },
        ocr: 3237969920,
        ocrr: OcrRes {
            voltage_window_mv: (2700, 3600),
            v18_allowed: false,
            over_2tb: false,
            uhs2_card_status: false,
            high_capacity: true,
            powered: true,
        },

        status: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134676480, 33722368, 50331648, 2147483648,
        ],
        statusr: StatusRes {
            bus_width: BusWidth::Four,
            secure_mode: false,
            sd_card_type: 0,
            protected_area_size: 50331648,
            speed_class: 2, // Class 4
            app_perf_class: 0,
            discard_support: false,
        },
        scr: [16777216, 37060608],
        scrr: ScrRes {
            sd_spec: 2,
            bus_widths: 5,
            sd_spec3: true,
            sd_spec4: false,
            sd_spec5: 0,
            version: SDSpecVersion::V3,
        },
    },
    // Sandisk 8 Gb Class 4
    TestCard {
        cid: [2197869198, 2149469225, 1429223495, 55788627],
        cidr: CidRes {
            mid: 3,
            serial: 508307843,
            name: "SU08G",
            oem: "SD",
        },
        csd: [171983022, 993492864, 1532559360, 1074659378],
        csdr: CsdRes {
            version: 1,
            device_size: 15159,
            size_bytes: 7948206080,
            blocks: 15523840,
        },

        ocr: 3237969920,
        ocrr: OcrRes {
            voltage_window_mv: (2700, 3600),
            v18_allowed: false,
            over_2tb: false,
            uhs2_card_status: false,
            high_capacity: true,
            powered: true,
        },

        status: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184877056, 33722368, 50331648, 2147483648,
        ],
        statusr: StatusRes {
            bus_width: BusWidth::Four,
            secure_mode: false,
            sd_card_type: 0,
            protected_area_size: 50331648,
            speed_class: 2, // Class 4
            app_perf_class: 0,
            discard_support: false,
        },

        scr: [0, 37060609],
        scrr: ScrRes {
            sd_spec: 2,
            bus_widths: 5,
            sd_spec3: true,
            sd_spec4: false,
            sd_spec5: 0,
            version: SDSpecVersion::V3,
        },
    },
];

#[test]
fn test_cid() {
    for card in CARDS {
        let cid: CID = card.cid.into();
        println!("{:?}", cid);

        assert_eq!(cid.serial(), card.cidr.serial);
        assert_eq!(cid.manufacturer_id(), card.cidr.mid);

        assert_eq!(cid.product_name(), card.cidr.name);
        assert_eq!(cid.oem_id(), card.cidr.oem);
    }
}

#[test]
fn test_csd() {
    for card in CARDS {
        let csd: CSD = card.csd.into();
        println!("{:?}", csd);

        assert_eq!(csd.version(), card.csdr.version);
        assert_eq!(csd.block_count(), card.csdr.blocks);
        assert_eq!(csd.card_size(), card.csdr.size_bytes);
    }
}

#[test]
fn test_ocr() {
    for card in CARDS {
        let ocr: OCR = card.ocr.into();
        println!("{:?}", ocr);

        assert_eq!(
            ocr.voltage_window_mv().unwrap(),
            card.ocrr.voltage_window_mv
        );
        assert_eq!(ocr.v18_allowed(), card.ocrr.v18_allowed);
        assert_eq!(ocr.over_2tb(), card.ocrr.over_2tb);
        assert_eq!(ocr.uhs2_card_status(), card.ocrr.uhs2_card_status);
        assert_eq!(ocr.high_capacity(), card.ocrr.high_capacity);
        assert_eq!(ocr.is_busy(), !card.ocrr.powered);
    }
}

#[test]
fn test_sdstatus() {
    for card in CARDS {
        let status: SDStatus = card.status.into();
        println!("{:?}", status);

        let r = &card.statusr;
        assert_eq!(status.bus_width(), r.bus_width);
        assert_eq!(status.secure_mode(), r.secure_mode);
        assert_eq!(status.sd_memory_card_type(), r.sd_card_type);
        assert_eq!(status.protected_area_size(), r.protected_area_size);
        assert_eq!(status.speed_class(), r.speed_class);
        assert_eq!(status.app_perf_class(), r.app_perf_class);
        assert_eq!(status.discard_support(), r.discard_support);
    }
}

#[test]
fn test_scr() {
    for card in CARDS {
        let scr: SCR = card.scr.into();
        println!("{:?}", scr);

        let r = &card.scrr;
        assert_eq!(scr.bus_widths(), r.bus_widths);
        assert_eq!(scr.version(), r.version);
    }
}
