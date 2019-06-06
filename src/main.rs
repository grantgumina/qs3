extern crate clap;
extern crate rusoto_core;
extern crate rusoto_s3;

use if_chain::if_chain;
use clap::{Arg, ArgMatches, App, SubCommand};


fn import(matches: &ArgMatches) {

    if_chain! {
        if let file_path = matches.value_of("file_path");
        if let bucket_path = matches.value_of("s3_location");
        then {
            println!("Yup all arguments supplied!");
        }
    }

}

fn export(matches: &ArgMatches) {
}

fn main() {
    let matches = App::new("qs3")
                    .version("v0.0.1-alpha")
                    .author("Grant Gumina")
                    .about("Moves data between file systems and S3 based object storage")
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                    )
                    .subcommand(SubCommand::with_name("import")
                        .about("Import data from S3 to your file system")

                        .arg(Arg::with_name("s3_location")
                            .short("b")
                            .long("bucket")
                            .help("URL for data living in S3")
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("file_path")
                            .short("p")
                            .long("path")
                            .help("Path where imported data lives")
                            .takes_value(true)
                        )
                    )
                    .subcommand(SubCommand::with_name("export")
                        .about("Export data to S3")

                        .arg(Arg::with_name("s3_location")
                            .short("b")
                            .long("bucket")
                            .help("URL for data living in S3")
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("file_path")
                            .short("p")
                            .long("path")
                            .help("Exported file data path")
                            .takes_value(true)
                        )
                    )
                    .get_matches();

    // Parse out commands
    match matches.subcommand() {
        ("import", Some(m)) => import(m),
        ("export", Some(m)) => export(m),
        _ => println!("Use `qs3 -h` for help"),
    }
}
