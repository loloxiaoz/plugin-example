use crate::openc2::{OpenC2MsgType, Push, TraceIdent};
use chrono::Utc;
use serde_json::Value;

/// OpenC2Response
/// ```
/// use hyper_define::openc2::response::OpenC2Response;
/// use serde_derive::{Serialize,Deserialize};
///
///#[derive(Debug, Deserialize, Serialize, PartialEq)]
///         struct OpenC2RespResult {
///            header: BizDataHead,
///             body: Option<serde_json::Value>,
///         }
///
///         #[derive(Debug, Deserialize, Serialize, PartialEq)]
///         struct BizDataHead {
///             #[serde(rename = "type")]
///             pub result_type: String,
///
///             #[serde(rename = "device_id")]
///             pub device_id: String,
///
///            #[serde(rename = "msg_id")]
///             pub msg_id: String,
///         }
///
///         let head = BizDataHead {
///             result_type: "version_check".to_string(),
///             device_id: "202012011101".to_string(),
///             msg_id: "AX2020120111CB".to_string(),
///         };
///
///         #[derive(Debug, Deserialize, Serialize, PartialEq)]
///         struct ResultBody {
///             complete_status: bool,
///         }
///
///         let result_body = ResultBody {
///             complete_status: false,
///         };
///
///         let resp_result = OpenC2RespResult {
///             header: head,
///             body: Some(serde_json::to_value(result_body).unwrap()),
///         };
///
///         let value = serde_json::to_value(resp_result).unwrap();
///
///         let mut response = OpenC2Response::new(vec![value], "202012011101", "connector");
///         response.set_created(0);
///         let expect = r#"{"results":[{"body":{"complete_status":false},"header":{"device_id":"202012011101","msg_id":"AX2020120111CB","type":"version_check"}}],"msg_type":"response","request_id":"202012011101","created":0,"sender":"connector","status":200,"status_text":"Ok","desc":""}"#;
///        assert_eq!(expect, serde_json::to_string(&response).unwrap());
///
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct OpenC2Response {
    #[serde(rename = "results")]
    results: Vec<serde_json::Value>,

    #[serde(rename = "msg_type")]
    msg_type: OpenC2MsgType,

    #[serde(rename = "request_id")]
    request_id: String,

    #[serde(rename = "created")]
    created: i64,

    #[serde(rename = "sender")]
    sender: String,

    #[serde(rename = "status")]
    status: u16,

    #[serde(rename = "status_text")]
    status_text: String,

    #[serde(rename = "desc")]
    desc: String,
}

#[allow(clippy::wrong_self_convention)]
pub enum OpenC2RespStatus {
    OK,
}

impl From<OpenC2RespStatus> for (u16, String) {
    fn from(status: OpenC2RespStatus) -> Self {
        match status {
            OpenC2RespStatus::OK => (200, "Ok".into()),
        }
    }
}

impl OpenC2Response {
    pub fn new<S: Into<String>>(results: Vec<serde_json::Value>, request_id: S, sender: S) -> Self {
        let mut response = Self::default(request_id, sender);
        response.results = results;
        response
    }

    pub fn default<S: Into<String>>(request_id: S, sender: S) -> Self {
        let (status, status_text) = OpenC2RespStatus::OK.into();
        OpenC2Response {
            results: vec![],
            msg_type: OpenC2MsgType::Response,
            request_id: request_id.into(),
            created: Utc::now().timestamp_millis(),
            sender: sender.into(),
            status,
            status_text,
            desc: String::new(),
        }
    }

    pub fn new_status<S: Into<String>>(
        result: Vec<serde_json::Value>,
        request_id: S,
        sender: S,
        status: OpenC2RespStatus,
        desc: S,
    ) -> Self {
        let (status, status_text) = status.into();
        OpenC2Response {
            results: result,
            msg_type: OpenC2MsgType::Response,
            request_id: request_id.into(),
            created: Utc::now().timestamp_millis(),
            sender: sender.into(),
            status,
            status_text,
            desc: desc.into(),
        }
    }

    pub fn set_status(&mut self, status: u16, status_text: &str, desc: &str) {
        self.status = status;
        self.status_text = status_text.to_string();
        self.desc = desc.to_string();
    }

    pub fn set_status_text(&mut self, status_text: &str) {
        self.status_text = status_text.to_string();
    }

    pub fn set_created(&mut self, created: i64) {
        self.created = created
    }

    pub fn get_results(&self) -> &Vec<serde_json::Value> {
        &self.results
    }

    pub fn get_request_id(&self) -> &String {
        &self.request_id
    }

    pub fn get_status(&self) -> u16 {
        self.status
    }
}

impl TraceIdent for OpenC2Response {
    fn trace_id(&self) -> &str {
        &self.request_id
    }
}

impl Push<serde_json::Value> for OpenC2Response {
    fn push(&mut self, v: Value) {
        self.results.push(v)
    }
}

impl Push<&serde_json::Value> for OpenC2Response {
    fn push(&mut self, v: &Value) {
        self.results.push(v.clone())
    }
}

impl Push<Vec<serde_json::Value>> for OpenC2Response {
    fn push(&mut self, v: Vec<Value>) {
        self.results.extend(v)
    }
}

impl Push<&Vec<serde_json::Value>> for OpenC2Response {
    fn push(&mut self, value: &Vec<Value>) {
        for v in value {
            self.results.push(v.clone())
        }
    }
}

// 探针批量任务处理报错时，使用此结构与探针请求结果一致
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct BatchResultInner {
    pub status_code: i32,

    pub status_desc: String,

    // 保留字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time;

    #[test]
    fn test_response() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct OpenC2RespResult {
            header: BizDataHead,
            body: Option<serde_json::Value>,
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct BizDataHead {
            #[serde(rename = "type")]
            pub result_type: String,

            #[serde(rename = "device_id")]
            pub device_id: String,

            #[serde(rename = "msg_id")]
            pub msg_id: String,
        }

        let head = BizDataHead {
            result_type: "version_check".to_string(),
            device_id: "202012011101".to_string(),
            msg_id: "AX2020120111CB".to_string(),
        };

        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct ResultBody {
            complete_status: bool,
        }

        let result_body = ResultBody {
            complete_status: false,
        };

        let resp_result = OpenC2RespResult {
            header: head,
            body: Some(serde_json::to_value(result_body).unwrap()),
        };

        let value = serde_json::to_value(resp_result).unwrap();

        let mut response = OpenC2Response::new(vec![value], "202012011101", "connector");
        response.created = 0;
        let expect = r#"{"results":[{"body":{"complete_status":false},"header":{"device_id":"202012011101","msg_id":"AX2020120111CB","type":"version_check"}}],"msg_type":"response","request_id":"202012011101","created":0,"sender":"connector","status":200,"status_text":"Ok","desc":""}"#;
        assert_eq!(expect, serde_json::to_string(&response).unwrap());
    }

    #[test]
    fn test_response_created() {
        let now = Utc::now().timestamp_millis();
        std::thread::sleep(time::Duration::from_millis(1));
        let response = OpenC2Response::default("202012011101", "connector");
        assert_ne!(now, response.created);
    }
}
