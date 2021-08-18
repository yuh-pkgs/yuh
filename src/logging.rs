use yansi::Paint;

pub fn print(message: &str, print_type: PrintType) {
    match print_type {
        PrintType::Success => print!("{} ", Paint::green("✅")),
        PrintType::Error => print!("{} ", Paint::red("✗")),
        PrintType::Waiting => print!("{} ", Paint::yellow("..")),
        PrintType::None => (),
    }

    println!("{}", message);
}
pub enum PrintType {
    Success,
    Error,
    Waiting,
    None,
}
