use std::fmt;

#[derive(Debug)]
pub struct Infos<'a> {
    pub distro: &'a Distro,
    pub username: String,
}

#[derive(Debug)]
pub enum Distro {
    Arch,
    Debian,
}

#[derive(Debug)]
pub enum InstallationType {
    Default,
    Custom,
}

impl fmt::Display for InstallationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
