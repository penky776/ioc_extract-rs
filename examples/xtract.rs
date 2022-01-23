use ioc_extract::extract;

fn main() {
    let x = "there are ips in this test\n192.168.21.21 and ::ffff:127.0.0.1\nthe cidrs are:\n2001:0DB8:1234::/48 and \n10.0.0.0/8\n\n";
    let x = x.to_owned() + "check out https://www.google.com or www.google.com";
    let ioc = extract(&x);
    println!("IOC's:\n{:#?}", ioc);
    let ips = ioc.unwrap().ip_address;
    println!("IP's:\n{:#?}", ips);
}
