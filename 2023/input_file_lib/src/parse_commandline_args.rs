use std::env;

pub fn retrieve_commandline_args() -> Vec<String>{
    env::args().collect()
}