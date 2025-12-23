use serde::{Deserialize, Serialize};

/// =======================
/// ROOT CONFIG
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub firewall: ConfigFirewall,
    pub objects: ConfigObjects,
    pub nat: ConfigNat,
    pub rules: Vec<ConfigRule>,
    pub vpn: Option<ConfigVpn>,
    pub logging: Option<ConfigLogging>,
    pub security_features: Option<ConfigSecurityFeatures>,
}

/// =======================
/// FIREWALL
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFirewall {
    pub name: String,
    pub environment: String,
    pub default_policy: ConfigDefaultPolicy,
    pub zones: Vec<ConfigZone>,
    pub interfaces: Vec<ConfigInterface>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDefaultPolicy {
    pub ingress: PolicyAction,
    pub egress: PolicyAction,
    pub interzone: PolicyAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigZone {
    pub name: String,

    #[serde(rename = "type")]
    pub zone_type: ZoneType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigInterface {
    pub name: String,
    pub zone: String,
    pub ip: String,
}

/// =======================
/// OBJECTS
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigObjects {
    pub address_groups: ConfigAddressGroups,
    pub service_groups: ConfigServiceGroups,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigAddressGroups {
    pub admins_ips: Vec<String>,
    pub monitoring_servers: Vec<String>,
    pub dmz_web_servers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigServiceGroups {
    pub web_services: Vec<String>,
    pub admin_services: Vec<String>,
    pub vpn_services: Vec<String>,
}

/// =======================
/// NAT
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigNat {
    pub snat: Vec<ConfigSnat>,
    pub dnat: Vec<ConfigDnat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSnat {
    pub name: String,
    pub source: String,
    pub destination: String,
    pub translated_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDnat {
    pub name: String,
    pub protocol: DnatProtocol,
    pub public_ip: String,
    pub public_port: u16,
    pub private_ip: String,
    pub private_port: u16,
}

/// =======================
/// RULES
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRule {
    pub id: u32,
    pub name: String,
    pub source_zone: String,
    pub destination_zone: String,
    pub source_ip: String,
    pub destination_ip: String,
    pub service: String,
    pub action: RuleAction,
    pub log: bool,
}

/// =======================
/// VPN
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigVpn {
    pub ipsec: Vec<ConfigIpsecTunnel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIpsecTunnel {
    pub name: String,
    pub peer_ip: String,
    pub pre_shared_key: String,
    pub ike: ConfigIke,
    pub ipsec: ConfigIpsec,
    pub local_subnets: Vec<String>,
    pub remote_subnets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIke {
    pub version: String,
    pub encryption: String,
    pub integrity: String,
    pub dh_group: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIpsec {
    pub encryption: String,
    pub integrity: String,
    pub pfs: bool,
}

/// =======================
/// LOGGING
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLogging {
    pub firewall: ConfigFirewallLogging,
    pub nat: bool,
    pub vpn: bool,
    pub destination: ConfigLogDestination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFirewallLogging {
    pub allow: bool,
    pub deny: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLogDestination {
    #[serde(rename = "type")]
    pub destination_type: String,
    pub endpoint: String,
}

/// =======================
/// SECURITY FEATURES
/// =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSecurityFeatures {
    pub ids_ips: ConfigIdsIps,
    pub anti_ddos: ConfigAntiDdos,
    pub geo_blocking: ConfigGeoBlocking,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIdsIps {
    pub enabled: bool,
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigAntiDdos {
    pub enabled: bool,
    pub threshold_pps: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeoBlocking {
    pub deny_countries: Vec<String>,
}

/// =======================
/// ENUMS
/// =======================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyAction {
    Allow,
    Deny,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZoneType {
    Untrusted,
    SemiTrusted,
    Trusted,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DnatProtocol {
    Tcp,
    Udp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Deny,
}
