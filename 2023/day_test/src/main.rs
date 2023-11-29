use input_file_lib;

fn main() {
    println!("Hello, world!");

    let mom: Option<String> = input_file_lib::read_file_content();
    println!("So now from the main program");
    match mom {
        Some(string_val) => println!("Found {} lines in the file", string_val.lines().count()),
        None => println!("Did not get any string from read_file_content"),        
    }
}
