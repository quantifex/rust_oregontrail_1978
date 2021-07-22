use std::io::{Cursor, Write, BufRead, Seek, SeekFrom};

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
