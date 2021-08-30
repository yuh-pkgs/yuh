mod logging;
mod package;

use std::env;

use logging::*;
use package::Package;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];

    match option.as_str() {
        "i" | "install" => {
            let packages: &mut Vec<Package> = &mut Vec::new();

            print("Fetching package data...", PrintType::Waiting);

            for i in 0..args.len() - 2 {
                if let Some(package) = Package::fetch_package(&args[i]) {
                    packages.push(package);
                }
            }

            let mut package_str = "".to_owned();

            for i in 0..packages.len() {
                let package = &packages[i];

                package_str.push_str(&format!("{} {}", package.name, package.version_id));

                if i != packages.len() - 1 {
                    package_str.push(',');
                }
            }

            print(
                &format!("\n Found {} packages: {} \n", packages.len(), &package_str),
                PrintType::None,
            );

            for package in packages {
                print(
                    &format!(
                        "Executing {} {} build script...",
                        package.name, package.version_id
                    ),
                    PrintType::None,
                );

                package.execute_command();
            }
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

        _ => {
            print("usage: yuh <operation> [...]", PrintType::Error);

            print("operations:", PrintType::None);
            print("   yuh (-i, --install) [packages...]", PrintType::None);
            print("   yuh (-c, --create) [name]", PrintType::None);
        }
    };
}
