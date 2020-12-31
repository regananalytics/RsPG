mod srd;
mod pc;

use srd::properties::{
    Abilities, Alignment, ASMod, Lawness, Goodness, Size
};
use srd::races::Race;


fn new_elf(
    name: String, age: u8, alignment: Alignment, size: Size
) -> Race {
    let elf = Race{
        name,
        mods: vec![ ASMod { ability: Abilities::Dex, modifier: 2 } ],
        age,
        alignment,
        size,
        speed: 30,
        languages: vec![String::from("Common"), String::from("Elvish")],
    };
    return elf;
}


fn display_character(character: &Race) {
    // Display Character
    println!("*** {} ***\n", character.name);
    println!("Age: {} years", character.age);
    println!("Height: {} ft, Weight: {} lbs", character.size.height, character.size.weight);
    println!("Speed: {} ft\n", character.speed);
    println!("Alignment: {}\n", character.alignment);

    // Display Ability Score Modifiers 
    println!("Ability Score Modifiers:");
    for m in &character.mods{
        println!("  -> {}", m)
    }
    print!("\n");

    // Display Known Languages 
    println!("Known Languages:");
    for m in &character.languages{
        println!("  -> {}", m)
    }
    print!("\n");
}


fn main() {
    // Try creating an Elf
    let elf = new_elf(
        String::from("Phoenix de la Croix"),
        30,
        Alignment {lawness: Lawness::Lawful, goodness: Goodness::Good},
        Size { height: 5, weight: 125},
    );
    display_character(&elf);
}
