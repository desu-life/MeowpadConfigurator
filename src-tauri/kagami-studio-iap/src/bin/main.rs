use std::fs;
use hidapi::{self, HidApi};

use kagami_studio_iap::{CRC32, BinParser, HexParser, KagamiStudioIAP};



fn main() {
    let f = fs::read_to_string("./mp3-firmware-814.hex").unwrap();
    let parser = HexParser::<2048>::new(&f);
    let parts = parser.parse().unwrap();
    let hex_last_part = parts.last().unwrap();

    let b = fs::read("test.bin").unwrap();
    let parser = BinParser::<2048>::new(&b, 0x08008000);
    let parts = parser.parse().unwrap();
    let bin_last_part = parts.last().unwrap();
    
    println!("hex_last_offset: 0x{:08X}", hex_last_part.offset);
    println!("bin_last_offset: 0x{:08X}", bin_last_part.offset);

    dbg!(hex_last_part.crc32() == bin_last_part.crc32());


    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFA00;
    let hid = HidApi::new().unwrap();
    let iap = KagamiStudioIAP::new(&hid, VID, PID).unwrap();

    iap.enter_iap_mode().unwrap();

    let app_addr = iap.get_iap_address().unwrap();
    println!("app_addr: 0x{app_addr:08X}");

    println!("start download");

    iap.begin_download_usb().unwrap();
    for part in &parts {
        iap.download_file_part(part).unwrap();
    }
    iap.end_download_usb().unwrap();

    println!("download finished");

    println!("crc check");

    for part in &parts {
        iap.crc_verify_part(part).unwrap();
    }

    println!("crc check finished");

    println!("jump to app");

    iap.jump_to_app().unwrap();
    
}
