
#[derive(Debug)]
pub struct TranslationTable {
    pub translation_step: TranslationStep,
    pub list: Vec<TranslationRange>,
}

#[derive(Debug, PartialEq)]
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
    pub start: u64,
    pub map: u64,
    pub amount: u64,
}

impl TranslationTable {
    pub fn translate(&self, input: u64) -> u64 {
        for range in &self.list {
            if range.start > input {
                return input;
            }
            if input >= range.start && input < range.start + (range.amount -1)
            {
                //examples:
                // 50 60 1, means 50 transforms to 60, but 51 stays 51
                // 50 60 0, means no transformation (0 transforamtion).
                // 50 60 2 meanse 50 & 51 can transform to 60 & 61
                let mut output: u64 = input - range.start;
                output += range.map;
                return output;
            }
        }
        input
    }

}