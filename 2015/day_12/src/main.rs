// mod file_processor_file;
// mod json;
mod file_processor_file;

use file_processor_file::FileProcessor;


fn main() {
    const INPUT: &str = "input.json";
    let parsed: json::JsonValue;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => parsed =json::parse(&read_content).expect("JSON file was not valid JSON format."),
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }

    let mut tot_val: i32 = 0;
    get_sum_of_JSON_contents(&parsed, &mut tot_val);
    println!("Total value is: {}", tot_val);

}

#[allow(non_snake_case)]
fn get_sum_of_JSON_contents(json_object: &json::JsonValue, sum_total: &mut i32)
{

    for object in json_object.entries()
    {
        if object.1.is_object()
        {
            get_sum_of_JSON_contents(object.1,  sum_total);
        }
        else if object.1.is_number()
        {
            *sum_total += object.1.as_i32().expect("Casting Json nr to i32 failed");
        }
        else if object.1.is_array()
        {
            for json_arr_itt in object.1.members()
            {
                if json_arr_itt.is_number()
                {
                    *sum_total += json_arr_itt.as_i32().expect("Casting Json nr to i32 failed");
                }
                if object.1.is_object()
                {
                    get_sum_of_JSON_contents(object.1,  sum_total);
                }
            }
        }
    }
}
