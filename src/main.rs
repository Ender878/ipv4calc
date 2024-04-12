use std::env;
use std::process; // to exit the program without panic
use ipaddress::IPAddress; // to manage ip addresses

fn main() {

    let args: Vec<String> = env::args().collect(); // get the arguments

    // chose the mode
    // 1. ipv4calc <ipaddr>/<cidr>
    // 2. ipv4calc <ipaddr> <number of subnets>
    match args.len() {
        2 => {
            show_ip_details(&args[1]);
        },
        3 => {
            show_subnets(&args[1], args[2].parse().expect("ERROR: Wrong argument."));
        },
        _ => {
            println!("ERROR: Wrong arguments.\nipv4calc <ipaddr>/<cidr>");
            process::exit(1);
        }
    }

}

fn show_ip_details(ip: &String) {
    // parse the ip
    let ip = IPAddress::parse(ip)
        .unwrap_or_else( |_err| {
            println!("- ERROR: Invalid IP");
            process::exit(1);
        });

    println!("INFO ABOUT IPV4 ADDRESS {}", ip.to_string());
    println!("\t- Subnet Bits:\t\t{}", ip.prefix.num % 8);
    println!("\t- Number of subnets:\t{}", (2 as u32).pow((ip.prefix.num % 8) as u32));
    println!("\t- Host Bits:\t\t{}", ip.prefix.host_prefix());
    println!("\t- Hosts per Subnet:\t{}", ip.size() - 2 as u32);
    println!("\t- SUBNET MASK:\t\t{} ", ip.netmask().to_string());
    println!("\t- NETWORK:\t\t{}", ip.network().to_string());
    println!("\t- BROADCAST:\t\t{}", ip.broadcast().to_string());
    println!("\t- First Host:\t\t{}", ip.first().to_string());
    println!("\t- Last Host:\t\t{}", ip.last().to_string());
}

fn show_subnets(ip: &String, subnets: u8) {
    // parse the ip
    let ip = IPAddress::parse(ip)
        .unwrap_or_else( |_err| {
            println!("- ERROR: Invalid IP");
            process::exit(1);
        });

    let subnet_ips = ip.subnet(ip.prefix.num + ((subnets as f32).log2().ceil()) as usize);

    match subnet_ips {
        Ok(subnet_ips) => {
            for i in 0..subnets {
                println!("{}", subnet_ips[i as usize].to_string());
            }
        },
        Err(err) => {
            println!("- ERROR: {}", err);
            process::exit(1);
        }
    }

}