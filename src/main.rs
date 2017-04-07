#[macro_use]
extern crate clap;
extern crate hyper;
extern crate ipaddress;

use clap::{Arg,App};

use hyper::Client;

use ipaddress::IPAddress;

fn main() {
    let matches = App::new(crate_name!())
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about(crate_description!())
                    .arg(Arg::with_name("ipaddress")
                        .short("i")
                        .long("ipaddress")
                        .value_name("IP")
                        .help("IP or IP range to test")
                        .required(true)
                    )
                    .arg(Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .value_name("CLIENT | SERVER")
                        .help("Sets the type of Empire shell to search for")
                        .required(true)
                    )
                    .arg(Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Sets the level of verbosity")
                        .required(false)
                    )
                    .get_matches();

    let mut ip_address: Result<IPAddress, String> = match IPAddress::is_valid(matches.value_of("ipaddress").unwrap()) {
        true => IPAddress::parse(matches.value_of("ipaddress").unwrap()),
        false => panic!("Invalid IP address or IP range entered.")
    };

    let detect_type = match String::from(matches.value_of("type").unwrap()).to_lowercase().as_str() {
        "client" => panic!("Not yet implemented check back later"),
        "server" => String::from(matches.value_of("type").unwrap()).to_lowercase(),
        _ => panic!("Invalid type provided."),
    };

    let verbosity = match matches.occurrences_of("verbose") {
        0 => 0,
        1 => 1,
        2 | _ => {
            println!("Don't be crazy!");
            1
        },
    };

    let client = Client::new();

    let res = client.get("http://google.com/").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
}
