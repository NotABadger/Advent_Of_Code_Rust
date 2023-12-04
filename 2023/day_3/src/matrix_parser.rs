pub fn check_matrix_sizes(input_file_content: &str) -> Result<(usize, usize), Box<dyn std::error::Error>>
{
    //checking the matrix sizes
    let mut line_length : usize = 0;
    for  line in input_file_content.lines().enumerate()
    {
        if line_length == 0
        {
            line_length = line.1.len();
        }
        if line_length != line.1.len()
        {
            return Err(format!("First line len was: {}, line {} len is: {}", line_length, line.0, line.1.len()).into());
        }
    }

    Ok((input_file_content.lines().count(), line_length))
} 

pub fn parse_matrix(input_file_content: &str, matrix_size: (usize,usize)) -> Vec<Vec<char>>
{
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(matrix_size.0);

    for line in input_file_content.lines().enumerate()
    {
        matrix.push(Vec::with_capacity(matrix_size.1));
        for char in line.1.chars().enumerate()
        {
            matrix[line.0].push(char.1)
        }
    }

    matrix
}