use std::{net::IpAddr, fmt::format};
use redis::Commands;
extern crate redis;

/*pub fn set_soa(
    zone: &str,
    target: &str,
    ttl: i64,
    minttl: i64,
    mbox: &str,
    ns: &str,
    refresh: i64,
    retry: i64,
    expire: i64,
) -> redis::RedisResult<()> {
    let record = json!({
        "soa": {
            "ttl": ttl,
            "minttl": minttl,
            "MBox": mbox,
            "ns": ns,
            "refresh": refresh,
            "retry": retry,  
            "expire": expire
        }
    });
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _:  redis::RedisResult<()> = con.hset(&zone, &target, &serde_json::to_string(&record).unwrap());
    Ok(())
}

pub fn set_ns(
    zone: &str,
    target: &str,
    ttl: i64,
    host: &str,
) -> redis::RedisResult<()>{
    let record = json!({
        "ns": {
            "ttl": ttl,
            "host": host,
        }
    });
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _:  redis::RedisResult<()> = con.hset(&zone, &target, &serde_json::to_string(&record).unwrap());
    Ok(())
}*/

pub fn set_a(
    target: &str, 
    ttl: i64, 
    ip: IpAddr
) -> redis::RedisResult<()>{
    let key = format!("A/{}", target);
    let val = format!("{} IN A {}", ttl, ip.to_string());
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: redis::RedisResult<()> = con.set(&key, &val);
    Ok(())
}

/*pub fn set_srv(
    zone: &str, 
    target: &str,
    ttl: i64, 
    priority: i64, 
    weight: i64, 
    port: i16
) -> redis::RedisResult<()> {
    let record = json!({
        "srv": [
            {
                "ttl": ttl,
                "priority": priority,
                "weight": weight,
                "port": port,
                "target": target
            }
        ]
    });
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _:  redis::RedisResult<()> = con.hset(&zone, &target, &serde_json::to_string(&record).unwrap());
    Ok(())
}*/