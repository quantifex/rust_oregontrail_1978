#[derive(PartialEq)]
#[derive(Debug)]
pub enum MealChoice {
    Unknown,
    Poorly,
    Moderately,
    Well,
}

impl MealChoice {
    pub fn from_u32(value: u32) -> MealChoice {
        match value {
            1 => MealChoice::Poorly,
            2 => MealChoice::Moderately,
            3 => MealChoice::Well,
            _ => MealChoice::Unknown
        }
    }
    pub fn to_food(meal: MealChoice) -> u32 {
        match meal {
            MealChoice::Poorly => 8 + 5,
            MealChoice::Moderately => 8 + 10,
            MealChoice::Well => 8 + 15,
            _ => panic!("Invalid meal choice"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meal_u32_poorly() {
        assert_eq!(MealChoice::Poorly, MealChoice::from_u32(1));
    }

    #[test]
    fn meal_poorly_food() {
        assert_eq!(8 + 5, MealChoice::to_food(MealChoice::Poorly));
    }

    #[test]
    fn meal_u32_moderately() {
        assert_eq!(MealChoice::Moderately, MealChoice::from_u32(2));
    }

    #[test]
    fn meal_moderately_food() {
        assert_eq!(8 + 10, MealChoice::to_food(MealChoice::Moderately));
    }

    #[test]
    fn meal_u32_well() {
        assert_eq!(MealChoice::Well, MealChoice::from_u32(3));
    }

    #[test]
    fn meal_well_food() {
        assert_eq!(8 + 15, MealChoice::to_food(MealChoice::Well));
    }

    #[test]
    #[should_panic]
    fn meal_panic_food() {
        MealChoice::to_food(MealChoice::Unknown);
    }

    #[test]
    fn meal_u32_unknown() {
        assert_eq!(MealChoice::Unknown, MealChoice::from_u32(0));
        assert_eq!(MealChoice::Unknown, MealChoice::from_u32(6));
        assert_eq!(MealChoice::Unknown, MealChoice::from_u32(100));
    }
}