use std::str;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::*;

const ASK_FINISH_MINISTER: &str = "Would you like a minister (y/n)? ";
const ASK_FINISH_FUNERAL: &str = "Would you like a fancy funeral (y/n)? ";
const ASK_FINISH_NOTIFY_KIN: &str = "Would you like to inform your next of kin (y/n)? ";

const NOTIFY_TELEGRAPH_CHARGE: &str = "\nThat will be $4.50 for the telegraph charge.\n\n";
const NOTIFY_SADIE_WORRIED: &str = "\nYour Aunt Sadie in St. Louis is really worried about you...\n\n";

pub fn complete_trip<W: Write>(out: &mut W, supplies: &mut Supplies) {
    let prefix = include_str!("../strings/complete_prefix.txt");
    let suffix = include_str!("../strings/complete_suffix.txt");
    let supplies_status = format!("Supplies left:\n{}\n", supplies);
    out.write(prefix.as_bytes()).unwrap();
    out.write(supplies_status.as_bytes()).unwrap();
    out.write(suffix.as_bytes()).unwrap();
    out.flush().unwrap();
}

pub fn handle_death<W: Write, R: BufRead>(out: &mut W, input: &mut R) {
    out.write(include_str!("../strings/death_prefix.txt").as_bytes()).unwrap();
    let _ = ask_yn!(ASK_FINISH_MINISTER, out, input);
    let _ = ask_yn!(ASK_FINISH_FUNERAL, out, input);
    let notify_sadie = ask_yn!(ASK_FINISH_NOTIFY_KIN, out, input);
    
    if notify_sadie {
        out.write(NOTIFY_TELEGRAPH_CHARGE.as_bytes()).unwrap();
    } else {
        out.write(NOTIFY_SADIE_WORRIED.as_bytes()).unwrap();
    }
    out.write(include_str!("../strings/death_suffix.txt").as_bytes()).unwrap();
}

#[test]
fn test_complete_trip() {
    let mut supplies = Supplies::new();
    let prefix = include_str!("../strings/complete_prefix.txt");
    let suffix = include_str!("../strings/complete_suffix.txt");
    let trip_message = format!("{}Supplies left:\n{}\n{}", prefix, supplies, suffix);
    let mut c = Cursor::new(Vec::new());

    complete_trip(&mut c, &mut supplies);
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut trip_out = Vec::new();
    c.read_to_end(&mut trip_out).unwrap();

    assert_eq!(trip_message, str::from_utf8(&trip_out).unwrap());
}

#[test]
fn test_handle_death_notify() {
    let prefix = include_str!("../strings/death_prefix.txt");
    let suffix = include_str!("../strings/death_suffix.txt");
    let trip_message = format!("{}{}{}{}{}{}",
        prefix, ASK_FINISH_MINISTER, ASK_FINISH_FUNERAL, ASK_FINISH_NOTIFY_KIN, NOTIFY_TELEGRAPH_CHARGE, suffix);

    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"y\r\ny\r\ny").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    handle_death(&mut cout, &mut cin);
    cout.seek(SeekFrom::Start(0)).unwrap();
    let mut trip_out = Vec::new();
    cout.read_to_end(&mut trip_out).unwrap();

    assert_eq!(trip_message, str::from_utf8(&trip_out).unwrap());
}

#[test]
fn test_handle_death_no_notify() {
    let prefix = include_str!("../strings/death_prefix.txt");
    let suffix = include_str!("../strings/death_suffix.txt");
    let trip_message = format!("{}{}{}{}{}{}",
        prefix, ASK_FINISH_MINISTER, ASK_FINISH_FUNERAL, ASK_FINISH_NOTIFY_KIN, NOTIFY_SADIE_WORRIED, suffix);

    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"y\r\ny\r\nn").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    handle_death(&mut cout, &mut cin);
    cout.seek(SeekFrom::Start(0)).unwrap();
    let mut trip_out = Vec::new();
    cout.read_to_end(&mut trip_out).unwrap();

    assert_eq!(trip_message, str::from_utf8(&trip_out).unwrap());
}