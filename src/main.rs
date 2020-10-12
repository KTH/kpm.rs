use async_std::process::exit;
use httpdate::fmt_http_date;
use serde::Deserialize;
use std::env;
use std::time::{Duration, SystemTime};
use tera::Tera;
use tide::http::headers::{EXPIRES, LOCATION, SET_COOKIE};
use tide::http::mime;
use tide::{Request, Response};
use tide_tera::prelude::*;

mod css;
mod footer;

type Error = Box<dyn std::error::Error + Send + Sync>;

/// The main entry point.
///
/// This just initalizies the logger, calls run and logs when it returns.
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

/// The app entry point
///
/// This creates the state and the app, adds the route handlers and
/// runs the app.
async fn run() -> Result<(), Error> {
    let mut app = tide::with_state(State::new()?);

    app.at("/kpm/").get(start_page);
    app.at("/kpm/").post(enable_or_disable);
    app.at("/kpm/index.js").get(index_js);
    app.at(&format!("/kpm/{}", css::page_css_name()))
        .get(page_css);
    app.at(&format!("/kpm/{}", css::menu_css_name()))
        .get(menu_css);
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
    /// Initialize the global state.
    ///
    /// This should only be done once, on application startup.
    fn new() -> Result<State, Error> {
        let mut tera = Tera::new("templates/**/*")?;
        tera.autoescape_on(vec!["html"]);
        let footer = footer::Footer::new();
        Ok(State { tera, footer })
    }
    /// Get the base url of this app.
    ///
    /// In production, this should return "https://app.kth.se/kpm/".
    /// The result depends on the `$SERVER_HOST_URL` environment, and
    /// adds "/kpm/" to that.
    fn base_url(&self) -> String {
        let host_url = env_or("SERVER_HOST_URL", "http://localdev.kth.se:8080");
        format!("{}/kpm/", host_url)
    }
}

async fn start_page(req: Request<State>) -> Result<Response, tide::Error> {
    let kpm = req.state();
    let is_active = req.cookie("use_kpm").is_some();
    kpm.tera.render_response(
        "index.html",
        &context! {
            "is_active" => is_active,
            "page_css" => css::page_css_name(),
            "kth_footer" => *kpm.footer.get().await,
        },
    )
}

/// The action for a POST of the enable/disable form.
///
/// Parses / validates the form data.
/// If the request is to enable kpm, a `use_kpm` cookie is set in the
/// response.
/// Otherwise (to disable kpm), the `use_kpm` cookie is cleared.
/// In either case, the action is logged.
async fn enable_or_disable(mut req: Request<State>) -> Result<Response, tide::Error> {
    let post: StatusForm = req.body_form().await?;
    let kpm = req.state();
    let activate = post.action == "enable";
    if activate {
        tide::log::info!("A user activated KPM");
    } else {
        tide::log::info!("A user disabled KPM");
    }
    Ok(Response::builder(302)
        .header(LOCATION, kpm.base_url())
        .header(
            SET_COOKIE,
            if activate {
                "use_kpm=t; Domain=.kth.se; Path=/; HttpOnly"
            } else {
                "use_kpm=; Max-Age=0; Domain=.kth.se; Path=/; HttpOnly"
            },
        )
        .build())
}

/// The form data required when posting to `enable_or_disable`.
///
/// Currently only contains an `action` string.
#[derive(Debug, Deserialize)]
struct StatusForm {
    action: String,
}

async fn monitor(_req: Request<State>) -> Result<Response, tide::Error> {
    Ok(format!(
        "APPLICATION_STATUS: {} {}-{}\n",
        "OK",
        env!("CARGO_PKG_NAME"),
        option_env!("dockerVersion").unwrap_or("unknown"),
    )
    .into())
}

async fn index_js(req: Request<State>) -> Result<Response, tide::Error> {
    let kpm = req.state();
    let base_url = kpm.base_url();
    kpm.tera.render_response(
        "index.js",
        &context! {
            "css_url" => format!("{}{}", base_url, css::menu_css_name()),
            "kpm_base" => base_url,
        },
    )
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
