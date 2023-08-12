mod file_processor_file;
mod text_line;

use file_processor_file::FileProcessor;
use text_line::TextLine;

fn main() {
    const INPUT: &str = "input.txt";
    let file_content: String;
    let mut lines_of_text: Vec<TextLine> = vec![];

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => file_content = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }
    
    let mut total_literal_length: u32 = 0;
    let mut total_value_length: u32 = 0;
    let mut total_literal_code_length: u32 = 0;
    for line in file_content.lines()
    {
        let txt_line: TextLine =  TextLine::new(line);
        total_literal_length += txt_line.get_literal_len() as u32;
        total_value_length += txt_line.get_values_len() as u32;
        total_literal_code_length += txt_line.get_litteral_code_len() as u32;
        lines_of_text.push(txt_line);
    }

    println!("original length = {:?}", total_literal_length);
    println!("value length = {:?}", total_value_length);
    println!("original code length = {:?}", total_value_length);
    println!("original minus value lengths = {:?}", total_literal_length - total_value_length);
    println!("litteral code length minus original length = {:?}", total_literal_code_length - total_literal_length);

}
