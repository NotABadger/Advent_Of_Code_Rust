use std::collections::VecDeque;

use crate::Replacement;
use crate::molecule_data::MoleculeData;


pub fn reverse_engineer_molecule(molecule: &str, replacements: &Vec<Replacement>)
{
    let mut current_working_mol: MoleculeData = MoleculeData::new(molecule);
    let mut replacements_local : Vec<Replacement> = replacements.clone();
    replacements_local.sort_by(|a, b|b.replace.len().cmp(&a.replace.len()));
    loop{
        if !replace_largest_possible_option(&mut current_working_mol, &replacements_local)
        {
            if current_working_mol.molecule == "e"
            {
                println!("Found E in {} steps", current_working_mol.increment_count);
            }
            else {
                println!("Did not find \"e\", and could not continue");
            }
            return;
        }
        dbg!(current_working_mol.molecule.len());
    }
}

fn replace_largest_possible_option(working_mol: &mut MoleculeData, replacements: &Vec<Replacement>) -> bool
{
    for replacement in replacements
    {
        match working_mol.molecule.find(&replacement.replace) {
            Some(found_index) => {
                working_mol.increment_count += 1;
                working_mol.molecule.replace_range(found_index..found_index+replacement.replace.len(), &replacement.find);
                return true;
            },
            _ => (),
        }
    }
    return false;
}
