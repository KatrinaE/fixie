use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{UserRequestType, UserStatus};
use serde::{Deserialize, Serialize};

/// UserRequest (MsgType BE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub user_request_id: String,              // Required - Tag 923
    pub user_request_type: UserRequestType,   // Required - Tag 924
    pub username: String,                     // Required - Tag 553
    pub password: Option<String>,             // Tag 554
    pub new_password: Option<String>,         // Tag 925
}

impl UserRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BE".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(923, self.user_request_id.clone());
        msg.set_field(924, self.user_request_type.to_fix().to_string());
        msg.set_field(553, self.username.clone());
        if let Some(password) = &self.password {
            msg.set_field(554, password.clone());
        }
        if let Some(new_password) = &self.new_password {
            msg.set_field(925, new_password.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let user_request_id = raw.get_field(923)
            .ok_or(FixParseError::MissingRequiredField(923))?
            .to_string();

        let user_request_type_char = raw.get_field(924)
            .ok_or(FixParseError::MissingRequiredField(924))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 924,
                value: "".to_string(),
                error: "Empty user request type".to_string(),
            })?;
        let user_request_type = UserRequestType::from_fix(user_request_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 924,
                value: user_request_type_char.to_string(),
                error: "Invalid user request type".to_string(),
            })?;

        let username = raw.get_field(553)
            .ok_or(FixParseError::MissingRequiredField(553))?
            .to_string();

        let password = raw.get_field(554).map(|s| s.to_string());
        let new_password = raw.get_field(925).map(|s| s.to_string());

        Ok(UserRequest {
            user_request_id,
            user_request_type,
            username,
            password,
            new_password,
        })
    }
}

/// UserResponse (MsgType BF)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_request_id: String,              // Required - Tag 923
    pub username: String,                     // Required - Tag 553
    pub user_status: Option<UserStatus>,      // Tag 926
    pub user_status_text: Option<String>,     // Tag 927
}

impl UserResponse {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BF".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(923, self.user_request_id.clone());
        msg.set_field(553, self.username.clone());
        if let Some(status) = self.user_status {
            msg.set_field(926, status.to_fix().to_string());
        }
        if let Some(text) = &self.user_status_text {
            msg.set_field(927, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let user_request_id = raw.get_field(923)
            .ok_or(FixParseError::MissingRequiredField(923))?
            .to_string();

        let username = raw.get_field(553)
            .ok_or(FixParseError::MissingRequiredField(553))?
            .to_string();

        let user_status = raw.get_field(926)
            .and_then(|s| s.chars().next())
            .and_then(UserStatus::from_fix);

        let user_status_text = raw.get_field(927).map(|s| s.to_string());

        Ok(UserResponse {
            user_request_id,
            username,
            user_status,
            user_status_text,
        })
    }
}

/// UserNotification (MsgType CB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNotification {
    pub username: String,                     // Required - Tag 553
    pub user_status: Option<UserStatus>,      // Tag 926
    pub user_status_text: Option<String>,     // Tag 927
}

impl UserNotification {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "CB".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(553, self.username.clone());
        if let Some(status) = self.user_status {
            msg.set_field(926, status.to_fix().to_string());
        }
        if let Some(text) = &self.user_status_text {
            msg.set_field(927, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let username = raw.get_field(553)
            .ok_or(FixParseError::MissingRequiredField(553))?
            .to_string();

        let user_status = raw.get_field(926)
            .and_then(|s| s.chars().next())
            .and_then(UserStatus::from_fix);

        let user_status_text = raw.get_field(927).map(|s| s.to_string());

        Ok(UserNotification {
            username,
            user_status,
            user_status_text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_request_round_trip() {
        let request = UserRequest {
            user_request_id: "UR001".to_string(),
            user_request_type: UserRequestType::LogOnUser,
            username: "trader1".to_string(),
            password: Some("pass123".to_string()),
            new_password: None,
        };

        let raw = request.to_raw();
        assert_eq!(raw.get_field(35), Some("BE"));
        assert_eq!(raw.get_field(923), Some("UR001"));
        assert_eq!(raw.get_field(553), Some("trader1"));

        let parsed = UserRequest::from_raw(&raw).expect("Failed to parse");
        assert_eq!(parsed.user_request_id, "UR001");
        assert_eq!(parsed.username, "trader1");
    }

    #[test]
    fn test_user_response_round_trip() {
        let response = UserResponse {
            user_request_id: "UR001".to_string(),
            username: "trader1".to_string(),
            user_status: Some(UserStatus::LoggedIn),
            user_status_text: Some("Login successful".to_string()),
        };

        let raw = response.to_raw();
        assert_eq!(raw.get_field(35), Some("BF"));
        assert_eq!(raw.get_field(926), Some("1"));

        let parsed = UserResponse::from_raw(&raw).expect("Failed to parse");
        assert_eq!(parsed.user_status, Some(UserStatus::LoggedIn));
    }

    #[test]
    fn test_user_notification_round_trip() {
        let notification = UserNotification {
            username: "trader1".to_string(),
            user_status: Some(UserStatus::PasswordChanged),
            user_status_text: Some("Password updated".to_string()),
        };

        let raw = notification.to_raw();
        assert_eq!(raw.get_field(35), Some("CB"));
        assert_eq!(raw.get_field(553), Some("trader1"));

        let parsed = UserNotification::from_raw(&raw).expect("Failed to parse");
        assert_eq!(parsed.user_status, Some(UserStatus::PasswordChanged));
    }
}
