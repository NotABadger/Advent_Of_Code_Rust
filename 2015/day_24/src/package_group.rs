
#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub remaining_nrs: Vec<i32>,
    pub first_grp: Vec<i32>,
    pub second_grp: Vec<i32>,
    pub third_grp: Vec<i32>,
    pub quantum_entanglement: i32,
}

impl PackageGroup {
    // pub fn new(numbers_list: &Vec<i32>) -> Self {
    //     PackageGroup{remaining_nrs: numbers_list.clone(), first_grp : Vec::new(), second_grp: Vec::new(), third_grp: Vec::new(), quantum_entanglement: i32::MAX}
    // }

}

impl Default for PackageGroup {
    fn default() -> Self {
        PackageGroup{remaining_nrs: Vec::new(), first_grp : Vec::new(), second_grp: Vec::new(), third_grp: Vec::new(), quantum_entanglement: i32::MAX}
    }
}