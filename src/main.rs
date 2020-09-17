use tera::Tera;
use tide::{Request, Response};
use tide_tera::prelude::*;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tide::log::start();

    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/kpm/").get(start_page);
    app.at("/kpm/toolbar.js").get(toolbar_js);
    app.at("/kpm/_monitor")
        .get(|_| async { Ok("APPLICATION_STATUS: OK\n") });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn start_page(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    tera.render_response("index.html", &context! {})
}

async fn toolbar_js(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    tera.render_response("toolbar.js", &context! {})
}

