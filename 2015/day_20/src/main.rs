
const PACKAGES_TO_HAVE: u32 = 29_000_000; 
fn main() {
    let mut house_not_found: bool = true;
    let mut house_counter: u32 = 0;

    while house_not_found
    {
        let mut amount_of_presents: u32 = 0;
        for dwarf_counter in 1..=house_counter
        {
            if house_counter % dwarf_counter == 0
            {
                amount_of_presents += dwarf_counter*10;
            }
        }
        if amount_of_presents >= PACKAGES_TO_HAVE 
        {
            house_not_found = false;
            println!("first house to have 29 mil packages is: {}", house_counter);
            println!("it has packages: {}", amount_of_presents);
        }
        if house_counter % 10000 == 0
        {
            println!("trying house:{}", house_counter);
        }
        house_counter += 1;
    }
}