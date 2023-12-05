pub struct PlantData{
    pub seed_nr: u32,
    pub soil_nr: u32,
    pub fertilizer_nr: u32,
    pub water_nr: u32,
    pub light_nr: u32,
    pub temprature_nr: u32,
    pub humidity_nr: u32,
    pub location_nr: u32,
}

impl PlantData {
    pub fn new(seed_nr: u32) -> Self {
        Self { seed_nr: seed_nr, soil_nr: 0, fertilizer_nr: 0, water_nr: 0, light_nr: 0, temprature_nr: 0, humidity_nr: 0, location_nr: 0 }
    }
}