use crate::replacement::Replacement;

pub fn calibrate_machine(molecule: &str, replacements: &Vec<Replacement>, created_mutations: &mut Vec<String>)
{
    for replacement in replacements
    {
        let mut index: usize = 0;
        loop{
            let found_something = molecule[index..].find(&replacement.find);
            match found_something {
                Some(found_index) => {
                    let mut molecule_variant: String = molecule.to_string();
                    molecule_variant.replace_range(index + found_index..(index+found_index+replacement.find.len()), &replacement.replace);
                    add_mutation_to_list(molecule_variant, created_mutations);
                    index += found_index +1;       
                },
                None => return,
            }
        }
    }
}  

fn add_mutation_to_list(mutation: String, created_mutations: &mut Vec<String>)
{
    if !created_mutations.contains(&mutation)
    {
        created_mutations.push(mutation);
    }
}