mod elf;

use crate::elf::Elf;
const PACKAGES_TO_HAVE: u32 = 29_000_000; 
fn main() {
    let mut house_not_found: bool = true;
    let mut current_house: u32 = 670_000;
    let mut list_of_elves: Vec<Elf> = Vec::with_capacity((PACKAGES_TO_HAVE/11) as usize);
    println!("initializing elfs");
    for i in 0..PACKAGES_TO_HAVE/11
    {
        list_of_elves.push(Elf::new(i));
    }
    println!("Done");

    while house_not_found
    {
        let mut elves_to_be_eliminated: bool = true;
        while elves_to_be_eliminated
        {//eliminate all weak elves
            if (list_of_elves[0].number * 50) < current_house
            {
                list_of_elves.remove(0);
            }
            else {
                elves_to_be_eliminated = false;
            }
        }
        
        let mut amount_of_presents: u32 = 0;
        for elf in &list_of_elves
        {
            if elf.number > current_house
            {
                break;
            }
            if current_house % elf.number == 0
            {
                amount_of_presents += elf.number*11;
            }
        }

        if amount_of_presents >= PACKAGES_TO_HAVE 
        {
            house_not_found = false;
            println!("first house to have 29 mil packages is: {}", current_house);
            println!("it has packages: {}", amount_of_presents);
        }
        if current_house % 10000 == 0
        {
            println!("trying house:{}", current_house);
        }
        current_house += 1;
    }
}