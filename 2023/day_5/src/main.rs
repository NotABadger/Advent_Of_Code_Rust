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

    //sort all translation tables by start number for faster look-up.
    translation_tables.iter_mut().for_each(|
            table: &mut TranslationTable | table.list.sort_by(
                | element, element2 | element.from.cmp(&element2.from)));

    let mut working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::SeedToSoil).unwrap();
    for plant in &mut plant_data_list
    {
        for result in working_trans_table.translate_range(plant.seed_nr).iter().enumerate()
        {
            //fill 0'th in original plant,
            //create more plants with new ranges
            //append new plant to plant_data_list :)
        }
        
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::SoilToFertilizer).unwrap();

    for plant in &mut plant_data_list
    {
        plant.fertilizer_nr = working_trans_table.translate(plant.soil_nr);
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::FertilizerToWater).unwrap();

    for plant in &mut plant_data_list
    {
        plant.water_nr = working_trans_table.translate(plant.fertilizer_nr);
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::WaterToLight).unwrap();

    for plant in &mut plant_data_list
    {
        plant.light_nr = working_trans_table.translate(plant.water_nr);
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::LightToTemprature).unwrap();

    for plant in &mut plant_data_list
    {
        plant.temprature_nr = working_trans_table.translate(plant.light_nr);
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::TempratureToHumidity).unwrap();

    for plant in &mut plant_data_list
    {
        plant.humidity_nr = working_trans_table.translate(plant.temprature_nr);
    }

    working_trans_table = translation_tables.iter().find(
        |trans_table | trans_table.translation_step == TranslationStep::HumidityToLocation).unwrap();

    for plant in &mut plant_data_list
    {
        plant.location_nr = working_trans_table.translate(plant.humidity_nr);
    }

    plant_data_list.sort_by(| plant, plant2 | plant.location_nr.cmp(&plant2.location_nr));
    println!("plant seed: {}, plant location: {}", plant_data_list.first().unwrap().seed_nr, plant_data_list.first().unwrap().location_nr);

}
