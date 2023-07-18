// mod md5;
// mod hex_literal;

use md5::{Md5, Digest};

fn main() {
    println!("Hello, world!");
    const INPUT: [u8; 8] = *b"yzbqklnj";

    let mut output;
    let mut added_nr: u32 =0;

    loop
    {
        let mut input_vec : Vec<u8> = INPUT.to_vec();
        let number_string: String = added_nr.to_string();
        input_vec.append(&mut number_string.as_bytes().to_vec());
        
        let mut hasher = Md5::new();
        hasher.update(&input_vec);
        output = hasher.finalize();
        if output[0] == 0 && output[1] == 0 && output[2] == 0 //Last comp should be output[2] < 16 for first part
        {
            break;
        }
        added_nr += 1;
    }

    println!("Value added was: {:?}",added_nr);
}
