mod minecraft_server;
mod dns;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use crate::minecraft_server::{Difficulty, Gamemode, WorldType};

fn main() {
    let mut port = 25566;
    let start_time = Instant::now(); // Start the timer
    
    // SET UP DNS
    /*let _ = dns::set_soa("instantmc.gg.", "@", 100, 300, "admin.instantmc.gg", "ns1.instantmc.gg", 55, 66, 44);
    let _ = dns::set_ns("instantmc.gg.", "ns1", 1800, "ns1.instantmc.gg");
    let _ = dns::set_ns("instantmc.gg.", "ns2", 1800, "ns2.instantmc.gg");
    let _ = dns::set_a("instantmc.gg.", "ns1.instantmc.gg.", 300, IpAddr::V4(Ipv4Addr::new(73, 157, 184, 122)));
    let _ = dns::set_a("instantmc.gg.", "ns2.instantmc.gg.",300, IpAddr::V4(Ipv4Addr::new(73, 157, 184, 122)));
    let _ = dns::set_a("instantmc.gg.", "instantmc.gg", 300, IpAddr::V4(Ipv4Addr::new(73, 157, 184, 122)));*/

    for i in 0..3 {
        let _server = minecraft_server::Server::new(Gamemode::Survival, Difficulty::Normal, WorldType::Normal, false, true, 500+i, port+i);
    }
    

    let end_time = Instant::now(); // Stop the timer
    let execution_time = end_time.duration_since(start_time);
    println!("Execution Time: {:?}", execution_time); // Print the execution time
}
