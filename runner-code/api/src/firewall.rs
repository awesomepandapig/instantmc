use std::process::Command;

pub fn firewall_open(port: i64) {
    Command::new("sudo")
        .arg("ufw")
        .arg("allow")
        .arg(&port.to_string());
}
pub fn firewall_close(port: i64) {
    Command::new("sudo")
        .arg("ufw")
        .arg("delete")
        .arg("allow")
        .arg(&port.to_string());
}