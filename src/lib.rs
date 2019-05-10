use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

pub enum RedisResult {
    RString(String),
    RArr(Vec<String>),
}

struct CommandWriter {
    buf: String
}

impl CommandWriter {
    pub fn new() -> CommandWriter {
        CommandWriter { 
            buf: "".to_string(), 
        }
    }

    // 如果是数组
    // For Arrays the first byte of reply is '*'
    fn write_arrs(&mut self, n: usize) -> &mut Self {
        self.add_char('*');
        self.add_uint(n);
        self.add_crnl();
        self
    }

    // 如果是字符串
    // For Bulk Strings the first byte of the reply is "$"
    fn write_buik_string(&mut self, s: &str) -> &mut Self {
        if s == "" {
            // Null Bulk String
            self.add_str("$-1\r\n");
            return self
        } else {
            self.add_char('$');
            self.add_uint(s.len());
            self.add_crnl();
            self.add_str(s);
            self.add_crnl();
            self
        }    
    }

    // For Integers the first byte of the reply is ":"
    #[allow(dead_code)]
    fn write_int(&mut self, n: usize) -> &mut Self {
        self.add_char(':');
        self.add_uint(n);
        self.add_crnl();
        self
    }
    
    
    fn add_char(&mut self, s: char) {
        self.buf.push(s);
    }

    fn add_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn add_uint(&mut self, n: usize) {
       self.add_str(n.to_string().as_str());
    }

    fn add_crnl(&mut self) {
        self.add_char('\r');
        self.add_char('\n');
    }    
    
}

fn parse_io(response: &str) -> Option<RedisResult> {
    let vec: Vec<&str> = response.split("\r\n").collect();
    // match the first char
    match &vec[0][0..1] {
        "$" => return Some(RedisResult::RString(vec[1].to_string())),
        "*" => {
            let len = vec[0][1..].parse::<usize>().unwrap();
            let mut v: Vec<String> = Vec::new();
            for i in 0..len {
                v.push(vec[i + 1].to_string());
            }
            return Some(RedisResult::RArr(v));
            //return None
        }
        "+" => return Some(RedisResult::RString(vec[1].to_string())),
        "-" => panic!(vec[0].to_string()),
        _ => return None,
    }
}



/// you can use this Client to operate redis-server
pub struct RedisClient {
    io: TcpStream
}

impl RedisClient {
    /// connect to redis server
    /// 
    /// # Example
    /// 
    /// ```
    /// let sock_addr: &str = "127.0.0.1:6379";
    /// let mut client = RedisClient::new(sock_addr);
    /// ```
    pub fn new(sock_addr: &str) -> RedisClient {
        let tcp_strem = TcpStream::connect(sock_addr).unwrap();
        RedisClient {
            io : tcp_strem
        }
    }

    /// SET KEY VALUE
    pub fn set(&mut self, key: &str, val: &str) {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(3)
            .write_buik_string("SET")
            .write_buik_string(key)
            .write_buik_string(val);

        //println!("{}", cmd.buf);

        self.io.write(cmd.buf.as_bytes()).unwrap();
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    

        //let s = str::from_utf8(&buffer).unwrap();

        //println!("{}", s);
    }

    /// GET KEY
    pub fn get(mut self, key: &str) -> String {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(2)
            .write_buik_string("GET")
            .write_buik_string(key);

        self.io.write(cmd.buf.as_bytes()).unwrap();
        
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    

        let response = str::from_utf8(&buffer).unwrap();

        //println!("{}", response);

        let parse = parse_io(response).unwrap();
        
        match parse {
            RedisResult::RString(parse) => return parse.to_string(),
            _ => panic!("error")
        }
        
    }
}

