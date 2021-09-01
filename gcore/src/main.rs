use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("gurita")
        .version("0.1.0")
        .about("GURITA")
        .arg(clap::Arg::with_name("daemon")
            .short("d")
            .required(false)
            .help("Run as daemon")
        )
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets custom conifg file.")
            .takes_value(true)
            .default_value("/etc/gurita/gurita.toml")
        )
        .arg(clap::Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Set listen port")
            .takes_value(true)
        )
        .arg(clap::Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity (-v -vv -vvv)")
        )
        .get_matches();

    println!("Value of config set: {}", matches.value_of("config").unwrap());


    Ok(())
}
