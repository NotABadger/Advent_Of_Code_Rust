
#[derive(Debug)]
pub struct TextLine
{
    line: String,
    literal_length: usize,
    amount_of_chars: usize,
    slashes_found: usize,
    hexa_found: usize,
}

impl TextLine
{
    pub fn new(line: &str) -> Self
    {
        let line_for_saving = line[1..line.len()-1].to_string();
        let mut var = Self{line: line_for_saving, literal_length: line.len(), amount_of_chars: 0, slashes_found: 0, hexa_found: 0};
        var.check_value_chars_len();
        var
    }

    pub fn get_literal_len(&self) -> usize
    {
        self.literal_length
    }
    
    pub fn get_values_len(&self) -> usize
    {
        self.amount_of_chars
    }

    fn check_value_chars_len(&mut self)
    {
        self.amount_of_chars = self.literal_length -2; // minus 2 for quotes in beginning & end
        if !self.line.contains('\\')
        {
            return;
        }
        
        let mut last_char_slash: bool = false;
        for char in self.line.chars()
        {
            if char == '\\' && !last_char_slash
            {//found escape character
                last_char_slash = true;
                self.amount_of_chars -= 1;
                self.slashes_found += 1;
                continue;
            }
            if char == 'x' && last_char_slash
            {// Found hexadecimal '/x43' 
                last_char_slash = false;
                self.amount_of_chars -= 2;//x32 are three chars that represent 1, total len is -2
                self.hexa_found += 1;
                continue;
                
            }
            last_char_slash = false;
        }
    }
}