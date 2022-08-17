use std::net::{SocketAddr, ToSocketAddrs};
use std::{vec};

#[derive(Debug, Clone)]
pub struct Config {
    address: SocketAddr,
    username: String,
    password: String,
}


impl Config {
    pub fn from<A, S>(address: A , username: S, password: S) -> Config
        where
            A: ToSocketAddrs<Iter = vec::IntoIter<SocketAddr>>, S: ToString {
        Config {
            address: address.to_socket_addrs().unwrap().next().unwrap(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }


    pub fn address(&self) -> SocketAddr {
        self.address
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}


