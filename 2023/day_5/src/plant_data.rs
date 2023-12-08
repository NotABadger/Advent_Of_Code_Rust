use std::ops::Range;


#[derive(Debug)]
pub struct PlantData{
    pub seed_nr: Range<u64>,
    pub soil_nr: Range<u64>,
    pub fertilizer_nr: Range<u64>,
    pub water_nr: Range<u64>,
    pub light_nr: Range<u64>,
    pub temprature_nr: Range<u64>,
    pub humidity_nr: Range<u64>,
    pub location_nr: Range<u64>,
}

impl PlantData {
    pub fn new(seed_nr: Range<u64>) -> Self {
        Self { seed_nr: seed_nr, soil_nr: 0..0, fertilizer_nr: 0..0, water_nr: 0..0, light_nr: 0..0, temprature_nr: 0..0, humidity_nr: 0..0, location_nr: 0..0 }
    }
}