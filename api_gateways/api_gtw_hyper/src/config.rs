use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs::File, io::Read};

// Example of configuration yaml file.
//
// authorization_api_url: "https://auth.example.com/api/v1/authorization"
// services:
//   - path: "/users"
//     target_service: "http://user-service.default.svc.cluster.local"
//     target_port: 8080
//   - path: "/orders"
//     target_service: "http://order-service.default.svc.cluster.local"
//     target_port: 8080
//

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
    pub target_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub port: u16,
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    // Open the YAML configuration file.
    let mut file = File::open(path).unwrap();

    // Get the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the YAML content
    serde_yaml::from_str(&contents).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let cfg = load_config("config.yaml");
        assert_eq!(
            cfg.authorization_api_url,
            "https://auth.example.com/api/v1/authorization"
        );
        assert_eq!(cfg.services.len(), 2);
    }
}
