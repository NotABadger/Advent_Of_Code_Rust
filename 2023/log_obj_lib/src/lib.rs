#[derive(Debug, Clone)]
pub struct LogObject {
    logs: Vec<String>,
}

impl LogObject {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log_line(&mut self, log_msg: String)
    {
        self.logs.push(log_msg);
    }

    pub fn print_to_stdout(&self) {
        for line in &self.logs {
            println!("{}", line);
        }
    }

    pub fn return_logs_list(&self) -> Vec<String> {
        self.logs.clone()
    }

    pub fn return_logs(&self) -> String {
        let mut ret_val: String = String::new();
        for line in &self.logs {
            ret_val.push_str(line);
            if !line.ends_with('\n')
            {
                ret_val.push('\n');
            }
        }
        ret_val
    }
}