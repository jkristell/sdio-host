use sdio_host::{Cid, Csd, Ocr, Scr, SdSpecVersion, SdStatus};

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
    v27_28: bool,
    v28_29: bool,
    v29_30: bool,
    v30_31: bool,
    v31_32: bool,
    v32_33: bool,
    v33_34: bool,
    v34_35: bool,
    v35_36: bool,
    v18_allowed: bool,
    over_2tb: bool,
    uhs2_card_status: bool,
    high_capacity: bool,
    powered: bool,
}

struct StatusRes {
    bus_width: u8,
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
    version: SdSpecVersion,
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
            v27_28: true,
            v28_29: true,
            v29_30: true,
            v30_31: true,
            v31_32: true,
            v32_33: true,
            v33_34: true,
            v34_35: true,
            v35_36: true,
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
            bus_width: 2,
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
            version: SdSpecVersion::V3,
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
            v27_28: true,
            v28_29: true,
            v29_30: true,
            v30_31: true,
            v31_32: true,
            v32_33: true,
            v33_34: true,
            v34_35: true,
            v35_36: true,
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
            bus_width: 2,
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
            version: SdSpecVersion::V3,
        },
    },
];

#[test]
fn test_cid() {
    for card in CARDS {
        let cid = Cid(card.cid);
        println!("{:?}", cid);

        assert_eq!(cid.serial(), card.cidr.serial);
        assert_eq!(cid.mid(), card.cidr.mid);

        let name_bytes = cid.name_bytes();
        let name = std::str::from_utf8(&name_bytes).unwrap();
        assert_eq!(name, card.cidr.name);

        let oemb = cid.oem();
        let oem = std::str::from_utf8(&oemb).unwrap();
        assert_eq!(oem, card.cidr.oem);
    }
}

#[test]
fn test_csd() {
    for card in CARDS {
        let csd = Csd::parse(card.csd).unwrap();

        if let Csd::V2(csd) = csd {
            assert_eq!(csd.version(), card.csdr.version);
            assert_eq!(csd.device_size(), card.csdr.device_size);
            assert_eq!(csd.blocks(), card.csdr.blocks);
            assert_eq!(csd.card_size(), card.csdr.size_bytes);
        } else if let Csd::V1(_csd) = csd {
        } else {
            assert!(false);
        }
    }
}

#[test]
fn test_ocr() {
    for card in CARDS {
        let ocr = Ocr(card.ocr);
        assert_eq!(ocr.v27_28(), card.ocrr.v27_28);
        assert_eq!(ocr.v28_29(), card.ocrr.v28_29);
        assert_eq!(ocr.v29_30(), card.ocrr.v29_30);
        assert_eq!(ocr.v30_31(), card.ocrr.v30_31);
        assert_eq!(ocr.v31_32(), card.ocrr.v31_32);
        assert_eq!(ocr.v32_33(), card.ocrr.v32_33);
        assert_eq!(ocr.v33_34(), card.ocrr.v33_34);
        assert_eq!(ocr.v34_35(), card.ocrr.v34_35);
        assert_eq!(ocr.v35_36(), card.ocrr.v35_36);
        assert_eq!(ocr.v18_allowed(), card.ocrr.v18_allowed);
        assert_eq!(ocr.over_2tb(), card.ocrr.over_2tb);
        assert_eq!(ocr.uhs2_card_status(), card.ocrr.uhs2_card_status);
        assert_eq!(ocr.high_capacity(), card.ocrr.high_capacity);
        assert_eq!(ocr.powered(), card.ocrr.powered);
    }
}

#[test]
fn test_sdstatus() {
    for card in CARDS {
        let status = SdStatus(card.status);

        let r = &card.statusr;
        assert_eq!(status.bus_width(), r.bus_width);
        assert_eq!(status.secure_mode(), r.secure_mode);
        assert_eq!(status.sd_card_type(), r.sd_card_type);
        assert_eq!(status.protected_area_size(), r.protected_area_size);
        assert_eq!(status.speed_class(), r.speed_class);
        assert_eq!(status.app_perf_class(), r.app_perf_class);
        assert_eq!(status.discard_support(), r.discard_support);
    }
}

#[test]
fn test_scr() {
    for card in CARDS {
        let scr = Scr(card.scr);

        let r = &card.scrr;
        assert_eq!(scr.sd_spec(), r.sd_spec);
        assert_eq!(scr.bus_widths(), r.bus_widths);
        assert_eq!(scr.sd_spec3(), r.sd_spec3);
        assert_eq!(scr.sd_spec4(), r.sd_spec4);
        assert_eq!(scr.sd_spec5(), r.sd_spec5);
        assert_eq!(scr.version(), r.version);
    }
}
