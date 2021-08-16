mod package;
use package::Package;

fn main() {
    let package = &mut Package::load_package(String::from("./"));

    if let Some(package) = package {
        package.execute_command();
        package.store_package();
    }
}
