use serde_derive::{Deserialize, Serialize};
use std::sync::OnceLock;

pub static DATA: OnceLock<Root> = OnceLock::new();

// TODO: switch this from runtime to compile time next
pub fn get_data() -> &'static Root {
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../jsons/wmn-data.json"))
            .expect("can't parse the whatsmyname json...")
    })
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub license: Vec<String>,
    pub authors: Vec<String>,
    pub categories: Vec<String>,
    pub sites: Vec<Site>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub name: String,

    #[serde(rename = "uri_check")]
    pub uri_check: String,

    #[serde(rename = "e_code")]
    pub e_code: u16,

    #[serde(rename = "e_string")]
    pub e_string: String,

    #[serde(rename = "m_string")]
    pub m_string: String,

    #[serde(rename = "m_code")]
    pub m_code: u16,

    pub known: Vec<String>,

    pub cat: String,

    #[serde(default)]
    pub protection: Vec<String>,

    pub valid: Option<bool>,

    #[serde(rename = "uri_pretty")]
    pub uri_pretty: Option<String>,

    #[serde(rename = "post_body")]
    pub post_body: Option<String>,

    pub headers: Option<Headers>,

    #[serde(rename = "strip_bad_char")]
    pub strip_bad_char: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,

    #[serde(rename = "X-Tool")]
    pub x_tool: Option<String>,

    #[serde(rename = "X-VT-Anti-Abuse-Header")]
    pub x_vt_anti_abuse_header: Option<String>,

    #[serde(rename = "Content-Type")]
    pub content_type: Option<String>,

    #[serde(rename = "Cookie")]
    pub cookie: Option<String>,

    #[serde(rename = "content-type")]
    pub content_type2: Option<String>,

    #[serde(rename = "Accept")]
    pub accept: Option<String>,

    #[serde(rename = "Cache-Control")]
    pub cache_control: Option<String>,

    #[serde(rename = "Host")]
    pub host: Option<String>,

    #[serde(rename = "Accept-Language")]
    pub accept_language: Option<String>,

    #[serde(rename = "Origin")]
    pub origin: Option<String>,

    #[serde(rename = "Referer")]
    pub referer: Option<String>,

    #[serde(rename = "TE")]
    pub te: Option<String>,

    #[serde(rename = "accept")]
    pub accept2: Option<String>,
}
