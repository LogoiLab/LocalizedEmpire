#[macro_use]
extern crate clap;
extern crate hyper;
extern crate ipaddress;

use std::io::Read;

use clap::{Arg,App};

use hyper::Client;
use hyper::client::Response;

use ipaddress::IPAddress;

fn v0(ip_address: &IPAddress, detect_type: String) {
    if detect_type == String::from("client") {
        unimplemented!();
    } else {
        unimplemented!();
    }
}

fn v1(ip_address: &IPAddress, detect_type: String) {
    if detect_type == String::from("client") {
        let high: &IPAddress = &ip_address.first();
        let low: &IPAddress = &ip_address.last();
        let client = Client::new();

        println!("[verbose]: Starting client.");
        for i in high.to_string().split(".").collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap() .. low.to_string().split(".").collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap() {
            let ip: String = low.to_string() + i.to_string().as_str();
            println!("[verbose]: Sending GET request for IP address: {}", &ip);
            let response = client.get("http://google.com/").send().unwrap();
            println!("{:?}", response);
        }
    } else {
        unimplemented!();
    }
}

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
    let client = Client::new();
    let mut response = client.get("http://google.com/").send().unwrap();

    let body = response.bytes();
    let mut temp: Vec<u8> = vec!();
    for byte in body {
        temp.push(byte.unwrap());
    }
    println!("{:?}", String::from_utf8(temp));

    let ip_address: Result<IPAddress, String> = match IPAddress::is_valid(matches.value_of("ipaddress").unwrap()) {
        true => IPAddress::parse(matches.value_of("ipaddress").unwrap()),
        false => panic!("[fatal]: Invalid IP address or IP range entered.")
    };

    let detect_type: String = match String::from(matches.value_of("type").unwrap()).to_lowercase().as_str() {
        "client" => panic!("[fatal]: Not yet implemented check back later."),
        "server" => String::from(matches.value_of("type").unwrap()).to_lowercase(),
        _ => panic!("[fatal]: Invalid type provided."),
    };

    match matches.occurrences_of("verbose") {
        0 => v0(&ip_address.unwrap(), detect_type),
        1 => v1(&ip_address.unwrap(), detect_type),
        2 | _ => {
            println!("[warning]: Don't be crazy!");
            v1(&ip_address.unwrap(), detect_type);
        },
    };
}
