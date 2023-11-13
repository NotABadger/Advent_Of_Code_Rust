use crate::Replacement;
use crate::molecule_data::MoleculeData;


pub fn reverse_engineer_molecule(molecule: &str, replacements: &Vec<Replacement>)
{
    let mut molecules: Vec<MoleculeData> = Vec::new();
    molecules.push(MoleculeData::new(molecule));

    
}