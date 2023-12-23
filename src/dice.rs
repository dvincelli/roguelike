use rand::Rng;

pub struct Dice {
    sides: u32,
}

impl Dice {
    pub fn new(sides: u32) -> Self {
        Dice { sides }
    }
    pub fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..(self.sides + 1))
    }
    pub fn d2() -> Self {
        Dice::new(2)
    }
    pub fn d4() -> Self {
        Dice::new(4)
    }
    pub fn d6() -> Self {
        Dice::new(6)
    }
    pub fn d8() -> Self {
        Dice::new(8)
    }
    pub fn d12() -> Self {
        Dice::new(12)
    }
    pub fn d10() -> Self {
        Dice::new(10)
    }
    pub fn d20() -> Self {
        Dice::new(20)
    }
    pub fn d100() -> Self {
        Dice::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let dice = Dice { sides: 6 };
        let result = dice.roll();
        assert!(result >= 1 && result <= 6);
    }

    #[test]
    fn test_new() {
        let dice = Dice::new(6);
        assert_eq!(dice.sides, 6);
    }

    #[test]
    fn test_roll() {
        let dice = Dice::new(6);
        let result = dice.roll();
        assert!(result >= 1 && result <= 6);
    }

    #[test]
    fn test_d2() {
        let dice = Dice::d2();
        assert_eq!(dice.sides, 2);
    }

    #[test]
    fn test_d4() {
        let dice = Dice::d4();
        assert_eq!(dice.sides, 4);
    }

    #[test]
    fn test_d6() {
        let dice = Dice::d6();
        assert_eq!(dice.sides, 6);
    }

    #[test]
    fn test_d8() {
        let dice = Dice::d8();
        assert_eq!(dice.sides, 8);
    }

    #[test]
    fn test_d10() {
        let dice = Dice::d10();
        assert_eq!(dice.sides, 10);
    }

    #[test]
    fn test_d12() {
        let dice = Dice::d12();
        assert_eq!(dice.sides, 12);
    }


    #[test]
    fn test_d20() {
        let dice = Dice::d20();
        assert_eq!(dice.sides, 20);
    }

    #[test]
    fn test_d100() {
        let dice = Dice::d100();
        assert_eq!(dice.sides, 100);
    }
}
