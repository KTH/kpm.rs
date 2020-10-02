use tera::Tera;
use tide::{Request, Response};
use tide_tera::prelude::*;
use tide::http::mime;
use tide::http::headers::EXPIRES;
use httpdate::fmt_http_date;
use std::time::{Duration, SystemTime};
use std::env;
use async_std::process::exit;

mod css;
mod footer;

type Error = Box<dyn std::error::Error + Send + Sync>;

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

async fn run() -> Result<(), Error> {
    let mut app = tide::with_state(State::new()?);

    app.at("/kpm/").get(start_page);
    app.at("/kpm/index.js").get(index_js);
    app.at(&format!("/kpm/{}", css::page_css_name())).get(page_css);
    app.at(&format!("/kpm/{}", css::menu_css_name())).get(menu_css);
    app.at("/kpm/_monitor").get(monitor);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

/// The application state for kpm
#[derive(Clone)]
struct State {
    tera: Tera,
    footer: footer::Footer,
}

impl State {
    fn new() -> Result<State, Error> {
        let mut tera = Tera::new("templates/**/*")?;
        tera.autoescape_on(vec!["html"]);
        let footer = footer::Footer::new();
        Ok(State { tera, footer })
    }
}

async fn start_page(req: Request<State>) -> Result<Response, tide::Error> {
    let kpm = req.state();
    kpm.tera.render_response(
        "index.html",
        &context! {
            "page_css" => css::page_css_name(),
            "kth_footer" => *kpm.footer.get().await,
        },
    )
}

async fn monitor(_req: Request<State>) -> Result<Response, tide::Error> {
    Ok(format!(
        "APPLICATION_STATUS: {} {}-{}\n",
        "OK",
        env!("CARGO_PKG_NAME"),
        option_env!("dockerVersion").unwrap_or("unknown"),
    ).into())
}

async fn index_js(req: Request<State>) -> Result<Response, tide::Error> {
    let kpm = req.state();
    let host_url = env_or("SERVER_HOST_URL", "http://localdev.kth.se:8080");
    kpm.tera.render_response("index.js", &context! {
        "css_url" => format!("{}/kpm/{}", host_url, css::menu_css_name()),
    })
}

fn env_or(var: &str, default: &str) -> String {
    env::var(var).unwrap_or_else(|err| {
        tide::log::error!("Error getting {:?}: {}, using {:?}", var, err, default);
        default.into()
    })
}

async fn page_css(_: Request<State>) -> Result<Response, tide::Error> {
    Ok(css_result(css::PAGE_CSS))
}
async fn menu_css(_: Request<State>) -> Result<Response, tide::Error> {
    Ok(css_result(css::MENU_CSS))
}
fn css_result(style: &str) -> Response {
    let mut res: Response = style.into();
    res.set_content_type(mime::CSS);
    res.insert_header(EXPIRES, fmt_http_date(SystemTime::now() + 180 * DAY));
    res
}

const DAY: Duration = Duration::from_secs(24 * 60 * 60);
