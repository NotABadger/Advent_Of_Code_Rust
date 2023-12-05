mod translation_range;
mod plant_data;
mod data_parser;

use input_file_lib::read_file_content;
use plant_data::PlantData;
use translation_range::*;

fn main() {
    let file_content = read_file_content().expect("File is expected as parameter");

    let mut plant_data_list: Vec<PlantData> = Vec::new();
    let mut translation_tables: Vec<TranslationTable> = data_parser::parse_data_from_file_content(&file_content, &mut plant_data_list);



}
