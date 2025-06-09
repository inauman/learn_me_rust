enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}



fn route(ip_kind: IpAddrKind) {
   // let home = IpAddr {
   //     kind: IpAddrKind::V4,
   //     address: String::from("127.0.0.1"),
   // };
    
    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    let home2 = IpAddr::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127,0,0,1);
    let loopback3 = IpAddr3::V6(String::from("::1"));
    //println!("Routing to {}", ip_kind);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    println!("Hello, world!");
}
