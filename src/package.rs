use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::Write, process::Command};

#[derive(Serialize, Deserialize, Clone)]
pub struct Package {
    name: String,
    version_id: String,
    command: String,
    path: String,

    print_command_output: bool,
}

impl Package {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),

            version_id: "1.0".to_string(),
            command: "ls".to_string(),

            print_command_output: true,
        }
    }

    pub fn load_package(path: String) -> Option<Self> {
        let contents = fs::read_dir(path).unwrap();

        for content in contents {
            if let Ok(content) = content {
                if content
                    .file_name()
                    .to_str()
                    .map(|s| s.to_string())
                    .unwrap()
                    .ends_with(".package")
                {
                    return Some(
                        serde_yaml::from_str(
                            fs::read_to_string(content.path().as_os_str().to_str().unwrap())
                                .unwrap()
                                .as_str(),
                        )
                        .expect("Unable to read file."),
                    );
                }
            }
        }

        return None;
    }

    pub fn store_package(&mut self) {
        let file = &mut File::create(self.clone().path);

        if let Ok(file) = file {
            file.write_all(
                serde_yaml::to_string(self)
                    .expect("Unable to parse to YAML")
                    .as_bytes(),
            )
            .unwrap();
        }
    }

    pub fn execute_command(&mut self) {
        let parameters = self.command.split(" ");

        let mut command = "";
        let mut arguments: Vec<String> = vec![];

        for ele in parameters {
            if command == "" {
                command = ele;
            } else {
                arguments.push(ele.to_string());
            }
        }

        let command_result = Command::new(command).args(arguments).output();

        if self.print_command_output {
            if let Ok(result) = command_result {
                println!("{:?}", String::from_utf8(result.stdout).unwrap());
                println!("{:?}", String::from_utf8(result.stderr).unwrap());
            } else {
                println!("Unable to execute command.");
            }
        }
    }
}
