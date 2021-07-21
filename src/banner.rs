use std::str;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::*;

pub fn print_banner<W: Write>(out: &mut W) {
    let banner = include_str!("../strings/banner.txt");
    out.write(banner.as_bytes()).unwrap();
    out.flush().unwrap();
}

pub fn complete_trip<W: Write>(out: &mut W, supplies: Supplies) {
    let prefix = include_str!("../strings/complete_prefix.txt");
    let suffix = include_str!("../strings/complete_suffix.txt");
    let supplies_status = format!("Supplies left:\n{}\n", supplies);
    out.write(prefix.as_bytes()).unwrap();
    out.write(supplies_status.as_bytes()).unwrap();
    out.write(suffix.as_bytes()).unwrap();
    out.flush().unwrap();
}

#[test]
fn test_banner() {
    let banner = include_str!("../strings/banner.txt");
    let mut c = Cursor::new(Vec::new());

    print_banner(&mut c);
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut banner_out = Vec::new();
    c.read_to_end(&mut banner_out).unwrap();

    assert_eq!(banner, str::from_utf8(&banner_out).unwrap());
}

#[test]
fn test_complete_trip() {
    let supplies = Supplies::new();
    let prefix = include_str!("../strings/complete_prefix.txt");
    let suffix = include_str!("../strings/complete_suffix.txt");
    let trip_message = format!("{}Supplies left:\n{}\n{}", prefix, supplies, suffix);
    let mut c = Cursor::new(Vec::new());

    complete_trip(&mut c, supplies);
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut trip_out = Vec::new();
    c.read_to_end(&mut trip_out).unwrap();

    assert_eq!(trip_message, str::from_utf8(&trip_out).unwrap());
}