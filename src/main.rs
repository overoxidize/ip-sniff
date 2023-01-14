use std::{env, net};
use net::IpAddr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

fn main() {

    let args: Vec<String> = env::args().collect();
        // collect() can consume any iterable, and in its most basic form, will turn one collection into another.

        // A typical pattern is to call iter() on a collection, transform the contents, and then call collect at the end to group them into a collection again.

    for i in &args {
        println!("{}", i);
    }


    println!("{:?}", args);
}
