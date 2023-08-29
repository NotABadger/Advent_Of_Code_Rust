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
    get_sum_of_JSON_value(&parsed, &mut tot_val);
    println!("Total value is: {}", tot_val);

}

#[allow(non_snake_case)]
fn get_sum_of_JSON_value(json_object: &json::JsonValue, sum_total: &mut i32)
{
    if json_object.is_array()
    {
        for json_arr_itt in json_object.members()
        {
            if json_arr_itt.is_number()
            {
                *sum_total += json_arr_itt.as_i32().expect("Casting Json nr to i32 failed");
            }
            if json_arr_itt.is_object() || json_arr_itt.is_array()
            {
                get_sum_of_JSON_value(json_arr_itt,  sum_total);
            }
        }
    }
    else if json_object.is_object()
    {
        let mut object_painted_red: bool = false;
        let mut object_value: i32 = 0;
        for object in json_object.entries()
        {
            if object.1.is_string() && object.1.eq("red")
            {
                object_painted_red = true;
                break;
            }
            if object.1.is_object() || object.1.is_array()
            {
                get_sum_of_JSON_value(object.1,  &mut object_value);
            }
            else if object.1.is_number()
            {
                object_value += object.1.as_i32().expect("Casting Json nr to i32 failed");
            }
        }
        if !object_painted_red //ignore this if-statement, and always add object value for part 1
        {
            *sum_total += object_value;
        }
    }
}
