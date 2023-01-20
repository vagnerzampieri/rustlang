use std::io;
use std::net::ToSocketAddrs;

fn trace() {
    println!("{:-^40}", "");
}

fn main() {
    trace();
    println!("Coloque o domínio que será verificado");
    trace();

    let mut host = String::new();

    io::stdin()
        .read_line(&mut host)
        .expect("Error reading console");

    trace();

    let host_port = (host.trim(), 0);
    let socket_iter = host_port.to_socket_addrs().unwrap();

    for socket in socket_iter {
        if socket.is_ipv4() {
            println!("IPv4 {}", socket.ip());
        } else if socket.is_ipv6() {
            println!("IPv6 {}", socket.ip());
        } else {
            println!("Error");
        }
    }
}
