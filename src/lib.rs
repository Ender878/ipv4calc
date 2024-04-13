use ipaddress::IPAddress;
use std::error::Error;

pub struct Config {
    pub ip: IPAddress,
    pub args: Vec<String>
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        // check the number of arguments
        if args.len() < 2 || args.len() > 4 {
            return Err("\nOptions:\n\tipaddr/cidr\t\tShow the details of the ip address\n\tipaddr subnets\t\tShow the subnets of the ip address\n\tipaddr subnets -v\tShow the details of the subnets of the ip address\n");
        }

        // parse the ip address
        let ip = IPAddress::parse(&args[1]);

        match ip {
            Ok(mut ip) => {

                // if the prefix is not provided, get the class prefix
                if ip.prefix.num == 32 {
                    ip.prefix.num = get_ip_class_prefix(&ip);
                }

                Ok(Config { ip, args })
            },
            Err(_) => return Err("ERROR: Invalid IP")
        }
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!();

    // choose the action
    if config.args.len() == 2 {
        show_ip_details(config.ip.clone());
    } else if config.args.len() >= 3 {
        if let Err(e) = show_subnets(config) {
            return Err(e.into());
        }
    }

    Ok(())
}

pub fn show_ip_details(ip: IPAddress) {
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
    println!();
}

pub fn show_subnets(config: Config) -> Result<(), String> {

    // get the number of subnets
    let subnets: u32 = match config.args[2].parse::<u32>() {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    // check if the number of subnets is greater than the number of subnets that can be created
    if subnets > 2u32.pow(32 - config.ip.prefix.num as u32 - 1) {
        return Err("The number of subnets is greater than the number of subnets that can be created.".to_string());
    }

    // get the subnets
    let subnet_ips = config.ip.subnet(config.ip.prefix.num + ((subnets as f32).log2().ceil()) as usize);

    match subnet_ips {
        Ok(subnet_ips) => {
            if config.args.len() == 4 {
                match config.args[3].as_str() {
                    "-v" => {
                        for i in 0..subnets {
                            show_ip_details(subnet_ips[i as usize].clone());
                        }
                    },
                    _ => {
                        return Err("Wrong argument.".to_string());
                    }
                }
            } else {
                for i in 0..subnets {
                    println!("\t- SUBNET {}:\t{}", i + 1, subnet_ips[i as usize].to_string());
                }
            }
        },
        Err(err) => {
            return Err(err);
        }
    }

    Ok(())

}

pub fn get_ip_class_prefix(ip: &IPAddress) -> usize {
    let first_octet = ip.parts()[0];

    if first_octet < 128 {
        8
    } else if first_octet < 192 {
        16
    } else {
        24
    }
}