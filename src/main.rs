use tera::Tera;
use tide::{Request, Response};
use tide_tera::prelude::*;
use tide::http::mime;
use tide::http::headers::EXPIRES;
use httpdate::fmt_http_date;
use std::time::{Duration, SystemTime};
use std::env;
mod css;
use async_std::process::exit;

#[async_std::main]
async fn main() {
    tide::log::start();
    match run().await {
        Ok(()) => tide::log::info!("Shutting down"),
        Err(e) => {
            tide::log::error!("Shutting down on error", {
                error: e.to_string(),
            });
            exit(1);
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/kpm/").get(start_page);
    app.at("/kpm/index.js").get(index_js);
    app.at(&format!("/kpm/index-{}.css", css::hash())).get(index_css);
    app.at("/kpm/_monitor").get(monitor);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn start_page(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    tera.render_response("index.html", &context! {})
}

async fn monitor(_req: Request<Tera>) -> Result<Response, tide::Error> {
    Ok(format!(
        "APPLICATION_STATUS: {} {}-{}\n",
        "OK",
        env!("CARGO_PKG_NAME"),
        option_env!("dockerVersion").unwrap_or("unknown"),
    ).into())
}

async fn index_js(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    let host_url = env_or("SERVER_HOST_URL", "http://localdev.kth.se:8080");
    tera.render_response("index.js", &context! {
        "css_url" => format!("{}/kpm/index-{}.css", host_url, css::hash()),
    })
}

fn env_or(var: &str, default: &str) -> String {
    env::var(var).unwrap_or_else(|err| {
        tide::log::error!("Error getting {:?}: {}, using {:?}", var, err, default);
        default.into()
    })
}

async fn index_css(_: Request<Tera>) -> Result<Response, tide::Error> {
    let mut res: Response = css::CSS.into();
    res.set_content_type(mime::CSS);
    res.insert_header(EXPIRES, fmt_http_date(SystemTime::now() + 180 * DAY));
    Ok(res)
}

const DAY: Duration = Duration::from_secs(24 * 60 * 60);
