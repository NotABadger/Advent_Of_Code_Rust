use std::ops::{Range, RangeBounds};


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
    pub from: u64,
    pub to: u64,
    pub amount: u64,
    pub from_range: Range<u64>,
    pub to_range: Range<u64>,
}

impl TranslationRange {
    pub fn calculate_range(&mut self) {
        self.from_range = self.from..(self.from + self.amount);
        self.to_range = self.to..(self.to + self.amount);
    }
}

impl TranslationTable {
    pub fn translate(&self, input: u64) -> u64 {
        for range in &self.list {
            if range.from > input {
                return input;
            }
            if input >= range.from && input < range.from + (range.amount -1)
            {
                //examples:
                // 50 60 1, means 50 transforms to 60, but 51 stays 51
                // 50 60 0, means no transformation (0 transforamtion).
                // 50 60 2 meanse 50 & 51 can transform to 60 & 61
                let mut output: u64 = input - range.from;
                output += range.to;
                return output;
            }
        }
        input
    }

    pub fn translate_range(self, input: Range<u64>) -> Vec<Range<u64>>
    {
        //input = 1..11 -> litteral 1-10. 
        //want 1..3, 4..7 -> not translation possible, 8..10
        let mut ret_rang_list: Vec<Range<u64>> = Vec::new();
        let mut working_range: Range<u64> = input; //remaining range
        for translation_range in &self.list { //TODO, this ony works when everything fits in lists that lock into eachother!
            if working_range.start > translation_range.from_range.end
            {//input is higher than end of translation list
                continue;
            }
            if working_range.start < translation_range.from_range.start && 
            (working_range.end > translation_range.from_range.start && working_range.end < translation_range.from_range.end)
            {//start point is too low, up to this range -> AM WORKING HERE
                let mut ret_rang = working_range.start..translation_range.from_range.start;
                ret_rang_list.push(working_range);
                working_range.start = translation_range.from_range.start;
            }
            if working_range.start >= translation_range.from_range.start && working_range.start < translation_range.from_range.end
            {//working range start point is in translation table
                
                let mut last_index: u64 = working_range.start;
                if working_range.end > translation_range.from_range.end
                {//end point is outside of range
                    let mut ret_rang: Range<u64> = Range{start: working_range.start, end: translation_range.from_range.end};
                    working_range.start = translation_range.from_range.end;

                    ret_rang.start = ret_rang.start - translation_range.from;
                    ret_rang.end = ret_rang.end - translation_range.from;
                    ret_rang.start += translation_range.to;
                    ret_rang.end += translation_range.to;

                    ret_rang_list.push(working_range);
                }
                else { // working range fits in translation range
                    working_range.start = working_range.start - translation_range.from;
                    working_range.end = working_range.end - translation_range.from;
                    working_range.start += translation_range.to;
                    working_range.end += translation_range.to;
                    ret_rang_list.push(working_range);
                    return ret_rang_list;
                }
            }
        }
        ret_rang_list.push(working_range);
        ret_rang_list
    }

}