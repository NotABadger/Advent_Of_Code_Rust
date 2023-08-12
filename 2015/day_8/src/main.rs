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
    
    let mut total_literal_chars: u32 = 0;
    let mut total_value_chars: u32 = 0;
    for line in file_content.lines()
    {
        let txt_line: TextLine =  TextLine::new(line);
        total_literal_chars += txt_line.get_literal_len() as u32;
        total_value_chars += txt_line.get_values_len() as u32;
        lines_of_text.push(txt_line);
    }

    println!("litteral length = {:?}", total_literal_chars);
    println!("value length = {:?}", total_value_chars);
    println!("litteral minus value lengths = {:?}", total_literal_chars - total_value_chars);

}
