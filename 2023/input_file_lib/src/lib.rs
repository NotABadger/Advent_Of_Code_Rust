mod file_reader;
mod parse_commandline_args;

pub fn read_file_content() -> Option<String>
{
    let arguments: Vec<String> = parse_commandline_args::retrieve_commandline_args();
    match arguments.get(1)
    {
        Some(first_parameter) => {
            if file_reader::check_file_exists(first_parameter)
            {
                return Some(file_reader::read_file(first_parameter).expect("Tried to read a file that should exist, but does not."));
            }
            else {
                println!("I did find a parameter, but its not a file");
                return None;
            }
        }
        
        None => {
            println!("could not find any parameter, or parameter file does not exist");
            None
        }
    }
}