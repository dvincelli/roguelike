use crate::dice::Dice;
use std::mem;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug)]
pub struct Character {
    name: String,
    level: u32,
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32,
    class: CharacterClass,
    alignment: Alignment,
}

#[derive(EnumIter, EnumString, Debug, PartialEq)]
pub enum CharacterClass {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(EnumIter, EnumString, Debug, PartialEq)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

impl Character {
    pub fn new(
        name: String,
        level: u32,
        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        wisdom: u32,
        charisma: u32,
        class: CharacterClass,
        alignment: Alignment,
    ) -> Self {
        Character {
            name,
            level,
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
            class,
            alignment,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_level(&self) -> u32 {
        self.level
    }
    pub fn get_strength(&self) -> u32 {
        self.strength
    }
    pub fn get_dexterity(&self) -> u32 {
        self.dexterity
    }
    pub fn get_constitution(&self) -> u32 {
        self.constitution
    }
    pub fn get_intelligence(&self) -> u32 {
        self.intelligence
    }
    pub fn get_wisdom(&self) -> u32 {
        self.wisdom
    }
    pub fn get_charisma(&self) -> u32 {
        self.charisma
    }
    pub fn generate(name: String) -> Self {
        let level = 1;

        let d20 = Dice::d20();
        let d12 = Dice::d12();

        let strength = d20.roll();
        let dexterity = d20.roll();
        let constitution = d20.roll();
        let intelligence = d20.roll();
        let wisdom = d20.roll();
        let charisma = d20.roll();

        let num_classes = CharacterClass::iter().count() as u32;
        let num_alignments = Alignment::iter().count() as u32;

        let class = CharacterClass::iter()
            .nth((d20.roll() % num_classes) as usize)
            .unwrap();

        let alignment = Alignment::iter()
            .nth((d20.roll() % num_alignments) as usize)
            .unwrap();

        Self::new(
            name,
            level,
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
            class,
            alignment,
        )
    }
}
