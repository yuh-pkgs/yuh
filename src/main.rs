mod logging;
mod package;

use std::env;

use byte_unit::Byte;
use logging::*;
use package::Package;

#[tokio::main]
async fn main() {
    let build_folder = env::current_dir();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_fallback_message();
        return;
    }

    let option = &args[1];

    match option.as_str() {
        "i" | "install" => {
            let packages: &mut Vec<Package> = &mut Vec::new();
            let installed_packages: &mut Vec<Package> = &mut Vec::new();

            print("Fetching package data...", PrintType::Waiting);

            for i in 0..args.len() - 2 {
                match Package::fetch_package(&args[i + 2]) {
                    Some(package) => packages.push(package),
                    None => (),
                }
            }

            let mut package_str = "".to_owned();
            let mut file_size = 0;

            for i in 0..packages.len() {
                let package = &packages[i];
                let package_name = package.clone().get_display_name();

                package_str.push_str(&format!("{}", package_name));

                if i != packages.len() - 1 {
                    package_str.push(',');
                }

                file_size += fs_extra::dir::get_size(format!("{}", package.path)).unwrap();
            }

            print(
                &format!("\nFound {} package(s): {}", packages.len(), &package_str),
                PrintType::None,
            );

            print(
                &format!(
                    "Total Binary Size: {} \n",
                    Byte::from_bytes(file_size as u128).get_appropriate_unit(true)
                ),
                PrintType::None,
            );

            for package in packages {
                let package_name = package.clone().get_display_name();

                print(
                    &format!("[{}] - Executing build script...", package_name),
                    PrintType::None,
                );

                package.execute_command();

                print(
                    &format!("[{}] - Cleaning installation directory...", package_name),
                    PrintType::None,
                );

                // clean the build directory of the package
                package.clean_work_directory();

                // move back to the build directory
                match &build_folder {
                    Ok(directory) => env::set_current_dir(directory).expect("Unable to move to directory."),
                    Err(_error) => println!("{:#?}", _error)
                };
                
                // register the package to the installed_packages vec
                installed_packages.push(package.clone());
            }

            print(
                &format!(
                    "\nSuccessfully installed {} package(s)!",
                    installed_packages.len()
                ),
                PrintType::None,
            );
        }

        "c" | "create" => {
            let package_name = &args[2];
            let package = &mut Package::new(&package_name, &format!("{}.package", package_name));

            print(
                &format!("Creating package with name {}", package_name),
                PrintType::Waiting,
            );

            package.store_package();

            print("Successfully created package", PrintType::Success);
        }

        "v" | "version" => {
            println!(
                "yuh-pkgs v{} - https://github.com/yuh-pkgs",
                env!("CARGO_PKG_VERSION")
            );
        }

        _ => {
            print_fallback_message();
        }
    };
}

fn print_fallback_message() {
    print("usage: yuh <operation> [...]", PrintType::Error);

    print("operations:", PrintType::None);
    print("   yuh (i, install) [packages...]", PrintType::None);
    print("   yuh (c, create) [name]", PrintType::None);
    print("   yuh (v, version)", PrintType::None);
}
