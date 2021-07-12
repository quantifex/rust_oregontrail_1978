use std::io::{Write, BufRead, stdout, stdin};
use crate::marksman::*;
use crate::supplies::*;

#[cfg(test)]
mod tests;
mod ask;
mod marksman;
mod supplies;

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();
    print_banner(&mut stdout);

    let mut marksman;
    loop {
        marksman = MarksmanQuality::from_u32(ask!(include_str!("../strings/ask_marksman.txt"), stdout, stdin));
        if marksman != MarksmanQuality::Unknown { break; }
    }

    let mut supplies = Supplies::new();
    loop {
        let spend_oxen = ask!("How much do you want to spend on your oxen team? ", &mut stdout, &mut stdin.lock());
        match supplies.buy_oxen(spend_oxen) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }
}

fn print_banner<W: Write>(out: &mut W) {
    let banner = include_str!("../strings/banner.txt");
    out.write(banner.as_bytes()).unwrap();
    out.flush().unwrap();
}
