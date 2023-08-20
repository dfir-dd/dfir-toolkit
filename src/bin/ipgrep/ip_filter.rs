use clap::ValueEnum;


#[derive(ValueEnum, Clone, PartialEq, PartialOrd)]
pub enum IpFilter {
    #[clap(name = "ipv4")]
    IPv4,

    #[clap(name = "ipv6")]
    IPv6,

    Public,
    Private,
    Loopback,
}