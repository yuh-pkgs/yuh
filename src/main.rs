mod package;

use std::env;

use git2::Repository;
use package::Package;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];

    match option.as_str() {
        "-i" | "--install" => {
            let package_name = &args[2];
            let folder = format!("./{}/", package_name);
        
            let url = format!("https://github.com/{}/{}", "Pacrus", package_name);
        
            if let Ok(repository) = Repository::clone(url.as_str(), folder.to_string()) {
                let package = &mut Package::load_package(folder.to_string());
        
                env::set_current_dir(folder).unwrap();
        
                if let Some(package) = package {
                    package.execute_command();
                }
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
