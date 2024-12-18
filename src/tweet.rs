use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};
use spin_sdk::http as spinhttp;

#[derive(Serialize, Deserialize, Debug)]
struct Publish {
    html: String,
}

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(tweet: |user: String, id: String| {
        let url = format!("https://publish.twitter.com/oembed?url=https://twitter.com/{}/status/{}", user, id);
        let req = spinhttp::Request::get(&url);
        let res_fut = spinhttp::send(req);
        let res: spinhttp::Response = spinhttp::run(res_fut).unwrap();

        let mut html = "".to_string();
        let body = res.body();
        let str = std::str::from_utf8(body).unwrap().to_string();

        if is_success_status(res.status()) {
            let deserialized: Publish = serde_json::from_str(&str).unwrap();
            html = deserialized.html.to_string()
        } else {
            html = str
        }

        html
    });

    x.register_helper("tweet", Box::new(tweet));
}

fn is_success_status(status: &spinhttp::StatusCode) -> bool {
    http::StatusCode::from_u16(*status).is_ok_and(|s| s.is_success())
}
