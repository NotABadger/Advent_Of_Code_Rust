use crate::aunt::Aunt;

pub fn parse_aunts(text_to_parse: &str) -> Vec<Aunt>
{
    let mut return_vec = Vec::with_capacity(500);
    for line in text_to_parse.lines()
    {
        println!("{}", line);
        let mut current_aunt: Aunt = Aunt::default();
        let mut line_itterator: std::str::SplitWhitespace<'_> = line.split_whitespace();
        line_itterator.next();//expext "sue"
        let mut aunt_number_string: String  = line_itterator.next().unwrap().to_string();//expect <nr>:
        aunt_number_string.truncate(aunt_number_string.len() -1); //cut off colon
        let number : u32 = aunt_number_string.trim().parse::<u32>().unwrap();
        current_aunt.set_parameter("Sue", number);

        loop
        { // parse any number of aunt properties/traits
            let itterator_val = line_itterator.next();
            if itterator_val == None
            {
                break;
            }
            let mut prop_text = itterator_val.unwrap().to_string();
            prop_text.truncate(prop_text.len() -1);

            let mut value_string: String  = line_itterator.next().unwrap().to_string();//expect number and optional comma
            if value_string.ends_with(',')
            {
                value_string.truncate(value_string.len() -1); //cut off colon
            }
            let value : u32 = value_string.trim().parse::<u32>().unwrap();
            current_aunt.set_parameter(&prop_text, value);
        }
        return_vec.push(current_aunt);
    }
    
    return_vec
}