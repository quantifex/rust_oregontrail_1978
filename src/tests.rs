use std::str;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::*;

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
