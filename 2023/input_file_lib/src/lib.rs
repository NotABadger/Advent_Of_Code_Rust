mod file_reader;
mod parse_commandline_args;


pub fn read_file_content() -> Result<String,  Box<dyn std::error::Error>>
{
    let arguments: Vec<String> = parse_commandline_args::retrieve_commandline_args();
    match arguments.get(1)
    {
        Some(first_parameter) => {
            if file_reader::check_file_exists(first_parameter)
            {
                return Ok(file_reader::read_file(first_parameter)?);
            }
            return Err("File does not exist".into());
        }
        None => {
            Err("could not find any parameter, or parameter file does not exist".into())
        }
    }
}