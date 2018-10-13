extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

extern crate http;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate aws_lambda;
extern crate reqwest;

extern crate rand;

extern crate episod;

use std::collections::HashMap;

use rand::Rng;

lazy_static! {
    pub static ref PLANNING_HTML: String = {
        reqwest::get("https://www.episod.com/planning/")
            .unwrap()
            .text()
            .unwrap()
    };
}

mod aws_api_helpers;

use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    token: String,
    channel: String,
    query: String,
}

fn pseudo_nlp(query: &str) -> episod::episod::Filters {
    let sport = match query {
        query if query.contains("boxe") => Some("boxing".to_string()),
        query if query.contains("boxing") => Some("boxing".to_string()),
        query if query.contains("yoga") => Some("yoga".to_string()),
        query if query.contains("bootcamp") => Some("bootcamp".to_string()),
        query if query.contains("pilates") => Some("pilates".to_string()),
        _ => None,
    };

    let hub = match query {
        query if query.contains("bourse") => Some("bourse".to_string()),
        query if query.contains("republique") => Some("republique".to_string()),
        query if query.contains("république") => Some("republique".to_string()),
        query if query.contains("clichy") => Some("clichy".to_string()),
        _ => None,
    };

    let coach = match query {
        query if query.contains("alexandre") => Some("alexandre".to_string()),
        query if query.contains("alyona") => Some("alyona".to_string()),
        query if query.contains("amadou") => Some("amadou".to_string()),
        query if query.contains("ange") => Some("ange".to_string()),
        query if query.contains("anna") => Some("anna".to_string()),
        query if query.contains("anne-julie") => Some("anne-julie".to_string()),
        query if query.contains("avine") => Some("avine".to_string()),
        query if query.contains("bouba") => Some("bouba".to_string()),
        query if query.contains("clelia") => Some("clelia".to_string()),
        query if query.contains("clio-pajczer") => Some("clio-pajczer".to_string()),
        query if query.contains("djamel") => Some("djamel".to_string()),
        query if query.contains("emelyne") => Some("emelyne".to_string()),
        query if query.contains("emeric") => Some("emeric".to_string()),
        query if query.contains("estelle") => Some("estelle".to_string()),
        query if query.contains("evidant") => Some("evidant".to_string()),
        query if query.contains("floriane") => Some("floriane".to_string()),
        query if query.contains("franck") => Some("franck".to_string()),
        query if query.contains("gary") => Some("gary".to_string()),
        query if query.contains("guillaume") => Some("guillaume".to_string()),
        query if query.contains("helen") => Some("helen".to_string()),
        query if query.contains("jimmy") => Some("jimmy".to_string()),
        query if query.contains("john") => Some("john".to_string()),
        query if query.contains("johnny") => Some("johnny".to_string()),
        query if query.contains("karine") => Some("karine".to_string()),
        query if query.contains("laura") => Some("laura".to_string()),
        query if query.contains("laurence") => Some("laurence".to_string()),
        query if query.contains("ludo") => Some("ludo".to_string()),
        query if query.contains("ludovic") => Some("ludo".to_string()),
        query if query.contains("ludob") => Some("ludob".to_string()),
        query if query.contains("ludo-m") => Some("ludo-m".to_string()),
        query if query.contains("ludovic-r") => Some("ludovic-r".to_string()),
        query if query.contains("lylou") => Some("lylou".to_string()),
        query if query.contains("marie-soline") => Some("marie-soline".to_string()),
        query if query.contains("martin") => Some("martin".to_string()),
        query if query.contains("maxime") => Some("maxime".to_string()),
        query if query.contains("michael") => Some("michael".to_string()),
        query if query.contains("minh") => Some("minh".to_string()),
        query if query.contains("nasser") => Some("nasser".to_string()),
        query if query.contains("nhick") => Some("nhick".to_string()),
        query if query.contains("olivia") => Some("olivia".to_string()),
        query if query.contains("pascale") => Some("pascale".to_string()),
        query if query.contains("patrice") => Some("patrice".to_string()),
        query if query.contains("philippe") => Some("philippe".to_string()),
        query if query.contains("rachid") => Some("rachid".to_string()),
        query if query.contains("rahim") => Some("rahim".to_string()),
        query if query.contains("rakesh") => Some("rakesh".to_string()),
        query if query.contains("salim") => Some("salim".to_string()),
        query if query.contains("skander") => Some("skander".to_string()),
        query if query.contains("sophia") => Some("sophia".to_string()),
        query if query.contains("souleymane") => Some("souleymane".to_string()),
        query if query.contains("steven") => Some("steven".to_string()),
        query if query.contains("ted") => Some("ted".to_string()),
        query if query.contains("thierry") => Some("thierry".to_string()),
        query if query.contains("yassin") => Some("yassin".to_string()),
        _ => None,
    };

    let period = match query {
        query if query.contains("matin") => Some(episod::episod::Period::Matin),
        query if query.contains("midi") => Some(episod::episod::Period::Midi),
        query if query.contains("soir") => Some(episod::episod::Period::Soir),
        _ => None,
    };

    let day = match query {
        query if query.contains("lundi") => Some(episod::episod::Day::Lundi),
        query if query.contains("mardi") => Some(episod::episod::Day::Mardi),
        query if query.contains("mercredi") => Some(episod::episod::Day::Mercredi),
        query if query.contains("jeudi") => Some(episod::episod::Day::Jeudi),
        query if query.contains("vendredi") => Some(episod::episod::Day::Vendredi),
        query if query.contains("samedi") => Some(episod::episod::Day::Samedi),
        query if query.contains("dimanche") => Some(episod::episod::Day::Dimanche),
        query if query.contains("demain") => Some(episod::episod::Day::Demain),
        _ => None,
    };

    let date = query
        .split(' ')
        .filter(|t| t.contains('/'))
        .last()
        .and_then(episod::episod::extract::short_date_to_date);

    episod::episod::Filters {
        sport,
        coach,
        date,
        day,
        hub,
        period,
    }
}

fn send_sessions(notifications: aws_lambda::event::sns::SnsEvent) {
    let colors = vec![
        "#C0C0C0", "#FF0000", "#00FF00", "#439FE0", "#00FFFF", "#008080", "#0000FF", "#FF00FF",
        "#800080", "#3cb371", "#ffa500", "#6a5acd", "#ee82ee",
    ];
    notifications.records.iter().for_each(|notification| {
        let msg: Notification =
            serde_json::from_str(&notification.clone().sns.message.unwrap()).unwrap();

        let sessions =
            episod::episod::extract_sessions_and_filter(&PLANNING_HTML, &pseudo_nlp(&msg.query));
        send_to_slack(&episod::slack::Message {
            attachments: sessions
                .iter()
                .take(10)
                .map(|session| episod::slack::Attachment {
                    fallback: None,
                    text: Some(format!(
                        "{} le *{}* à *{}* ({} minutes)",
                        session.sport, session.date, session.time, session.duration_minutes
                    )),
                    actions: vec![episod::slack::AttachmentAction {
                        action_type: "button".to_string(),
                        url: session.reservation_link.clone(),
                        text: "Réserver 🏅".to_string(),
                        style: Some("primary".to_string()),
                    }],
                    thumb_url: match session.sport.as_ref() {
                        "bootcamp" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/weight-lifter_1f3cb.png".to_string()),
                        "boxing" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/boxing-glove_1f94a.png".to_string()),
                        "yoga-vinyasa" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "yin-yoga" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "yoga-hatha" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "rowing" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/rowboat_1f6a3.png".to_string()),
                        "cycling" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/bicyclist_1f6b4.png".to_string()),
                        "pilates" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-doing-cartwheel_1f938.png".to_string()),
                        _ => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/flexed-biceps_1f4aa.png".to_string()),
                    },
                    ts: None,
                    color: Some(rand::thread_rng().choose(&colors).unwrap().to_string()),
                    author_name: Some(format!("{} ({})", session.coach, session.hub))
                }).collect(),
            channel: msg.channel,
            token: msg.token,
            text: None,
        });
    });
}

fn send_to_slack(message: &episod::slack::Message) {
    reqwest::Client::new()
        .post("https://slack.com/api/chat.postMessage")
        .json(message)
        .bearer_auth(env::var("slack_token").unwrap())
        .send()
        .unwrap()
        .text()
        .unwrap();
}

fn main() {
    aws_lambda::start(|notifications: aws_lambda::event::sns::SnsEvent| {
        send_sessions(notifications);
        Ok(aws_lambda::event::apigw::ApiGatewayProxyResponse {
            body: Some("not gateway anyway".to_string()),
            status_code: 200,
            is_base64_encoded: None,
            headers: HashMap::new(),
        })
    })
}
