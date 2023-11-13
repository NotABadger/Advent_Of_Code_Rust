
pub struct MoleculeData{
    pub molecule: String,
    pub increment_count: u32,
}

impl MoleculeData{
    pub fn new(molecule: &str) -> Self
    {
        Self { molecule: molecule.to_string(), increment_count: 0 }
    }
}