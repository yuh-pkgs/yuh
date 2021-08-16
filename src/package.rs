use std::process::Command;

pub struct Package {
    name: String,
    version_id: String,
    print_command_output: bool,
    command: String
}

impl Package {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version_id: "1.0".to_string(),
            print_command_output: true,
            command: "dir".to_string()
        }
    }

    pub fn execute_command(&mut self) {
        let parameters = self.command.split(" ");
        
        let mut command = "";
        let mut arguments: Vec<String> = vec![];

        for ele in parameters {
            if command == ""  {
                command = ele;
            } else {
                arguments.push(ele.to_string());
            }
        }


        let command_result = Command::new(command)
            .args(arguments)
            .spawn();

        if self.print_command_output {
            if let Ok(result) = command_result {
                println!("{:?}", result);
            }
        }
    }
}