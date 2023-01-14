use std::{env, net, str};
use net::IpAddr;
use str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads})
        }

        else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want \r\n -h or -help to show this help message");
                return Err("help");
            }
        }


    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
        // collect() can consume any iterable, and in its most basic form, will turn one collection into another.

        // A typical pattern is to call iter() on a collection, transform the contents, and then call collect at the end to group them into a collection again.

    let program = args[0].clone();
}
