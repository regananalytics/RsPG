use crate::srd::properties;

// Enum of Races from 5e SRD
#[derive(Debug)]
pub enum Races {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
}

// Racial Traits
pub trait Modifiers {
    fn modifiers(&self) -> Vec<properties::ASMod>;
}

pub trait Age {
    fn age(&self) -> u8;
}

pub trait Alignment {
    fn alignment(&self) -> properties::Alignment;
}

pub trait Size {
    fn size(&self) -> properties::Size;
}

pub trait Speed {
    fn speed(&self) -> u8;
}

pub trait Languages {
    fn languages(&self) -> Vec<String>;
}

#[derive(Debug)]
pub struct Race {
    pub name: String,
    pub mods: Vec<properties::ASMod>,
    pub age: u8,
    pub alignment: properties::Alignment,
    pub size: properties::Size,
    pub speed: u8,
    pub languages: Vec<String>,
}