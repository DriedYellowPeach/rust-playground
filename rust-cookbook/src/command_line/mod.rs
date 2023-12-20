use clap::{Arg, Command};

pub fn build_app() {
    let matches = Command::new("My Test App")
        .version("0.1.0")
        .author("Kevin")
        .about("Learn clap usage")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .value_name("NUM")
                .help("Set a number"),
        )
        .get_matches();

    let myfile = matches
        .get_one::<String>("FILE")
        .expect("file is required")
        .as_str();
    println!("get file input: {myfile}");

    match matches.get_one::<i32>("NUM") {
        None => println!("you are not setting any number"),
        Some(s) => println!("your favorite number is {s}"),
    };
}

pub fn colorful_output() {
    use ansi_term::Color;
    println!(
        "This is {} in color, {} in color and {} in color",
        Color::Red.paint("red"),
        Color::Blue.paint("red"),
        Color::Yellow.paint("red"),
    )
}

pub fn style_output() {
    use ansi_term::{Color, Style};
    let stl = Style::new().bold().underline().paint("I am bold and underline");
    println!("{stl}");
    let stl2 = Color::Cyan.bold().underline().blink().paint("I am style two");
    println!("{stl2}");
}

#[test]
fn test_ansi_string() {
    colorful_output();
    style_output();
}

#[test]
fn test_build_app() {
    build_app();
}
