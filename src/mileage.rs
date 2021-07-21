

pub struct Mileage {
    traveled: u32,
}

impl Mileage {
    pub fn new() -> Mileage {
        Mileage {
            traveled: 0,
        }
    }

    pub fn traveled(&mut self) -> u32 {
        self.traveled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mileage_constructor() {
        let mut mileage = Mileage::new();
    
        assert_eq!(0, mileage.traveled());
    }
}