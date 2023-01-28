use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use std::{collections::HashMap, fmt};
use thiserror::Error;

pub type RResult<T> = Result<T, RError>;

#[derive(Debug, Deserialize, Error, Clone)]
pub struct RError {
    pub status_code: u16,
    pub status_text: String,
    pub error_message: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum RErrorKind {
    FileNotFound,
    FileInvalid,

    InputInvalid,
    UnKnowError,
}

lazy_static! {
    pub static ref ERR_MAP: HashMap<RErrorKind, (u16, String)> = {
        let mut map = HashMap::new();
        map.insert(
            RErrorKind::FileNotFound,
            (1001, "file not found, 文件未发现".to_string()),
        );
        map.insert(
            RErrorKind::FileInvalid,
            (1002, "file is invalid, 文件无效".to_string()),
        );
        map.insert(
            RErrorKind::InputInvalid,
            (1002, "input not found, 输入无效".to_string()),
        );
        map.insert(
            RErrorKind::UnKnowError,
            (9999, "unknow, 未知错误".to_string()),
        );
        map
    };
}

impl RError {
    pub fn new(status: RErrorKind, error_message: String) -> RError {
        return if let Some((status_code, status_text)) = ERR_MAP.get(&status) {
            RError {
                status_code: *status_code,
                status_text: status_text.clone(),
                error_message,
            }
        } else {
            RError {
                status_code: 9999,
                status_text: "unknow".to_string(),
                error_message,
            }
        };
    }
}

impl From<anyhow::Error> for RError {
    fn from(error: anyhow::Error) -> Self {
        Self::new(RErrorKind::InputInvalid, error.to_string())
    }
}

impl From<IoError> for RError {
    fn from(e: IoError) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => Self::new(RErrorKind::FileNotFound, e.to_string()),
            _ => Self::new(RErrorKind::UnKnowError, e.to_string()),
        }
    }
}

impl From<SerdeError> for RError {
    fn from(error: SerdeError) -> Self {
        Self::new(RErrorKind::InputInvalid, error.to_string())
    }
}

impl fmt::Display for RError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}
