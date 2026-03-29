use std::{collections::HashMap, net::SocketAddr};

#[derive(Debug)]
pub struct ServiceInfo {
    addr: SocketAddr,
}

#[derive(Debug, Default)]
pub struct ServiceLayout {
    services: HashMap<String, ServiceInfo>,
}
