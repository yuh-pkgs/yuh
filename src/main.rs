mod package;

use std::{env, time::SystemTime};

use git2::Repository;
use package::Package;
use yansi::Paint;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];

    match option.as_str() {
        "-i" | "--install" => {
            let package_name = &args[2];
            let repository_name = format!("https://github.com/{}/{}", "Pacrus", package_name);

            let folder = format!("./{}/", package_name);
            let start_time = SystemTime::now();

            
            log(format!("Cloning from repository {}", repository_name), LogType::Success);

            if let Ok(repository) = Repository::clone(repository_name.as_str(), folder.to_string()) {
                log(format!("Cloned from repository"), LogType::Success);
                log(format!("Loading package from directory {}", folder), LogType::Waiting);

                let package = &mut Package::load_package(folder.to_string());

                env::set_current_dir(folder).unwrap();
        
                if let Some(package) = package {
                    log(String::from("Loaded package from directory"), LogType::Success);
                    log(String::from("Trying to execute install script"), LogType::Waiting);

                    package.execute_command();

                    log(format!("Successfully installed {} in {}ms", package_name, &start_time.elapsed().unwrap().as_millis()), LogType::Success);
                } else {
                    log(String::from("Failed to load package from directory..."), LogType::Error);
                }
            } else {
                log(format!("Failed to clone from repository {}", repository_name), LogType::Error);
            }
        },

        "-c" | "--create" => {
            let package_name = &args[2];
            let package = &mut Package::new(&package_name, "./");

            package.store_package();
        },

        _ => {
            println!("usage: pacrus <operation> [...]");
            println!("operations:");

            println!("   pacrus (-i, --install)");
        }
    };
}

fn log(message: String, log_type: LogType) {
    match log_type {
        LogType::Success => print!("{} ", Paint::green("✅")),
        LogType::Error => print!("{} ", Paint::red("✗")),
        LogType::Waiting => print!("{} ", Paint::yellow("..")),
    }

    println!("{}", message);
}
enum LogType {
    Success,
    Error,
    Waiting,
}