mod package;
use package::Package;

fn main() {
    Package::new("hello!").execute_command();
}
