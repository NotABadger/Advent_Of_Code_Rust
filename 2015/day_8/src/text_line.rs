
#[derive(Debug)]
pub struct TextLine
{
    line: String,
    literal_length: usize,
    amount_of_chars: usize,
    literal_code_len: usize,
}

impl TextLine
{
    pub fn new(line: &str) -> Self
    {
        let mut var = Self{line: line.to_string(), literal_length: line.len(), amount_of_chars: line.len()-2, literal_code_len: line.len()+2};
        var.check_value_chars_len();
        var.check_value_literal_code_len();
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

    pub fn get_litteral_code_len(&self) -> usize
    {
        self.literal_code_len
    }

    fn check_value_chars_len(&mut self)
    {
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
                continue;
            }
            if char == 'x' && last_char_slash
            {// Found hexadecimal '/x43' 
                last_char_slash = false;
                self.amount_of_chars -= 2;//x32 are three chars that represent 1, total len is -2
                continue;
                
            }
            last_char_slash = false;
        }
    }

    fn check_value_literal_code_len(&mut self)
    {
        if !self.line.contains('\\') && !self.line.contains('\"')
        {
            return;
        }
        for char in self.line.chars()
        {
            if char == '\\'
            {
                self.literal_code_len += 1;
                continue;
            }
            if char == '\"'
            {
                self.literal_code_len += 1;
            }
        }
    }
}