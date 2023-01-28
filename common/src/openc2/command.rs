use abi_stable::StableAbi;
use chrono::Utc;
use enum_iterator::IntoEnumIterator;
use strum::*;

use crate::openc2::args::{OpenC2Args, ResponseRequested};
use crate::openc2::target::{Device, Target, TargetIdentity};
use crate::openc2::{OpenC2MsgType, TraceIdent};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    IntoEnumIterator,
    EnumString,
    ToString,
)]
#[serde(rename_all = "lowercase")]
pub enum OpenC2Action {
    //针对设备本身的操作,扫描检查,探活
    #[strum(to_string = "scan")]
    Scan,

    // 用于在设备未添加时使用设备登录信息探活
    #[strum(to_string = "pre_conn")]
    #[serde(rename = "pre_conn")]
    PreConn,

    #[strum(to_string = "locate")]
    Locate,

    //针对设备本身的操作，查询
    #[strum(to_string = "query")]
    Query,

    #[strum(to_string = "report")]
    Report,

    #[strum(to_string = "notify")]
    Notify,

    //针对设备本身的操作，断开
    #[strum(to_string = "deny")]
    Deny,

    #[strum(to_string = "contain")]
    Contain,

    #[strum(to_string = "allow")]
    Allow,

    //针对管理设备的操作，连接设备
    #[strum(to_string = "start")]
    Start,

    //针对管理设备的操作，下线设备
    #[strum(to_string = "stop")]
    Stop,

    #[strum(to_string = "restart")]
    Restart,

    #[strum(to_string = "pause")]
    Pause,

    #[strum(to_string = "resume")]
    Resume,

    //针对设备本身的操作，取消
    #[strum(to_string = "cancel")]
    Cancel,

    //针对设备本身的操作，设置
    #[strum(to_string = "set")]
    Set,

    //针对设备本身的操作，添加
    #[strum(to_string = "add")]
    Add,

    #[strum(to_string = "clear")]
    Clear,

    //1.针对管理设备的操作，更新设备  2.针对设备本身的操作，更新，如更新探针设备的数据抓包策略
    #[strum(to_string = "update")]
    Update,

    #[strum(to_string = "move")]
    Move,

    #[strum(to_string = "redirect")]
    Redirect,

    //针对管理设备的操作，添加设备
    #[strum(to_string = "create")]
    Create,

    //针对设备本身的操作，删除
    #[strum(to_string = "delete")]
    Delete,

    #[strum(to_string = "snapshot")]
    Snapshot,

    #[strum(to_string = "detonate")]
    Detonate,

    #[strum(to_string = "restore")]
    Restore,

    //针对设备本身的操作，保存
    #[strum(to_string = "save")]
    Save,

    #[strum(to_string = "throttle")]
    Throttle,

    #[strum(to_string = "delay")]
    Delay,

    #[strum(to_string = "substitute")]
    Substitute,

    #[strum(to_string = "copy")]
    Copy,

    #[strum(to_string = "sync")]
    Sync,

    #[strum(to_string = "investigate")]
    Investigate,

    #[strum(to_string = "mitigate")]
    Mitigate,

    #[strum(to_string = "remediate")]
    Remediate,

    #[strum(to_string = "shutdown")]
    Shutdown,
}

/// Command：
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OpenC2Command {
    #[serde(rename = "header")]
    header: OpenC2CmdHead,
    #[serde(rename = "command")]
    command: OpenC2CmdBody,
}

impl OpenC2Command {
    pub fn new(header: OpenC2CmdHead, command: OpenC2CmdBody) -> Self {
        OpenC2Command { header, command }
    }

    pub fn tuple_actuator_msg_id(&self) -> (String, String) {
        (
            self.header.request_id.to_string(),
            self.actuator_id_to_string(),
        )
    }

    pub fn device_msg_id(&self) -> String {
        format!(
            "[设备_消息:{}_{}]",
            &self.header.request_id,
            &self.actuator_id_to_string()
        )
    }
}

impl TraceIdent for OpenC2Command {
    fn trace_id(&self) -> &str {
        &self.header.request_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct OpenC2CmdHead {
    #[serde(rename = "request_id")]
    request_id: String,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "msg_type")]
    msg_type: OpenC2MsgType,

    #[serde(rename = "created")]
    created: i64,

    #[serde(rename = "sender")]
    sender: String,
}

impl OpenC2CmdHead {
    pub fn new<S: Into<String>>(request_id: S, sender: S) -> Self {
        OpenC2CmdHead {
            request_id: request_id.into(),
            version: "1.0".to_string(),
            msg_type: OpenC2MsgType::Request,
            created: Utc::now().timestamp_millis(),
            sender: sender.into(),
        }
    }

    pub fn get_request_id(&self) -> &String {
        &self.request_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OpenC2CmdBody {
    #[serde(rename = "action")]
    action: OpenC2Action,

    #[serde(rename = "target")]
    target: Target,

    #[serde(rename = "actuator")]
    actuator: Option<OpenC2Actuator>,

    #[serde(rename = "args")]
    args: Option<OpenC2Args>,
}

impl OpenC2CmdBody {
    pub fn new(
        action: OpenC2Action,
        target: Target,
        actuator: Option<OpenC2Actuator>,
        args: Option<OpenC2Args>,
    ) -> OpenC2CmdBody {
        OpenC2CmdBody {
            action,
            target,
            actuator,
            args,
        }
    }
}

#[repr(C)]
#[derive(AsRefStr, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, StableAbi)]
#[serde(rename_all = "lowercase")]
pub enum ActuatorType {
    Platform,
    Device,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OpenC2Actuator {
    #[serde(rename = "actuator_type")]
    actuator_type: ActuatorType,

    #[serde(rename = "actuator_id")]
    actuator_id: Vec<String>,
}

impl OpenC2Actuator {
    pub fn new(actuator_type: ActuatorType, actuator_id: Vec<String>) -> Self {
        OpenC2Actuator {
            actuator_type,
            actuator_id,
        }
    }

    pub fn actuator_id_to_string(&self) -> String {
        let mut id = "".into();
        if self.actuator_id.is_empty() {
            return id;
        }
        for i in &self.actuator_id {
            if id.is_empty() {
                id = i.clone();
                continue;
            }
            id = format!("{},{}", id, i)
        }
        id
    }
}

impl OpenC2Command {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn get_target(&self) -> &Target {
        &self.command.target
    }

    pub fn set_device_target(&mut self, device: Device) {
        self.command.target = Target::Device(device);
    }

    pub fn get_request_id(&self) -> &String {
        &self.header.request_id
    }

    pub fn get_created(&self) -> i64 {
        self.header.created
    }

    pub fn get_sender(&self) -> &String {
        &self.header.sender
    }

    pub fn get_action(&self) -> &OpenC2Action {
        &self.command.action
    }

    pub fn get_actuator(&self) -> &Option<OpenC2Actuator> {
        &self.command.actuator
    }

    pub fn set_actuator(&mut self, id: String, actuator_type: ActuatorType) {
        let actuator_id = vec![id];
        self.command.actuator = Some(OpenC2Actuator::new(actuator_type, actuator_id));
    }

    pub fn get_timeout(&self) -> Option<u64> {
        self.command
            .args
            .as_ref()
            .and_then(|args| args.get_timeout())
    }

    pub fn get_response_requested(&self) -> Option<&ResponseRequested> {
        self.command
            .args
            .as_ref()
            .map(|args| args.get_response_requested())
    }

    pub fn actuator_id_to_string(&self) -> String {
        if let Some(actuator) = &self.command.actuator {
            return actuator.actuator_id_to_string();
        }
        "".into()
    }

    pub fn is_none_actuator(&self) -> bool {
        self.command.actuator.is_none()
    }

    pub fn get_command_identity(&self) -> (OpenC2Action, String) {
        (self.command.action.clone(), self.command.target.identity())
    }
}
