mod datlib;

use crate::datlib::Header;
use std::fs;
use std::mem::size_of;

pub fn load(filename: &String) -> Vec<u8> {
    fs::read(filename).expect("Failed to locate file")
}

pub fn get_header(v: Vec<u8>) -> Header {
    if v.len() < size_of::<Header>() {
        panic!("dat file too small!")
    }

    let mut h: Header = Default::default();
    let mut a: [u8; 4] = Default::default();
    a.copy_from_slice(&v[0x0..0x4]);
    h.file_size = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x4..0x8]);
    h.body_size = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x8..0xC]);
    h.reltab_count = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0xC..0x10]);
    h.root_count = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x10..0x14]);
    h.xref_count = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x14..0x18]);
    h.unknown_0x14 = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x18..0x1C]);
    h.unknown_0x18 = u32::from_be_bytes(a);
    a.copy_from_slice(&v[0x1C..0x20]);
    h.unknown_0x1c = u32::from_be_bytes(a);
    h
}

pub fn as_hex(v: Vec<u8>) -> Vec<String> {
    return v.into_iter().map(|byte| format!("{byte:02X}")).collect();
}

#[cfg(test)]
mod tests {
    use crate::{as_hex, get_header, load};

    #[test]
    fn test_as_hex() {
        let result = as_hex(vec![0x0, 0x1, 0x2]);
        let expected = vec!["00", "01", "02"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_header() {
        let filename = String::from("../testdata/stages/GrBb.dat");
        let contents = load(&filename);

        let header = get_header(contents.clone());

        // validate Header.file_size
        let expected_file_size = contents.len() as u32;
        let observed_file_size = header.file_size;
        assert_eq!(expected_file_size, observed_file_size);

        // validate Header.body_size

        // validate Header.reltab_count

        // validate Header.root_count

        // validate Header.xref_count
    }
}
