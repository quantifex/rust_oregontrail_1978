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


#[test]
fn test_value_1() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"1").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();

    let value = ask!("test? ", cout, cin);

    assert_eq!(1, value);
}

#[test]
fn test_value_200() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"200").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();

    let value = ask!("test? ", cout, cin);

    assert_eq!(200, value);
}


#[test]
fn test_value_999_with_return() {
    let mut cout = Cursor::new(Vec::new());
    let mut cin = Cursor::new(Vec::new());
    cin.write(b"999\r\n").unwrap();
    cin.seek(SeekFrom::Start(0)).unwrap();

    let value = ask!("test? ", cout, cin);

    assert_eq!(999, value);
}
