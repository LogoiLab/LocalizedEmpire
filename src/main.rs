#[macro_use]
extern crate clap;
extern crate hyper;
extern crate ipaddress;

use std::io::Read;

use clap::{Arg,App};

use hyper::Client;
use hyper::client::Response;
use hyper::header::Headers;
use hyper::header::Header;
use hyper::mime::*;

use ipaddress::IPAddress;

fn v0(ip_address: &IPAddress, detect_type: String) {
    if detect_type == String::from("server") {
        unimplemented!();
    } else {
        unimplemented!();
    }
}

fn v1(ip_address: &IPAddress, detect_type: String) {
    if detect_type == String::from("server") {
        let high: &IPAddress = &ip_address.first();
        let low: &IPAddress = &ip_address.last();
        let client = Client::new();

        println!("[verbose]: Starting client.");
        println!("{}", &high.to_string().split(".").collect::<Vec<&str>>().last().unwrap().to_string());
        for i in high.to_string().split(".").collect::<Vec<&str>>().last().unwrap().to_string().split_at_slash()[1].parse::<i32>().unwrap() .. low.to_string().split(".").collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap() {
            let mut low_parts: Vec<String> = low.to_string().split(".").map(|p| p.to_string()).collect();
            let mut ip: String = String::new();
            for part in 0 .. (low_parts.len() + 1) {
                if part != low_parts.len() {
                    ip.push_str(low_parts.remove(0).as_str());
                    ip.push('.');
                } else {
                    ip.push_str(i.to_string().as_str());
                }
            }

            println!("[verbose]: Sending GET request for IP address: {}", &ip);
            let response = client.get(ip.as_str()).send();
            let head_map: &Vec<String> = &response.unwrap().headers.iter().map(|h| {println!("[verbose]: Header part: {}", h.value_string()); h.value_string()}).collect();
            let mut variances: usize = 0;
            for h in head_map {
                if h.contains("HTTP/1.0 200 OK") || h.contains("Server: Microsoft-IIS/7.5") || h.contains("Date: "){
                    println!("[verbose]: Similarity found.");
                    variances = variances + 1;
                }
            }
            let mut head_match: bool;
            if variances == 3 {
                println!("[verbose]: Header match found for: {}", &ip);
                head_match = true;
            }
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
