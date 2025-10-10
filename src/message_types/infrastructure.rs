use crate::parser::RawFixMessage;
use serde::{Deserialize, Serialize};

/// FIX Session Messages

/// Logon (MsgType A)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logon {
    pub encrypt_method: i32,  // Tag 98: 0 = None
    pub heart_bt_int: i32,    // Tag 108: Heartbeat interval in seconds
}

/// Logout (MsgType 5)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logout {
    pub text: Option<String>, // Tag 58: Optional logout reason
}

/// Heartbeat (MsgType 0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub test_req_id: Option<String>, // Tag 112: Required if responding to TestRequest
}
