use chrono;
use chrono::prelude::*;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Not, Predicate};

pub mod extract;

#[derive(Debug, Serialize, Clone)]
pub struct Session {
    pub reservation_link: String,
    pub coach: String,
    pub hub: String,
    pub sport: String,
    pub duration_minutes: i64,
    pub full: bool,
    pub time: NaiveTime,
    pub date: NaiveDate,
}

use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref CACHE_SESSIONS: Arc<RwLock<Option<Vec<Session>>>> =
        { Arc::new(RwLock::new(None)) };
}

pub fn extract_sessions_and_filter<'a>(html: &'a str, filters: &Filters) -> Vec<Session> {
    let mut sessions_lock = CACHE_SESSIONS.write().unwrap();
    let sessions = if let Some(sessions) = sessions_lock.clone() {
        sessions
    } else {
        let sessions: Vec<Session> = Document::from(html)
            .find(
                Name("li")
                    .and(Class("planning-item"))
                    .and(Not(Attr("id", "no-session"))),
            ).map(extract::node_to_session)
            .collect();
        *sessions_lock = Some(sessions.clone());
        sessions
    };

    sessions
        .iter()
        .filter(|session| match filters.hub {
            Some(ref hub) => session.hub.contains(hub),
            _ => true,
        }).filter(|session| match filters.coach {
            Some(ref coach) => session.coach.contains(coach),
            _ => true,
        }).filter(|session| match filters.sport {
            Some(ref sport) => session.sport.contains(sport),
            _ => true,
        }).filter(|session| match filters.day {
            Some(ref day) => session.date.weekday() == day.to_weekday(),
            _ => true,
        }).filter(|session| match filters.date {
            Some(ref date) => session.date == *date,
            _ => true,
        }).filter(|session| match filters.period {
            Some(ref period) => period.match_time(session.time),
            _ => true,
        }).cloned()
        .collect()
}

#[derive(Debug, Deserialize, Default)]
pub struct Filters {
    pub hub: Option<String>,
    pub coach: Option<String>,
    pub sport: Option<String>,
    pub day: Option<Day>,
    pub date: Option<NaiveDate>,
    pub period: Option<Period>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Day {
    Lundi,
    Mardi,
    Mercredi,
    Jeudi,
    Vendredi,
    Samedi,
    Dimanche,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Demain,
    Tomorrow,
}
impl Day {
    fn to_weekday(&self) -> chrono::Weekday {
        match self {
            Day::Lundi | Day::Monday => chrono::Weekday::Mon,
            Day::Mardi | Day::Tuesday => chrono::Weekday::Tue,
            Day::Mercredi | Day::Wednesday => chrono::Weekday::Wed,
            Day::Jeudi | Day::Thursday => chrono::Weekday::Thu,
            Day::Vendredi | Day::Friday => chrono::Weekday::Fri,
            Day::Samedi | Day::Saturday => chrono::Weekday::Sat,
            Day::Dimanche | Day::Sunday => chrono::Weekday::Sun,
            Day::Demain | Day::Tomorrow => Utc::now().naive_utc().weekday().succ(),
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Matin,
    Midi,
    Soir,
    Morning,
    Lunch,
    Evening,
}
impl Period {
    fn match_time(&self, time: NaiveTime) -> bool {
        match self {
            Period::Matin | Period::Morning => time < NaiveTime::from_hms(11, 0, 0),
            Period::Soir | Period::Evening => time > NaiveTime::from_hms(17, 0, 0),
            Period::Midi | Period::Lunch => {
                time > NaiveTime::from_hms(11, 45, 0) && time < NaiveTime::from_hms(14, 15, 0)
            }
        }
    }
}
