use tera::Tera;
use tide::{Request, Response};
use tide_tera::prelude::*;
use tide::http::mime;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tide::log::start();

    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/kpm/").get(start_page);
    app.at("/kpm/index.js").get(index_js);
    app.at("/kpm/index.css").get(index_css);
    app.at("/kpm/_monitor")
        .get(|_| async { Ok("APPLICATION_STATUS: OK\n") });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn start_page(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    tera.render_response("index.html", &context! {})
}

async fn index_js(req: Request<Tera>) -> Result<Response, tide::Error> {
    let tera = req.state();
    tera.render_response("index.js", &context! {})
}

async fn index_css(_: Request<Tera>) -> Result<Response, tide::Error> {
    let mut res: Response = CSS.into();
    res.set_content_type(mime::CSS);
    Ok(res)
}

static CSS: &str = "
    body{
        margin: 2rem;
    }
    #kpm>.container{
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        height: 2rem;
        background: coral;
        color: white;
    }\n";
