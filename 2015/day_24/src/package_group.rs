
#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub remaining_nrs: Vec<i32>,
    pub first_grp: Vec<i32>,
    pub second_grp: Vec<i32>,
    pub third_grp: Vec<i32>,
    pub quantum_entanglement: u64,
}

impl PackageGroup {
    pub fn calculate_quantum_entanglement(&mut self) {
        self.quantum_entanglement = 1;
        self.first_grp.iter().for_each(| num | self.quantum_entanglement = self.quantum_entanglement * (*num) as u64);
    }
}

impl Default for PackageGroup {
    fn default() -> Self {
        PackageGroup{remaining_nrs: Vec::new(), first_grp : Vec::new(), second_grp: Vec::new(), third_grp: Vec::new(), quantum_entanglement: u64::MAX}
    }
}