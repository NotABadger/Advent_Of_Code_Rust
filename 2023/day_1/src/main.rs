use input_file_lib;

fn main() {
    let file_content: String = input_file_lib::read_file_content().expect("Expecting file path as first parameter");
    let mut total_value: u32 = 0;
    let mut written_numbers: Vec<String> = Vec::new();
    written_numbers.push("zero".to_string()); // not used, but this way index matches value
    written_numbers.push("one".to_string());
    written_numbers.push("two".to_string());
    written_numbers.push("three".to_string());
    written_numbers.push("four".to_string());
    written_numbers.push("five".to_string());
    written_numbers.push("six".to_string());
    written_numbers.push("seven".to_string());
    written_numbers.push("eight".to_string());
    written_numbers.push("nine".to_string());
    
    for line in file_content.lines()
    {
        let mut found_first_number: bool = false;
        let mut numbers : (u32, u32) = (0,0);
        let mut indexes : (usize, usize) = (0,0);

        let mut char_index: usize = 0;
        //check all digits
        for char in line.chars()
        {
            if char.is_digit(10)
            {
                numbers.1 = char.to_digit(10).expect("checked on line 13");
                indexes.1 = char_index;

                if !found_first_number
                {
                    numbers.0 = numbers.1 * 10;
                    indexes.0 = char_index;
                    found_first_number = true;
                }
            }
            char_index +=1;
        }

        //checking all written numbers
        for element in written_numbers.iter().enumerate()
        {
            match line.find(element.1)
            {
                Some(index) => {
                    if index < indexes.0
                    {
                        indexes.0 = index;
                        numbers.0 = (element.0 * 10) as u32; //use index as value
                    }
                } ,
                None => (),
            }
            match line.rfind(element.1)
            {
                Some(index) => {
                    if index > indexes.1
                    {
                        indexes.1 = index;
                        numbers.1 = element.0 as u32; //use index as value
                    }
                } ,
                None => (),
            }
        }
        total_value += numbers.0;
        total_value += numbers.1;
    }
    
    println!("The total value is: {}", total_value);
}
