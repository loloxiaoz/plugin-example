use crate::util::desensitization::{IpFormat, NameFormat, PasswordFormat};
use enum_iterator::IntoEnumIterator;
use serde::{de::DeserializeOwned, Serialize};
use strum::*;

#[allow(clippy::large_enum_variant)]
#[derive(AsRefStr, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]

pub enum Target {
    Artifact(Artifact),
    Device(Device),
    Features(Features),
    #[strum(serialize = "ip_connection")]
    #[serde(rename = "ip_connection")]
    IpConnection(IpConnection),
    #[strum(serialize = "domain_name")]
    #[serde(rename = "domain_name")]
    DomainName(String),
    None,
}

impl TargetIdentity for Target {
    fn identity(&self) -> String {
        let target_str = self.as_ref().to_lowercase();
        let func = |iden: String| {
            if iden.is_empty() {
                target_str
            } else {
                format!("{}.{}", target_str, iden)
            }
        };
        match self {
            Target::Artifact(artifact) => func(artifact.identity()),
            Target::Device(device) => func(device.identity()),
            Target::Features(features) => func(features.identity()),
            Target::IpConnection(ip_connection) => func(ip_connection.identity()),
            Target::DomainName(_) => func(String::new()),
            Target::None => String::new(),
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    AsRefStr,
    IntoEnumIterator,
    EnumString,
    ToString,
)]
#[serde(rename_all = "lowercase")]
pub enum MimeType {
    #[strum(serialize = "cmd")]
    #[serde(rename = "cmd")]
    Cmd,

    // 扫描器上的扫描任务
    #[strum(serialize = "scanner/task")]
    #[serde(rename = "scanner/task")]
    ScannerTask,

    // 扫描器设备
    #[strum(serialize = "scanner/device")]
    #[serde(rename = "scanner/device")]
    ScannerDevice,

    // 扫描器模板
    #[strum(serialize = "scanner/template")]
    #[serde(rename = "scanner/template")]
    ScannerTemplate,

    // 扫描结果
    #[strum(serialize = "scanner/result")]
    #[serde(rename = "scanner/result")]
    ScannerResult,

    // 弱口令扫描任务服务类型
    #[strum(serialize = "scanner/service_type")]
    #[serde(rename = "scanner/service_type")]
    ScannerServiceType,

    #[strum(serialize = "probe/heartbeat")]
    #[serde(rename = "probe/heartbeat")]
    ProbeHeartbeat,

    #[strum(serialize = "probe/app")]
    #[serde(rename = "probe/app")]
    ProbeApp,

    #[strum(serialize = "probe/schedule")]
    #[serde(rename = "probe/schedule")]
    ProbeSchedule,

    #[strum(serialize = "probe/capture_policy")]
    #[serde(rename = "probe/capture_policy")]
    ProbePolicy,

    #[strum(serialize = "config/rule")]
    #[serde(rename = "config/rule")]
    ConfigRule,

    #[strum(serialize = "config/vrrp")]
    #[serde(rename = "config/vrrp")]
    ConfigVrrp,

    #[strum(serialize = "config/running")]
    #[serde(rename = "config/running")]
    ConfigRunning,

    #[strum(serialize = "config/sn")]
    #[serde(rename = "config/sn")]
    ConfigSn,

    #[strum(serialize = "config/version")]
    #[serde(rename = "config/version")]
    ConfigVersion,

    #[strum(serialize = "config/ha")]
    #[serde(rename = "config/ha")]
    ConfigHa,

    #[strum(serialize = "config/arp")]
    #[serde(rename = "config/arp")]
    ConfigArp,

    #[strum(serialize = "config/mac")]
    #[serde(rename = "config/mac")]
    ConfigMac,

    #[strum(serialize = "config/route")]
    #[serde(rename = "config/route")]
    ConfigRoute,

    #[strum(serialize = "config/restart")]
    #[serde(rename = "config/restart")]
    ConfigRestart,

    #[strum(serialize = "config/shutdown")]
    #[serde(rename = "config/shutdown")]
    ConfigShutdown,

    #[strum(serialize = "device/sn")]
    #[serde(rename = "device/sn")]
    DeviceSn,

    #[strum(serialize = "device/version")]
    #[serde(rename = "device/version")]
    DeviceVersion,

    #[strum(serialize = "device/running_config")]
    #[serde(rename = "device/running_config")]
    DeviceRunningConfig,

    #[strum(serialize = "device/rule")]
    #[serde(rename = "device/rule")]
    DeviceRule,

    #[strum(serialize = "device/vrrp")]
    #[serde(rename = "device/vrrp")]
    DeviceVrrp,

    #[strum(serialize = "device/nat")]
    #[serde(rename = "device/nat")]
    DeviceNat,

    #[strum(serialize = "device/ha")]
    #[serde(rename = "device/ha")]
    DeviceHa,

    #[strum(serialize = "device/arp")]
    #[serde(rename = "device/arp")]
    DeviceArp,

    #[strum(serialize = "device/mac")]
    #[serde(rename = "device/mac")]
    DeviceMac,

    #[strum(serialize = "device/route")]
    #[serde(rename = "device/route")]
    DeviceRoute,

    #[strum(serialize = "device/restart")]
    #[serde(rename = "device/restart")]
    DeviceRestart,

    #[strum(serialize = "device/shutdown")]
    #[serde(rename = "device/shutdown")]
    DeviceShutdown,
}

impl MimeType {
    pub fn need_replace_special_char_impl(mime_type: &str) -> bool {
        // 非采集类, 如: 策略下发类命令 保持设备原始报文
        if mime_type == MimeType::DeviceRule.as_ref() {
            return false;
        }
        // 采集类, 如: running_config 设备返回报文需要预处理 剔除特殊字符等
        true
    }
    pub fn need_replace_special_char(&self) -> bool {
        MimeType::need_replace_special_char_impl(self.as_ref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artifact {
    pub mime_type: String,
    payload: Payload,
}

impl TargetIdentity for Artifact {
    fn identity(&self) -> String {
        self.mime_type.clone()
    }
}

pub type Result<T> = std::result::Result<T, serde_json::Error>;

impl Artifact {
    pub fn new<T>(mime_type: &str, payload: T) -> Result<Self>
    where
        T: Serialize,
    {
        let payload = serde_json::to_value(&payload)?;
        Ok(Artifact {
            mime_type: mime_type.to_string(),
            payload: Payload(payload),
        })
    }

    pub fn with_value(mime_type: &str, payload: serde_json::Value) -> Result<Self> {
        Ok(Artifact {
            mime_type: mime_type.to_string(),
            payload: Payload(payload),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_inner<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let payload: T = serde_json::from_value(self.payload.0.clone())?;
        Ok(payload)
    }

    pub fn get_value(&self) -> &serde_json::Value {
        &self.payload.0
    }

    // get_mine_type_pair 获取翻译器翻译步骤的Query指令
    pub fn get_mine_type_pair(&self) -> String {
        if self.mime_type == MimeType::DeviceRunningConfig.to_string() {
            return MimeType::ConfigRunning.to_string();
        }
        if self.mime_type == MimeType::DeviceSn.to_string() {
            return MimeType::ConfigSn.to_string();
        }
        if self.mime_type == MimeType::DeviceVersion.to_string() {
            return MimeType::ConfigVersion.to_string();
        }
        if self.mime_type == MimeType::DeviceHa.to_string() {
            return MimeType::ConfigHa.to_string();
        }
        if self.mime_type == MimeType::DeviceVrrp.to_string() {
            return MimeType::ConfigVrrp.to_string();
        }
        if self.mime_type == MimeType::DeviceArp.to_string() {
            return MimeType::ConfigArp.to_string();
        }
        if self.mime_type == MimeType::DeviceMac.to_string() {
            return MimeType::ConfigMac.to_string();
        }
        if self.mime_type == MimeType::DeviceRoute.to_string() {
            return MimeType::ConfigRoute.to_string();
        }
        if self.mime_type == MimeType::DeviceRestart.to_string() {
            return MimeType::ConfigRestart.to_string();
        }
        if self.mime_type == MimeType::DeviceShutdown.to_string() {
            return MimeType::ConfigShutdown.to_string();
        }
        self.mime_type.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Payload(serde_json::Value);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct Device {
    pub id: String,
    #[serde(default)]
    pub hostname: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub manu: String,
    #[serde(default)]
    pub model: String,
    #[serde(default = "default_connect")]
    pub connections: Connections,
    #[serde(default)]
    pub vsys: Option<VirtualSystem>,
    #[serde(default)]
    pub root_path: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_connect() -> Connections {
    Connections::Tcp(TcpInfo::default())
}

fn default_enabled() -> bool {
    true
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct IpConnection {
    #[serde(rename = "src_zone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_zone: Option<Vec<String>>,
    #[serde(rename = "dst_zone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dest_zone: Option<Vec<String>>,
    #[serde(rename = "src_addr")]
    pub source_address_ip: Option<Vec<String>>,
    #[serde(rename = "dst_addr")]
    pub dest_address_ip: Option<Vec<String>>,
    #[serde(rename = "src_port")]
    pub service_src_port: Option<Vec<String>>,
    #[serde(rename = "dst_port")]
    pub service_dest_port: Option<Vec<String>>,
    pub protocol: Option<String>,
    #[serde(rename = "icmp_type")]
    pub icmp_type: Option<String>,
    #[serde(rename = "icmp_code")]
    pub icmp_code: Option<Vec<String>>,
    #[serde(rename = "ip_number")]
    pub ip_number: Option<Vec<String>>,
}

impl TargetIdentity for IpConnection {}

impl Device {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        hostname: &str,
        kind: &str,
        version: &str,
        manu: &str,
        model: &str,
        connections: Connections,
        vsys: Option<VirtualSystem>,
        root_path: &str,
        group_id: &str,
    ) -> Self {
        let group_id = if group_id.is_empty() {
            None
        } else {
            Some(group_id.to_string())
        };
        Device {
            id: id.to_string(),
            hostname: hostname.to_string(),
            kind: kind.to_string(),
            version: version.to_string(),
            manu: manu.to_string(),
            model: model.to_string(),
            connections,
            vsys,
            root_path: root_path.to_string(),
            group_id,
            enabled: true,
        }
    }

    pub fn get_timeout(&self) -> Option<u64> {
        match &self.connections {
            Connections::Tcp(info) => info.timeout,
            Connections::Http(info) => info.timeout,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum Connections {
    Tcp(TcpInfo),
    Http(HttpInfo),
    Passive,
}
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize, EnumString, ToString)]
pub enum HttpInfoProtocol {
    #[strum(to_string = "http")]
    #[serde(rename(deserialize = "http", deserialize = "Http", deserialize = "HTTP"))]
    Http,
    #[strum(to_string = "https")]
    #[serde(rename(deserialize = "https", deserialize = "Https", deserialize = "HTTPS"))]
    Https,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Hash)]
pub struct TcpInfo {
    pub protocol: String,
    pub address: IpFormat,
    pub username: NameFormat,
    pub password: PasswordFormat,
    pub passport: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Hash)]
pub struct HttpInfo {
    pub header: Option<String>,
    pub protocol: HttpInfoProtocol,
    pub url: String,
    pub username: Option<NameFormat>,
    pub password: Option<PasswordFormat>,
    pub cert: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Hash)]
pub struct VirtualSystem {
    pub name: String,
}

impl VirtualSystem {
    pub fn new(name: &str) -> Self {
        VirtualSystem {
            name: name.to_string(),
        }
    }
}

impl TargetIdentity for Device {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct Features(Vec<String>);

impl Features {
    pub fn new(array: &[String]) -> Self {
        Features(array.to_vec())
    }

    pub fn inner_first(&self) -> Option<&str> {
        self.0.first().map(|str| str.as_str())
    }

    pub fn inner(&self) -> &[String] {
        &self.0[..]
    }
}

impl TargetIdentity for Features {
    fn identity(&self) -> String {
        self.inner_first().unwrap_or_default().to_string()
    }
}

pub trait TargetIdentity {
    fn identity(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::ops::Deref;
    #[test]
    fn test_target() {
        let artifact = Target::Artifact(Artifact::new("test", 0).unwrap());
        assert_eq!(artifact.identity(), "artifact.test");

        let device = Target::Device(Device::new(
            "1",
            "localhost",
            "fw",
            "1.0",
            "lolo",
            "fw",
            Connections::Passive,
            Some(VirtualSystem::new("vsys")),
            "/",
            "",
        ));
        assert_eq!(device.identity(), "device");

        let vec = vec!["test".to_string()];
        let features = Target::Features(Features::new(&vec));
        assert_eq!(features.identity(), "features.test");

        let ip_connection = Target::IpConnection(IpConnection::default());
        assert_eq!(ip_connection.identity(), "ip_connection");

        let domain_name = Target::DomainName(String::from("http://www.baidu.com"));
        assert_eq!(domain_name.identity(), "domain_name");
    }

    #[test]
    fn test_artifact() {
        let artifact = Artifact::new("device/running_config", 0).unwrap();
        let mine_type_pair = artifact.get_mine_type_pair();
        assert_eq!(mine_type_pair, "config/running");

        let artifact = Artifact::new("cmd", 0).unwrap();
        let mine_type_pair = artifact.get_mine_type_pair();
        assert_eq!(mine_type_pair, "cmd");
    }

    #[test]
    fn test_replace_special() {
        let mtype = MimeType::DeviceRule;
        let mtype_sn = MimeType::DeviceSn;

        assert_eq!(mtype.need_replace_special_char(), false);
        assert_eq!(mtype_sn.need_replace_special_char(), true);
    }

    #[test]
    fn test_http_info_protocol() {
        //protocol Http
        let http_info = HttpInfo {
            protocol: HttpInfoProtocol::Http,
            url: "127.0.0.1".to_string(),
            ..Default::default()
        };
        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "http",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);

        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "Http",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);

        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "HTTP",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);
        //protocol Https
        let http_info = HttpInfo {
            protocol: HttpInfoProtocol::Https,
            url: "127.0.0.1".to_string(),
            ..Default::default()
        };
        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "https",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);

        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "Https",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);

        let deserialize = serde_json::from_str::<HttpInfo>(
            r#"{
            "protocol": "HTTPS",
            "url": "127.0.0.1"
            }"#,
        )
        .unwrap();
        assert_eq!(deserialize, http_info);
    }

    #[test]
    fn test_connection_info() {
        let tcp_info = json!({
            "protocol" :"tcp",
            "username": "zichenchen",
            "address": "10.48.84.80",
            "password": "testtest",
        });
        let obj = serde_json::from_value::<TcpInfo>(tcp_info).unwrap();
        let json = json!(obj);
        assert_eq!(
            json,
            json!({ "protocol": "tcp", "address": "10.48.84.80", "username": "zichenchen", "password": "testtest", "passport": null, "timeout": null })
        );
        assert_eq!(obj.password.deref(), "testtest");
        assert_eq!(format!("{:?}", &obj),"TcpInfo { protocol: \"tcp\", address: *.*.*.*, username: z*, password: ********, passport: None, timeout: None }");
        let obj_json = serde_json::to_value(obj).unwrap();
        assert_eq!(
            obj_json,
            json!({ "protocol": "tcp", "address": "10.48.84.80", "username": "zichenchen", "password": "testtest", "passport": null, "timeout": null })
        );
    }
}
