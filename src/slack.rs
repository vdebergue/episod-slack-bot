#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EventCallback {
    #[serde(rename = "app_mention")]
    AppMention {
        user: String,
        text: String,
        ts: String,
        channel: String,
        event_ts: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "url_verification")]
    UrlVerification { token: String, challenge: String },
    #[serde(rename = "event_callback")]
    EventCallback {
        token: String,
        team_id: String,
        api_app_id: String,
        event: EventCallback,
        event_id: String,
        event_time: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReply {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: Option<String>,
    pub channel: String,
    pub token: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub fallback: Option<String>,
    pub actions: Vec<AttachmentAction>,
    pub text: Option<String>,
    pub thumb_url: Option<String>,
    pub ts: Option<i64>,
    pub color: Option<String>,
    pub author_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub url: String,
    pub text: String,
    pub style: Option<String>,
}
