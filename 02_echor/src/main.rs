use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("J Alex Long <johnalexanderlong@gmail.com>")
        .about("Echo cli utility written in Rust")
        .arg(
            Arg::with_name("text")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
                .default_value("-"),
        )
        .get_matches();
    // println!("{:#?}", matches);
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
