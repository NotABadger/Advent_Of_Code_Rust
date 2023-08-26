fn main() {
    const INPUT: &str = "1113122113";
    const ITTERATIONS: u32 = 50;

    let mut string = INPUT.to_string();
    for number in  0..ITTERATIONS
    {
        string = translate_string(&string);
    }
    println!("output string: {:?}", string);
    println!("output string length: {:?}", string.len());
}

fn translate_string(input_str : &str) -> String
{
    let mut ret_val : String = String::new();
    let mut mom = input_str.chars();
    let mut current_number : char = 'a'; //not a number, but needed an init val
    let mut number_count: u32 = 0;
    for char in input_str.chars()
    {
        if current_number != char
        {
            if current_number != 'a'
            {
                ret_val.push_str(&number_count.to_string());
                ret_val.push(current_number);
            }
            current_number = char;
            number_count = 1;
        }
        else {
            number_count += 1;
        }
    }
    ret_val.push_str(&number_count.to_string());
    ret_val.push(current_number);
    ret_val
}