use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// NewOrderCross (MsgType s) - Submit a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderCross {
    pub cross_id: String,                              // Tag 548
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub ord_type: OrdType,                             // Tag 40
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    // For now we store them in the RawFixMessage groups
    pub price: Option<f64>,                            // Tag 44
    pub text: Option<String>,                          // Tag 58
}

impl NewOrderCross {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "s".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());
        raw.set_field(40, self.ord_type.to_fix().to_string());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(price) = self.price {
            raw.set_field(44, price.to_string());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: String::new(),
                error: "Empty ord type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid ord type".to_string(),
            })?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let price = raw.get_field(44).and_then(|s| s.parse().ok());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(NewOrderCross {
            cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            ord_type,
            transact_time,
            price,
            text,
        })
    }
}

/// CrossOrderCancelRequest (MsgType u) - Cancel a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrderCancelRequest {
    pub cross_id: String,                              // Tag 548
    pub orig_cross_id: String,                         // Tag 551
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    pub order_id: Option<String>,                      // Tag 37
    pub text: Option<String>,                          // Tag 58
}

impl CrossOrderCancelRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "u".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(551, self.orig_cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(ref order_id) = self.order_id {
            raw.set_field(37, order_id.clone());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let orig_cross_id = raw.get_field(551)
            .ok_or(FixParseError::MissingRequiredField(551))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let order_id = raw.get_field(37).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(CrossOrderCancelRequest {
            cross_id,
            orig_cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            transact_time,
            order_id,
            text,
        })
    }
}

/// CrossOrderCancelReplaceRequest (MsgType t) - Modify a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrderCancelReplaceRequest {
    pub cross_id: String,                              // Tag 548
    pub orig_cross_id: String,                         // Tag 551
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub ord_type: OrdType,                             // Tag 40
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    pub price: Option<f64>,                            // Tag 44
    pub text: Option<String>,                          // Tag 58
}

impl CrossOrderCancelReplaceRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "t".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(551, self.orig_cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());
        raw.set_field(40, self.ord_type.to_fix().to_string());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(price) = self.price {
            raw.set_field(44, price.to_string());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let orig_cross_id = raw.get_field(551)
            .ok_or(FixParseError::MissingRequiredField(551))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: String::new(),
                error: "Empty ord type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid ord type".to_string(),
            })?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let price = raw.get_field(44).and_then(|s| s.parse().ok());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(CrossOrderCancelReplaceRequest {
            cross_id,
            orig_cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            ord_type,
            transact_time,
            price,
            text,
        })
    }
}
