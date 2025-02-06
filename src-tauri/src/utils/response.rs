use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct R<T> {
    code: i32,
    msg: String,
    is_success: Option<bool>,
    data: Option<T>,
}

const MSG_SUCCESS: &str = "成功";

impl<T: Send + Sync> R<T> {
    pub fn data(data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: MSG_SUCCESS.to_owned(),
            is_success: Some(true),
            data,
        }
    }

    pub fn success() -> Self {
        Self {
            code: 200,
            msg: MSG_SUCCESS.to_owned(),
            is_success: Some(true),
            data: None,
        }
    }

    pub fn fail_data(msg: String, data: Option<T>) -> Self {
        Self {
            code: -1,
            msg,
            is_success: Some(false),
            data,
        }
    }

    pub fn fail(msg: String) -> Self {
        Self {
            code: -1,
            msg,
            is_success: Some(false),
            data: None,
        }
    }

    pub fn fail_code(code: i32, msg: &str) -> Self {
        Self {
            code: code,
            msg: msg.to_string(),
            is_success: Some(false),
            data: None,
        }
    }

    pub fn get_data(self: Self) -> Option<T> {
        self.data
    }

    pub fn get_is_success(self: Self) -> bool {
        if let Some(b) = self.is_success {
            return b;
        }
        false
    }
}
