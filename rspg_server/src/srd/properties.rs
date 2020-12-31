use std::fmt::Display;

// Ability Score Structure
#[derive(Debug)]
pub enum Abilities {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl Display for Abilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Abilities::Str => write!(f, "Strength"),
            Abilities::Dex => write!(f, "Dexterity"),
            Abilities::Con => write!(f, "Constitution"),
            Abilities::Int => write!(f, "Intelligence"),
            Abilities::Wis => write!(f, "Wisdom"),
            Abilities::Cha => write!(f, "Charisma"),
        }
    }
}


// Ability Score Modifier Struct
#[derive(Debug)]
pub struct ASMod {
    pub ability: Abilities,
    pub modifier: i8,
}

impl Display for ASMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ability = String::new();
        match (self, ability) {
            (ASMod {ability: Abilities::Str, modifier: _}, _) => ability = String::from("Strength"),
            (ASMod {ability: Abilities::Dex, modifier: _}, _) => ability = String::from("Dexterity"),
            (ASMod {ability: Abilities::Con, modifier: _}, _) => ability = String::from("Constitution"),
            (ASMod {ability: Abilities::Int, modifier: _}, _) => ability = String::from("Intelligence"),
            (ASMod {ability: Abilities::Wis, modifier: _}, _) => ability = String::from("Wisdom"),
            (ASMod {ability: Abilities::Cha, modifier: _}, _) => ability = String::from("Charisma"),
        }
        if self.modifier > 0 {
            write!(f, "{} +{}", ability, self.modifier)
        }
        else {
            write!(f, "{} -{}", ability, self.modifier)
        }
    }
}

// Alignment structure, made up of lawness and goodness
#[derive(Debug)]
pub struct Alignment {
    pub lawness: Lawness,
    pub goodness: Goodness,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment {lawness: Lawness::Neutral, goodness: Goodness::Neutral} => write!(f, "True Neutral"),
            _ => write!(f, "{} {}", self.lawness, self.goodness),
        }
    }
}

// Enumerations for Lawness
#[derive(Debug)]
pub enum Lawness {
    Lawful,
    Neutral,
    Chaotic,
}

impl Display for Lawness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lawness::Lawful => write!(f, "Lawful"),
            Lawness::Neutral => write!(f, "Neutral"),
            Lawness::Chaotic => write!(f, "Chaotic"),
        }
    }
}

// Enumerations for Goodness
#[derive(Debug)]
pub enum Goodness {
    Good,
    Neutral,
    Evil,
}

impl Display for Goodness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goodness::Good => write!(f, "Good"),
            Goodness::Neutral => write!(f, "Neutral"),
            Goodness::Evil => write!(f, "Evil"),
        }
    }
}

// Size structure
#[derive(Debug)]
pub struct Size {
    pub height: u8,
    pub weight: u8,
}

