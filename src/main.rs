use std::io::{Write, BufRead, stdout, stdin};
use crate::banner::*;
use crate::marksman::*;
use crate::supplies::*;

mod banner;
mod ask;
mod marksman;
mod supplies;

// GCOVR_EXCL_START
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

    loop {
        let spend_food = ask!("How much do you want to spend on food? ", &mut stdout, &mut stdin.lock());
        match supplies.buy_food(spend_food) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }

    loop {
        let spend_ammo = ask!("How much do you want to spend on ammunition? ", &mut stdout, &mut stdin.lock());
        match supplies.buy_ammo(spend_ammo) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }

    loop {
        let spend_clothes = ask!("How much do you want to spend on clothing? ", &mut stdout, &mut stdin.lock());
        match supplies.buy_clothes(spend_clothes) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }

    loop {
        let spend_misc = ask!("How much do you want to spend on miscellaneous supplies? ", &mut stdout, &mut stdin.lock());
        match supplies.buy_misc(spend_misc) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }
}
// GCOVR_EXCL_STOP