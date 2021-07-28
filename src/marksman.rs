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

    #[test]
    fn u32_goodshot() {
        assert_eq!(MarksmanQuality::GoodShot, MarksmanQuality::from_u32(2));
    }

    #[test]
    fn u32_fair() {
        assert_eq!(MarksmanQuality::Fair, MarksmanQuality::from_u32(3));
    }

    #[test]
    fn u32_needs_practice() {
        assert_eq!(MarksmanQuality::NeedPractice, MarksmanQuality::from_u32(4));
    }

    #[test]
    fn u32_shaky() {
        assert_eq!(MarksmanQuality::Shaky, MarksmanQuality::from_u32(5));
    }

    #[test]
    fn u32_unknown() {
        assert_eq!(MarksmanQuality::Unknown, MarksmanQuality::from_u32(0));
        assert_eq!(MarksmanQuality::Unknown, MarksmanQuality::from_u32(6));
        assert_eq!(MarksmanQuality::Unknown, MarksmanQuality::from_u32(100));
    }
}