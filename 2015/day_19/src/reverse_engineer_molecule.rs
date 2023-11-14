use std::collections::VecDeque;

use crate::Replacement;
use crate::molecule_data::MoleculeData;


pub fn reverse_engineer_molecule(molecule: &str, replacements: &Vec<Replacement>)
{
    let mut molecules: VecDeque<MoleculeData> = VecDeque::new();
    let mut current_working_mol: MoleculeData = MoleculeData::new(molecule);
    let mut most_amount_molecules : usize = 0;

    loop {
        calculate_all_mutations(&current_working_mol, replacements, &mut molecules);
        if molecules.iter().find(|x| x.molecule == "e").is_some() ||
            molecules.len() == 0
        { //found nothing or something
            break;
        }
        if molecules.len() > most_amount_molecules
        {
            most_amount_molecules = molecules.len();
        }
        current_working_mol = molecules.pop_front().unwrap();
        
    }

    println!("Max amount of molecules in list {}", most_amount_molecules);
    if molecules.len() > 0
    {
        println!("e found in {} steps", molecules.iter().find(|x| x.molecule == "e").expect("boo").increment_count)
    }
    else {
        println!("No e was found.");
    }
}

fn calculate_all_mutations(working_molecule: &MoleculeData, replacements: &Vec<Replacement>, molecule_mutations_list: &mut VecDeque<MoleculeData>)
{
    for replacement in replacements
    {// search string until all have been found
        let mut index: usize = 0;
        loop{
            let found_something = working_molecule.molecule[index..].find(&replacement.replace);
            match found_something {
                Some(found_index) => {
                    let mut molecule_variant: String = working_molecule.molecule.to_string();
                    molecule_variant.replace_range(index + found_index..(index+found_index+replacement.replace.len()), &replacement.find);
                    add_mutation_to_list(molecule_variant, working_molecule.increment_count, molecule_mutations_list);
                    index += found_index +1;       
                },
                None => break,
            }
        }
    }
}

fn add_mutation_to_list(mutation: String, mutation_count: u32, molecule_mutations_list: &mut VecDeque<MoleculeData>)
{
    if mutation.contains('e') && mutation.len() > 1
    { //final computation should only be 'e', not in the middle of a string
        return;
    }
    match molecule_mutations_list.iter_mut().find(|mutation_mol | mutation_mol.molecule.eq(&mutation))
    {
        None => molecule_mutations_list.push_back(MoleculeData { molecule: mutation, increment_count: mutation_count +1}),
        Some(molecule_data_val) => {
            if molecule_data_val.increment_count > mutation_count +1
            {
                molecule_data_val.increment_count = mutation_count +1;
            }
        },
    }
}