use yansi::Paint;

pub fn print(message: &str, print_type: PrintType) {
    let symbol = match print_type {
        PrintType::Success => Paint::green(":white_check_mark:").to_string(),
        PrintType::Error => Paint::red("✗").to_string(),
        PrintType::Waiting => Paint::yellow("..").to_string(),
        PrintType::None => "".to_string(),
    };

    
    if print_type == PrintType::Error {
        eprintln!("{} {}", symbol, message);
    } else {
        println!("{} {}", symbol, message);
    }
}

#[derive(PartialEq)]
pub enum PrintType {
    Success,
    Error,
    Waiting,
    None,
}
