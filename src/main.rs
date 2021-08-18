mod package;
mod logging;

use std::{env, time::SystemTime};

use git2::Repository;
use package::Package;
use logging::*;

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
            
            print(&format!("Cloning from repository {}", repository_name), PrintType::Waiting);

            if let Ok(_repository) = Repository::clone(repository_name.as_str(), folder.to_string()) {
                print(&format!("Cloned from repository"), PrintType::Success);
                print(&format!("Loading package from directory {}", folder), PrintType::Waiting);

                let package = &mut Package::load_package(folder.to_string());

                env::set_current_dir(folder).unwrap();
        
                if let Some(package) = package {
                    print("Loaded package from directory", PrintType::Success);
                    print("Trying to execute install script", PrintType::Waiting);

                    package.execute_command();

                    print(&format!("Successfully installed {} in {}ms", package_name, &start_time.elapsed().unwrap().as_millis()), PrintType::Success);
                } else {
                    print("Failed to load package from directory...", PrintType::Error);
                }
            } else {
                print(&format!("Failed to clone from repository {}", repository_name), PrintType::Error);
            }
        },

        "-c" | "--create" => {
            let package_name = &args[2];
            let package = &mut Package::new(&package_name, &format!("{}.package", package_name));

            print(&format!("Creating package with name {}", package_name), PrintType::Waiting);

            package.store_package();

            print("Successfully created package", PrintType::Success);
        },

        _ => {
            println!("usage: pacrus <operation> [...]");
            println!("operations:");

            println!("   pacrus (-i, --install)");
        }
    };
}