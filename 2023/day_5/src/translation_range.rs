
#[derive(Debug)]
pub struct TranslationTable {
    pub translation_step: TranslationStep,
    pub list: Vec<TranslationRange>,
}

#[derive(Debug)]
pub enum TranslationStep{
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemprature,
    TempratureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
pub struct TranslationRange {
    pub start: u32,
    pub map: u32,
    pub amount: u32,
}

