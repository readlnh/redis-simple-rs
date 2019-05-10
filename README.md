# redis-simple-rs
A really simple Rust client library for Redis inspired by [rust-redis](https://github.com/mneumann/rust-redis.git) 

## Quickstart
```
extern crate redis_simple_rs;

use redis_simple_rs::RedisClient;


fn main() {
    let sock_addr: &str = "127.0.0.1:6379";
    let mut client = RedisClient::new(sock_addr);
    client.set("x", "111");
    println!("{}", client.get("x"));
}
```

this is a simple set/get example.
