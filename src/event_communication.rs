//! Event Communication message types
//!
//! This module implements FIX 5.0 SP2 Event Communication messages (Pre-Trade category),
//! which allow market participants to send emails and news items through the FIX protocol.
//!
//! ## Message Types
//! - Email (C): Send general email messages
//! - News (B): Disseminate news to market participants
//!
//! ## Use Cases
//! - Email communication between trading parties
//! - Market news dissemination
//! - Administrative notifications
//! - Corporate announcements

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{EmailType, NewsRefType, NewsCategory};
use serde::{Deserialize, Serialize};

// ============================================================================
// Email (MsgType = C)
// ============================================================================

/// Email (C) - Send general email messages through FIX
///
/// Used for general email communication between trading parties. Supports
/// routing to multiple recipients and threaded conversations via reply chains.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Email {
    /// EmailThreadID (Tag 164) - Required - Unique identifier for email thread
    pub email_thread_id: String,

    /// EmailType (Tag 94) - Required - Type of email (New, Reply, AdminReply)
    pub email_type: EmailType,

    /// Subject (Tag 147) - Required - Email subject line
    pub subject: String,

    /// OrigTime (Tag 42) - Conditionally required - Time email was originated
    pub orig_time: Option<String>,

    /// OrderID (Tag 37) - Order to which email relates
    pub order_id: Option<String>,

    /// ClOrdID (Tag 11) - Client order ID to which email relates
    pub cl_ord_id: Option<String>,

    /// EmailMsgID (Tag 356) - Unique identifier for this email message
    pub email_msg_id: Option<String>,

    /// RawDataLength (Tag 95) - Length of raw data
    pub raw_data_length: Option<u32>,

    /// RawData (Tag 96) - Unformatted raw data (e.g., attachments)
    pub raw_data: Option<Vec<u8>>,

    // Note: NoRoutingIDs(215) and NoLinesOfText(33) repeating groups
    // are accessed via RawFixMessage.groups
}

impl Email {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(Email {
            email_thread_id: raw.get_field(164)
                .ok_or_else(|| FixParseError::MissingRequiredField(164))?
                .to_string(),
            email_type: raw.get_field(94)
                .and_then(|s| s.chars().next())
                .and_then(EmailType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(94))?,
            subject: raw.get_field(147)
                .ok_or_else(|| FixParseError::MissingRequiredField(147))?
                .to_string(),
            orig_time: raw.get_field(42).map(|s| s.to_string()),
            order_id: raw.get_field(37).map(|s| s.to_string()),
            cl_ord_id: raw.get_field(11).map(|s| s.to_string()),
            email_msg_id: raw.get_field(356).map(|s| s.to_string()),
            raw_data_length: raw.get_field(95).and_then(|s| s.parse().ok()),
            raw_data: None, // Binary data handling TBD
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "C".to_string());

        // Required fields
        msg.set_field(164, self.email_thread_id.clone());
        msg.set_field(94, self.email_type.to_fix().to_string());
        msg.set_field(147, self.subject.clone());

        // Optional fields
        if let Some(ref time) = self.orig_time {
            msg.set_field(42, time.clone());
        }
        if let Some(ref order_id) = self.order_id {
            msg.set_field(37, order_id.clone());
        }
        if let Some(ref cl_ord_id) = self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(ref email_msg_id) = self.email_msg_id {
            msg.set_field(356, email_msg_id.clone());
        }
        if let Some(len) = self.raw_data_length {
            msg.set_field(95, len.to_string());
        }

        msg
    }
}

// ============================================================================
// News (MsgType = B)
// ============================================================================

/// News (B) - Disseminate news to market participants
///
/// Used to broadcast news items to market participants. Supports categorization,
/// urgency levels, and news reference chains for updates and corrections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct News {
    /// OrigTime (Tag 42) - Time news was originated
    pub orig_time: Option<String>,

    /// Urgency (Tag 61) - Urgency of the news (0=Normal, 1=Flash, 2=Background)
    pub urgency: Option<char>,

    /// Headline (Tag 148) - Required - News headline
    pub headline: String,

    /// NewsID (Tag 1472) - Unique identifier for news item
    pub news_id: Option<String>,

    /// NewsCategory (Tag 1473) - Category of news
    pub news_category: Option<NewsCategory>,

    /// LanguageCode (Tag 1474) - ISO 639-1 language code
    pub language_code: Option<String>,

    /// MarketID (Tag 1301) - Market to which news relates
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Market segment to which news relates
    pub market_segment_id: Option<String>,

    /// URLLink (Tag 149) - URL for additional information
    pub url_link: Option<String>,

    /// RawDataLength (Tag 95) - Length of raw data
    pub raw_data_length: Option<u32>,

    /// RawData (Tag 96) - Unformatted raw data
    pub raw_data: Option<Vec<u8>>,

    // Note: NoNewsRefIDs(1476), NoRoutingIDs(215), NoLinesOfText(33),
    // NoRelatedSym(146) repeating groups are accessed via RawFixMessage.groups
}

impl News {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(News {
            orig_time: raw.get_field(42).map(|s| s.to_string()),
            urgency: raw.get_field(61).and_then(|s| s.chars().next()),
            headline: raw.get_field(148)
                .ok_or_else(|| FixParseError::MissingRequiredField(148))?
                .to_string(),
            news_id: raw.get_field(1472).map(|s| s.to_string()),
            news_category: raw.get_field(1473)
                .and_then(NewsCategory::from_fix),
            language_code: raw.get_field(1474).map(|s| s.to_string()),
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            url_link: raw.get_field(149).map(|s| s.to_string()),
            raw_data_length: raw.get_field(95).and_then(|s| s.parse().ok()),
            raw_data: None, // Binary data handling TBD
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "B".to_string());

        // Required fields
        msg.set_field(148, self.headline.clone());

        // Optional fields
        if let Some(ref time) = self.orig_time {
            msg.set_field(42, time.clone());
        }
        if let Some(urgency) = self.urgency {
            msg.set_field(61, urgency.to_string());
        }
        if let Some(ref news_id) = self.news_id {
            msg.set_field(1472, news_id.clone());
        }
        if let Some(category) = self.news_category {
            msg.set_field(1473, category.to_fix().to_string());
        }
        if let Some(ref lang) = self.language_code {
            msg.set_field(1474, lang.clone());
        }
        if let Some(ref market_id) = self.market_id {
            msg.set_field(1301, market_id.clone());
        }
        if let Some(ref seg_id) = self.market_segment_id {
            msg.set_field(1300, seg_id.clone());
        }
        if let Some(ref url) = self.url_link {
            msg.set_field(149, url.clone());
        }
        if let Some(len) = self.raw_data_length {
            msg.set_field(95, len.to_string());
        }

        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_round_trip() {
        let email = Email {
            email_thread_id: "THREAD123".to_string(),
            email_type: EmailType::New,
            subject: "Test Subject".to_string(),
            orig_time: Some("20250109-10:30:00".to_string()),
            order_id: Some("ORD456".to_string()),
            cl_ord_id: None,
            email_msg_id: Some("MSG789".to_string()),
            raw_data_length: None,
            raw_data: None,
        };

        let raw = email.to_raw();
        let parsed = Email::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.email_thread_id, "THREAD123");
        assert_eq!(parsed.email_type, EmailType::New);
        assert_eq!(parsed.subject, "Test Subject");
        assert_eq!(parsed.orig_time, Some("20250109-10:30:00".to_string()));
        assert_eq!(parsed.order_id, Some("ORD456".to_string()));
        assert_eq!(parsed.email_msg_id, Some("MSG789".to_string()));
    }

    #[test]
    fn test_email_reply_type() {
        let email = Email {
            email_thread_id: "THREAD456".to_string(),
            email_type: EmailType::Reply,
            subject: "Re: Test Subject".to_string(),
            orig_time: None,
            order_id: None,
            cl_ord_id: None,
            email_msg_id: None,
            raw_data_length: None,
            raw_data: None,
        };

        let raw = email.to_raw();
        let parsed = Email::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.email_type, EmailType::Reply);
        assert_eq!(parsed.subject, "Re: Test Subject");
    }

    #[test]
    fn test_news_round_trip() {
        let news = News {
            orig_time: Some("20250109-11:00:00".to_string()),
            urgency: Some('1'), // Flash
            headline: "Important Market News".to_string(),
            news_id: Some("NEWS001".to_string()),
            news_category: Some(NewsCategory::MarketplaceNews),
            language_code: Some("en".to_string()),
            market_id: Some("XNAS".to_string()),
            market_segment_id: None,
            url_link: Some("https://example.com/news/001".to_string()),
            raw_data_length: None,
            raw_data: None,
        };

        let raw = news.to_raw();
        let parsed = News::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.headline, "Important Market News");
        assert_eq!(parsed.urgency, Some('1'));
        assert_eq!(parsed.news_id, Some("NEWS001".to_string()));
        assert_eq!(parsed.news_category, Some(NewsCategory::MarketplaceNews));
        assert_eq!(parsed.language_code, Some("en".to_string()));
        assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    }

    #[test]
    fn test_news_minimal() {
        let news = News {
            orig_time: None,
            urgency: None,
            headline: "Minimal News".to_string(),
            news_id: None,
            news_category: None,
            language_code: None,
            market_id: None,
            market_segment_id: None,
            url_link: None,
            raw_data_length: None,
            raw_data: None,
        };

        let raw = news.to_raw();
        let parsed = News::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.headline, "Minimal News");
        assert_eq!(parsed.urgency, None);
        assert_eq!(parsed.news_category, None);
    }

    #[test]
    fn test_news_categories() {
        let categories = vec![
            NewsCategory::CompanyNews,
            NewsCategory::MarketplaceNews,
            NewsCategory::FinancialNews,
            NewsCategory::TechnicalNews,
            NewsCategory::OtherNews,
        ];

        for category in categories {
            let news = News {
                orig_time: None,
                urgency: None,
                headline: "Category Test".to_string(),
                news_id: None,
                news_category: Some(category),
                language_code: None,
                market_id: None,
                market_segment_id: None,
                url_link: None,
                raw_data_length: None,
                raw_data: None,
            };

            let raw = news.to_raw();
            let parsed = News::from_raw(&raw).expect("Should parse");

            assert_eq!(parsed.news_category, Some(category));
        }
    }
}
