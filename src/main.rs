use std::io::*;
use std::str::from_utf8;
use chrono::{NaiveDate, Duration};
use crate::banner::*;
use crate::marksman::*;
use crate::supplies::*;
use crate::mileage::*;

mod banner;
mod ask;
mod marksman;
mod supplies;
mod mileage;

const ASK_OXEN_SPEND: &str = "How much do you want to spend on your oxen team? ";
const ASK_FOOD_SPEND: &str = "How much do you want to spend on food? ";

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
    ask_oxen(&mut stdout, &mut stdin.lock(), &mut supplies);
    ask_food(&mut stdout, &mut stdin.lock(), &mut supplies);

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

    let mut mileage = Mileage::new();
    let mut trip_date: NaiveDate = NaiveDate::from_ymd(1847, 03, 29);
    println!("After all your purchases, you now have ${} left\n", supplies.money_left());

    loop {
        println!("\n=================================================================");
        if mileage.traveled() >= 2040 {
            complete_trip(&mut stdout, &mut supplies);
            std::process::exit(0);
        }

        if supplies.food_left() <= 12 {
            println!("You'd better do some hunting or buy some food, and soon!!!!");
        }
        println!("Total mileage traveled: {}", mileage.traveled());
        println!("It is now {}\n", trip_date.format("%A %d-%b-%Y"));
        println!("Supplies remaining:\n{}", supplies);

        'turn_input: loop {
            let turn_action = ask!("Do you want to?\n  1) Continue\n", &mut stdout, &mut stdin.lock());
            match turn_action {
                1 => break 'turn_input,
                _ => continue 'turn_input,
            }
        }

        trip_date += Duration::days(14);
        mileage.turn(200, supplies.oxen_left());
    }

}

fn ask_oxen<W: Write, R: BufRead>(out: &mut W, input: &mut R, supplies: &mut Supplies) {
    loop {
        let spend_oxen = ask!(ASK_OXEN_SPEND, out, input);
        match supplies.buy_oxen(spend_oxen) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }
}

fn ask_food<W: Write, R: BufRead>(out: &mut W, input: &mut R, supplies: &mut Supplies) {
    loop {
        let spend_food = ask!(ASK_FOOD_SPEND, out, input);
        match supplies.buy_food(spend_food) {
            Ok(_) => { break; }
            Err(e) => println!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ask_oxen() {
        let mut supplies = Supplies::new();
        let mut cout = Cursor::new(Vec::new());
        let mut cin = Cursor::new(Vec::new());
        cin.write(b"200").unwrap();
        cin.seek(SeekFrom::Start(0)).unwrap();

        ask_oxen(&mut cout, &mut cin, &mut supplies);
        assert_eq!(200, supplies.oxen_left());

        cout.seek(SeekFrom::Start(0)).unwrap();
        let mut ask_out = Vec::new();
        cout.read_to_end(&mut ask_out).unwrap();

        assert_eq!(ASK_OXEN_SPEND, from_utf8(&ask_out).unwrap());
    }

    #[test]
    fn test_ask_food() {
        let mut supplies = Supplies::new();
        let mut cout = Cursor::new(Vec::new());
        let mut cin = Cursor::new(Vec::new());
        cin.write(b"100").unwrap();
        cin.seek(SeekFrom::Start(0)).unwrap();

        ask_food(&mut cout, &mut cin, &mut supplies);
        assert_eq!(100, supplies.food_left());

        cout.seek(SeekFrom::Start(0)).unwrap();
        let mut ask_out = Vec::new();
        cout.read_to_end(&mut ask_out).unwrap();

        assert_eq!(ASK_FOOD_SPEND, from_utf8(&ask_out).unwrap());
    }
}