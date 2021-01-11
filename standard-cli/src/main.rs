use clap;

fn main() {
    let matches = clap::app_from_crate!()
        .subcommand(
            clap::App::new("format")
                .about("Formats the input file")
                .alias("fmt")
                .arg(
                    clap::Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("FILE")
                        .about("Sets a custom config file")
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::new("INPUT")
                        .about("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
}
