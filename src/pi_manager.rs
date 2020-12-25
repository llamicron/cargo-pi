//! This module handles all the Pi's you send your programs to
use std::net::Ipv4Addr;
use std::path::PathBuf;


pub struct Pi {
    pub name: String,
    pub ip: Ipv4Addr,
    pub default_dir: Option<PathBuf>
}

impl Pi {
    pub fn new<S: AsRef<str>>(name: S, ip: Ipv4Addr, default_dir: Option<PathBuf>) -> Self {
        Pi {
            name: name.as_ref().to_string(),
            ip,
            default_dir
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pi() {
        let ip = Ipv4Addr::new(192, 168, 0, 103);
        let pi = Pi::new("Volumio", ip, None);
        assert_eq!(pi.name, "Volumio");
        assert!(!pi.ip.is_unspecified());
    }

    #[test]
    fn test_new_pi_with_default_dir() {
        let ip = Ipv4Addr::new(192, 168, 0, 103);
        let path = PathBuf::from("/home/pi/project/");
        let pi = Pi::new("Volumio", ip, Some(path));

        assert_eq!(pi.name, "Volumio");
        assert!(!pi.ip.is_unspecified());
        assert_eq!(pi.default_dir.unwrap().to_str(), Some("/home/pi/project/"));
    }
}
