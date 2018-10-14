#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub token: String,
    pub channel: String,
    pub query: String,
}
