use fixie::*;

#[test]
fn test_email_round_trip() {
    let email = Email {
        email_thread_id: "THREAD123".to_string(),
        email_type: EmailType::New,
        subject: "Important Trade Information".to_string(),
        orig_time: Some("20250110-09:30:00".to_string()),
        order_id: Some("ORD12345".to_string()),
        cl_ord_id: Some("CLI98765".to_string()),
        email_msg_id: Some("EMAIL001".to_string()),
        raw_data_length: None,
        raw_data: None,
    };

    let raw = email.to_raw();
    assert_eq!(raw.get_field(35), Some("C"));
    assert_eq!(raw.get_field(164), Some("THREAD123"));
    assert_eq!(raw.get_field(94), Some("0"));
    assert_eq!(raw.get_field(147), Some("Important Trade Information"));
    assert_eq!(raw.get_field(42), Some("20250110-09:30:00"));
    assert_eq!(raw.get_field(37), Some("ORD12345"));
    assert_eq!(raw.get_field(11), Some("CLI98765"));

    let parsed = Email::from_raw(&raw).expect("Failed to parse Email");
    assert_eq!(parsed.email_thread_id, "THREAD123");
    assert_eq!(parsed.email_type, EmailType::New);
    assert_eq!(parsed.subject, "Important Trade Information");
    assert_eq!(parsed.orig_time, Some("20250110-09:30:00".to_string()));
    assert_eq!(parsed.order_id, Some("ORD12345".to_string()));
    assert_eq!(parsed.cl_ord_id, Some("CLI98765".to_string()));
    assert_eq!(parsed.email_msg_id, Some("EMAIL001".to_string()));
}

#[test]
fn test_email_reply_type() {
    let email = Email {
        email_thread_id: "THREAD456".to_string(),
        email_type: EmailType::Reply,
        subject: "Re: Order Confirmation".to_string(),
        orig_time: Some("20250110-10:15:00".to_string()),
        order_id: None,
        cl_ord_id: None,
        email_msg_id: Some("EMAIL002".to_string()),
        raw_data_length: None,
        raw_data: None,
    };

    let raw = email.to_raw();
    assert_eq!(raw.get_field(35), Some("C"));
    assert_eq!(raw.get_field(94), Some("1"));
    assert_eq!(raw.get_field(147), Some("Re: Order Confirmation"));

    let parsed = Email::from_raw(&raw).expect("Failed to parse Email");
    assert_eq!(parsed.email_type, EmailType::Reply);
    assert_eq!(parsed.subject, "Re: Order Confirmation");
}

#[test]
fn test_email_admin_reply() {
    let email = Email {
        email_thread_id: "THREAD789".to_string(),
        email_type: EmailType::AdminReply,
        subject: "Administrative Response".to_string(),
        orig_time: None,
        order_id: None,
        cl_ord_id: None,
        email_msg_id: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = email.to_raw();
    assert_eq!(raw.get_field(94), Some("2"));

    let parsed = Email::from_raw(&raw).expect("Failed to parse Email");
    assert_eq!(parsed.email_type, EmailType::AdminReply);
}

#[test]
fn test_email_minimal() {
    let email = Email {
        email_thread_id: "THREAD_MIN".to_string(),
        email_type: EmailType::New,
        subject: "Minimal Email".to_string(),
        orig_time: None,
        order_id: None,
        cl_ord_id: None,
        email_msg_id: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = email.to_raw();
    let parsed = Email::from_raw(&raw).expect("Failed to parse Email");
    assert_eq!(parsed.email_thread_id, "THREAD_MIN");
    assert_eq!(parsed.subject, "Minimal Email");
    assert_eq!(parsed.orig_time, None);
    assert_eq!(parsed.order_id, None);
}

#[test]
fn test_news_round_trip() {
    let news = News {
        orig_time: Some("20250110-11:00:00".to_string()),
        urgency: Some('1'), // Flash
        headline: "BREAKING: Market Volatility Alert".to_string(),
        news_id: Some("NEWS12345".to_string()),
        news_category: Some(NewsCategory::MarketplaceNews),
        language_code: Some("en".to_string()),
        market_id: Some("XNAS".to_string()),
        market_segment_id: Some("EQTY".to_string()),
        url_link: Some("https://example.com/news/12345".to_string()),
        raw_data_length: None,
        raw_data: None,
    };

    let raw = news.to_raw();
    assert_eq!(raw.get_field(35), Some("B"));
    assert_eq!(raw.get_field(148), Some("BREAKING: Market Volatility Alert"));
    assert_eq!(raw.get_field(42), Some("20250110-11:00:00"));
    assert_eq!(raw.get_field(61), Some("1"));
    assert_eq!(raw.get_field(1472), Some("NEWS12345"));
    assert_eq!(raw.get_field(1473), Some("1"));
    assert_eq!(raw.get_field(1474), Some("en"));
    assert_eq!(raw.get_field(1301), Some("XNAS"));
    assert_eq!(raw.get_field(1300), Some("EQTY"));
    assert_eq!(raw.get_field(149), Some("https://example.com/news/12345"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.headline, "BREAKING: Market Volatility Alert");
    assert_eq!(parsed.orig_time, Some("20250110-11:00:00".to_string()));
    assert_eq!(parsed.urgency, Some('1'));
    assert_eq!(parsed.news_id, Some("NEWS12345".to_string()));
    assert_eq!(parsed.news_category, Some(NewsCategory::MarketplaceNews));
    assert_eq!(parsed.language_code, Some("en".to_string()));
    assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    assert_eq!(parsed.market_segment_id, Some("EQTY".to_string()));
    assert_eq!(parsed.url_link, Some("https://example.com/news/12345".to_string()));
}

#[test]
fn test_news_minimal() {
    let news = News {
        orig_time: None,
        urgency: None,
        headline: "Simple News Item".to_string(),
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
    assert_eq!(raw.get_field(35), Some("B"));
    assert_eq!(raw.get_field(148), Some("Simple News Item"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.headline, "Simple News Item");
    assert_eq!(parsed.urgency, None);
    assert_eq!(parsed.news_category, None);
}

#[test]
fn test_news_company_category() {
    let news = News {
        orig_time: None,
        urgency: Some('0'), // Normal
        headline: "Company Earnings Report".to_string(),
        news_id: Some("NEWS_COMP".to_string()),
        news_category: Some(NewsCategory::CompanyNews),
        language_code: Some("en".to_string()),
        market_id: None,
        market_segment_id: None,
        url_link: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = news.to_raw();
    assert_eq!(raw.get_field(1473), Some("0"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.news_category, Some(NewsCategory::CompanyNews));
    assert_eq!(parsed.urgency, Some('0'));
}

#[test]
fn test_news_financial_category() {
    let news = News {
        orig_time: None,
        urgency: Some('2'), // Background
        headline: "Federal Reserve Policy Update".to_string(),
        news_id: None,
        news_category: Some(NewsCategory::FinancialNews),
        language_code: None,
        market_id: None,
        market_segment_id: None,
        url_link: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = news.to_raw();
    assert_eq!(raw.get_field(1473), Some("2"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.news_category, Some(NewsCategory::FinancialNews));
}

#[test]
fn test_news_technical_category() {
    let news = News {
        orig_time: None,
        urgency: None,
        headline: "Trading Platform Maintenance".to_string(),
        news_id: None,
        news_category: Some(NewsCategory::TechnicalNews),
        language_code: None,
        market_id: None,
        market_segment_id: None,
        url_link: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = news.to_raw();
    assert_eq!(raw.get_field(1473), Some("3"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.news_category, Some(NewsCategory::TechnicalNews));
}

#[test]
fn test_news_other_category() {
    let news = News {
        orig_time: None,
        urgency: None,
        headline: "Miscellaneous Announcement".to_string(),
        news_id: None,
        news_category: Some(NewsCategory::OtherNews),
        language_code: None,
        market_id: None,
        market_segment_id: None,
        url_link: None,
        raw_data_length: None,
        raw_data: None,
    };

    let raw = news.to_raw();
    assert_eq!(raw.get_field(1473), Some("99"));

    let parsed = News::from_raw(&raw).expect("Failed to parse News");
    assert_eq!(parsed.news_category, Some(NewsCategory::OtherNews));
}

#[test]
fn test_email_type_enum_values() {
    assert_eq!(EmailType::New.to_fix(), '0');
    assert_eq!(EmailType::Reply.to_fix(), '1');
    assert_eq!(EmailType::AdminReply.to_fix(), '2');

    assert_eq!(EmailType::from_fix('0'), Some(EmailType::New));
    assert_eq!(EmailType::from_fix('1'), Some(EmailType::Reply));
    assert_eq!(EmailType::from_fix('2'), Some(EmailType::AdminReply));
    assert_eq!(EmailType::from_fix('9'), None);
}

#[test]
fn test_news_ref_type_enum_values() {
    assert_eq!(NewsRefType::Replacement.to_fix(), '0');
    assert_eq!(NewsRefType::Cancellation.to_fix(), '1');
    assert_eq!(NewsRefType::Supplement.to_fix(), '2');

    assert_eq!(NewsRefType::from_fix('0'), Some(NewsRefType::Replacement));
    assert_eq!(NewsRefType::from_fix('1'), Some(NewsRefType::Cancellation));
    assert_eq!(NewsRefType::from_fix('2'), Some(NewsRefType::Supplement));
    assert_eq!(NewsRefType::from_fix('9'), None);
}

#[test]
fn test_news_category_enum_values() {
    assert_eq!(NewsCategory::CompanyNews.to_fix(), "0");
    assert_eq!(NewsCategory::MarketplaceNews.to_fix(), "1");
    assert_eq!(NewsCategory::FinancialNews.to_fix(), "2");
    assert_eq!(NewsCategory::TechnicalNews.to_fix(), "3");
    assert_eq!(NewsCategory::OtherNews.to_fix(), "99");

    assert_eq!(NewsCategory::from_fix("0"), Some(NewsCategory::CompanyNews));
    assert_eq!(NewsCategory::from_fix("1"), Some(NewsCategory::MarketplaceNews));
    assert_eq!(NewsCategory::from_fix("2"), Some(NewsCategory::FinancialNews));
    assert_eq!(NewsCategory::from_fix("3"), Some(NewsCategory::TechnicalNews));
    assert_eq!(NewsCategory::from_fix("99"), Some(NewsCategory::OtherNews));
    assert_eq!(NewsCategory::from_fix("100"), None);
}
