use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct R<T> {
    code: i32,
    msg: String,
    is_success: bool,
    data: Option<T>,
}

const MSG_SUCCESS: &str = "成功";

impl<T: Send + Sync> R<T> {
    pub fn data(data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: MSG_SUCCESS.to_owned(),
            is_success: true,
            data,
        }
    }

    pub fn success() -> Self {
        Self {
            code: 200,
            msg: MSG_SUCCESS.to_owned(),
            is_success: true,
            data: None,
        }
    }

    pub fn fail_data(msg: String, data: Option<T>) -> Self {
        Self {
            code: -1,
            msg,
            is_success: false,
            data,
        }
    }

    pub fn fail(msg: String) -> Self {
        Self {
            code: -1,
            msg,
            is_success: false,
            data: None,
        }
    }

    pub fn get_data(self: Self) -> Option<T> {
        self.data
    }

    pub fn get_is_success(self: Self) -> bool {
        self.is_success
    }
}
