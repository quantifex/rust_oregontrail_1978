use std::io::{Cursor, Write, BufRead, Seek, SeekFrom};
use crate::marksman::*;
use crate::meal::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TurnAction {
    Fort,
    Hunt,
    Continue,
}

#[macro_export]
macro_rules! ask {
    ( $question: expr, $out: expr, $input: expr ) => {
        {
            $out.write($question.as_bytes()).unwrap();
            $out.flush().unwrap();

            let mut buffer = String::new();
            $input.read_line(&mut buffer).unwrap();
            buffer.retain(|buffer| !buffer.is_whitespace());
            match buffer.parse::<u32>() {
                Ok(n) => n,
                Err(_e) => 0
            }
        }
    };
}

/// Ask the user to answer a Yes/No question (default to Yes)
#[macro_export]
macro_rules! ask_yn {
    ( $question: expr, $out: expr, $input: expr ) => {
        {
            $out.write($question.as_bytes()).unwrap();
            $out.flush().unwrap();

            let mut buffer = String::new();
            $input.read_line(&mut buffer).unwrap();
            buffer.retain(|buffer| !buffer.is_whitespace());

            buffer.push('y');
            match buffer.chars().next().unwrap() {
                'n' => false,
                _ => true,
            }
        }
    };
}

#[macro_export]
macro_rules! ask_ok {
    ( $ask: expr ) => {
        {
            loop {
                match $ask {
                    Ok(_) => { break; }
                    Err(e) => println!("{}", e),
                }
            }
        }
    };
}

pub fn ask_marksman<W: Write, R: BufRead>(out: &mut W, input: &mut R) -> MarksmanQuality {
    loop {
        let marksman = MarksmanQuality::from_u32(ask!(include_str!("../strings/ask_marksman.txt"), out, input));
        if marksman != MarksmanQuality::Unknown { return marksman; }
    }
}

pub fn ask_meal<W: Write, R: BufRead>(out: &mut W, input: &mut R) -> MealChoice {
    loop {
        let meal = MealChoice::from_u32(ask!("Do you want to eat: 1) Poorly, 2) Moderately, or 3) Well", out, input));
        if meal != MealChoice::Unknown { return meal; }
    }
}

pub fn ask_continue<W: Write, R: BufRead>(out: &mut W, input: &mut R) -> TurnAction {
    loop {
        let action = ask!("Do you want to 1) Continue? ", out, input);
        if action == 1 { return TurnAction::Continue; }
    }
}

pub fn ask_hunt_continue<W: Write, R: BufRead>(out: &mut W, input: &mut R) -> TurnAction {
    loop {
        let action = ask!("Do you want to 1) Hunt or 2) Continue? ", out, input);
        match action {
            1 => return TurnAction::Hunt,
            2 => return TurnAction::Continue,
            _ => continue,
        }
    }
}

pub fn ask_fort_hunt_continue<W: Write, R: BufRead>(out: &mut W, input: &mut R) -> TurnAction {
    loop {
        let action = ask!("Do you want to 1) Stop at a Fort, 2) Hunt or 3) Continue? ", out, input);
        match action {
            1 => return TurnAction::Fort,
            2 => return TurnAction::Hunt,
            3 => return TurnAction::Continue,
            _ => continue,
        }
    }
}

#[test]
fn test_ask_value_1() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask!("test? ", cout, cin);
    assert_eq!(1, value);
}

#[test]
fn test_ask_value_200() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"200").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask!("test? ", cout, cin);
    assert_eq!(200, value);
}


#[test]
fn test_ask_value_999_with_return() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"999\r\n").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask!("test? ", cout, cin);
    assert_eq!(999, value);
}

#[test]
fn test_ask_yn_value_y() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"y").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask_yn!("test? ", cout, cin);
    assert_eq!(true, value);
}

#[test]
fn test_ask_yn_value_n() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"n").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask_yn!("test? ", cout, cin);
    assert_eq!(false, value);
}

#[test]
fn test_ask_yn_value_test() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"test").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask_yn!("test? ", cout, cin);
    assert_eq!(true, value);
}

#[test]
fn test_ask_yn_value_invalid() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b" ").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    // Assert
    let value = ask_yn!("test? ", cout, cin);
    assert_eq!(true, value);
}

#[test]
fn test_ask_ok_success() {
    let mut okay_result: bool = false;
    let mut func = || -> Result<(), core::fmt::Error> {
        okay_result = true;
        Ok(())
    };
    // Assert
    ask_ok!(func());
    assert!(okay_result);
}

#[test]
fn test_ask_marksman() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_marksman(&mut cout, &mut cin);
    assert_eq!(MarksmanQuality::Ace, action);
}

#[test]
fn test_ask_meal() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"3").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let meal = ask_meal(&mut cout, &mut cin);
    assert_eq!(MealChoice::Well, meal);
}

#[test]
fn test_ask_continue() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Continue, action);
}

#[test]
fn test_ask_hunt_continue_hunt() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_hunt_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Hunt, action);
}

#[test]
fn test_ask_hunt_continue_continue() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"2").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_hunt_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Continue, action);
}

#[test]
fn test_ask_fort_hunt_continue_fort() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_fort_hunt_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Fort, action);
}

#[test]
fn test_ask_fort_hunt_continue_hunt() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"2").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_fort_hunt_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Hunt, action);
}

#[test]
fn test_ask_fort_hunt_continue_continue() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"3").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();
    let action = ask_fort_hunt_continue(&mut cout, &mut cin);
    assert_eq!(TurnAction::Continue, action);
}