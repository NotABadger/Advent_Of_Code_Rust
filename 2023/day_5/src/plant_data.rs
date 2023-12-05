
#[derive(Debug)]
pub struct PlantData{
    pub seed_nr: u64,
    pub soil_nr: u64,
    pub fertilizer_nr: u64,
    pub water_nr: u64,
    pub light_nr: u64,
    pub temprature_nr: u64,
    pub humidity_nr: u64,
    pub location_nr: u64,
}

impl PlantData {
    pub fn new(seed_nr: u64) -> Self {
        Self { seed_nr: seed_nr, soil_nr: 0, fertilizer_nr: 0, water_nr: 0, light_nr: 0, temprature_nr: 0, humidity_nr: 0, location_nr: 0 }
    }
}