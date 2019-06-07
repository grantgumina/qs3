extern crate clap;
extern crate rusoto_core;
extern crate rusoto_s3;

use std::fs::File;
use std::fs::metadata;
use std::io::Read;
use std::vec::Vec;
use if_chain::if_chain;
use clap::{Arg, ArgMatches, App, SubCommand};
use rusoto_core::{Region};
use rusoto_s3::{S3Client, S3, Bucket, PutObjectRequest};

pub mod constants;

fn import(matches: &ArgMatches) {

    if_chain! {
        if let _file_path = matches.value_of("file_path");
        if let _bucket_path = matches.value_of("s3_location");
        then {
            println!("Yup all arguments supplied!");
        }
    }

}

fn upload_file_to_s3(file_path: &str, bucket_url: &str) {

    let s3_client = S3Client::new(Region::UsWest2);
    let mut local_file = File::open(file_path).expect(constants::FILE_NOT_FOUND_ERROR);
    let mut local_file_contents: Vec<u8> = Vec::new();
    
    // Handle files which are too big to upload in one shot
    match local_file.read_to_end(&mut local_file_contents) {
        Ok(_) => {
            
            let request = PutObjectRequest {
                bucket: bucket_url.to_owned(),
                key: file_path.to_owned(),
                body: Some(local_file_contents.into()),
                ..Default::default()
            };
            
            s3_client.put_object(request).sync().unwrap();

        },
        Err(error) => {
            println!("{:#?}", error);
        }
    }

}

fn export(matches: &ArgMatches) {

    if_chain! {
        if let file_path = matches.value_of("file_path").unwrap();
        if let bucket_url = matches.value_of("s3_location").unwrap();

        then {

            // Check if path is a directory
            let path_metadata = metadata(file_path).expect(constants::FILE_NOT_FOUND_ERROR);
            
            if path_metadata.is_file() {
                upload_file_to_s3(file_path, bucket_url);
            } else {
                // Upload a directory
            }

        }
    }

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
        _ => println!("{}", constants::QS3_DEFAULT_HELP_MESSAGE),
    }
}



// match s3_client.list_buckets().sync() {
//     Ok(output) => {
//         match output.buckets {
//             Some(bucket_list) => {
//                 for bucket in bucket_list {
//                     println!("{:#?}", bucket.name);
//                 }
//             },
//             None => println!("No buckets found!"),
//         }
//     }, 
//     Err(error) => {
//         println!("Error: {:?}", error);
//     },
// }