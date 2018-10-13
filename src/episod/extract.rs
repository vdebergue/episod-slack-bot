use chrono;
use chrono::prelude::*;

use select::node::Node;
use select::predicate::{Class, Name};

pub fn short_date_to_date(short_date: &str) -> Option<NaiveDate> {
    let now = Utc::now().naive_utc();
    let year = now.year();

    let date_components: Vec<&str> = short_date.split('/').collect();
    if date_components.len() != 2 {
        return None;
    };
    let day = match date_components[0].parse::<u32>() {
        Ok(v) => v,
        _ => return None,
    };
    let month = match date_components[1].parse::<u32>() {
        Ok(v) => v,
        _ => return None,
    };
    let date = NaiveDate::from_ymd(year, month, day);
    if date < now.date() {
        Some(NaiveDate::from_ymd(year + 1, month, day))
    } else {
        Some(date)
    }
}

fn time_to_time(time: &str) -> NaiveTime {
    let time_components: Vec<&str> = time.split('H').collect();
    NaiveTime::from_hms(
        time_components[0].parse::<u32>().unwrap(),
        time_components[1].parse::<u32>().unwrap(),
        0,
    )
}

fn duration_to_duration(duration: &str) -> i64 {
    let duration_components: Vec<&str> = duration.split('\u{a0}').collect();
    match duration_components[1] {
        "mins" => chrono::Duration::minutes(duration_components[0].parse::<i64>().unwrap()),
        _ => chrono::Duration::minutes(duration_components[0].parse::<i64>().unwrap()),
    }.num_minutes()
}

pub fn node_to_session(node: Node) -> super::Session {
    super::Session {
        reservation_link: node.attr("data-href").unwrap().to_string(),
        sport: node.attr("data-sport").unwrap().to_string(),
        coach: node.attr("data-coach").unwrap().to_string(),
        hub: node.attr("data-hub").unwrap().to_string(),
        full: node.is(Class("status-complet")),
        duration_minutes: duration_to_duration(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("span"))
                .last()
                .unwrap()
                .text(),
        ),
        time: time_to_time(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("time"))
                .last()
                .unwrap()
                .text(),
        ),
        date: short_date_to_date(
            &node
                .find(Class("planning-date"))
                .last()
                .unwrap()
                .find(Name("div"))
                .last()
                .unwrap()
                .text(),
        ).unwrap(),
    }
}
