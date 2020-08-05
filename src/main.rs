/*
use pnet::datalink;

fn main() {
    for iface in datalink::interfaces() {
        println!("{:?}", iface.ips);
    }
}
*/
use local_ipaddress;

fn main() {
    println!("{}", local_ipaddress::get().unwrap());
}