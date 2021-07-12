
#[derive(PartialEq)]
#[derive(Debug)]
pub enum MarksmanQuality {
    Unknown,
    Ace,
    GoodShot,
    Fair,
    NeedPractice,
    Shaky,
}

impl MarksmanQuality {
    pub fn from_u32(value: u32) -> MarksmanQuality {
        match value {
            1 => MarksmanQuality::Ace,
            2 => MarksmanQuality::GoodShot,
            3 => MarksmanQuality::Fair,
            4 => MarksmanQuality::NeedPractice,
            5 => MarksmanQuality::Shaky,
            _ => MarksmanQuality::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_ace() {
        assert_eq!(MarksmanQuality::Ace, MarksmanQuality::from_u32(1));
    }
}