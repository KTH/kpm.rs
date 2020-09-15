#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/kpm/").get(|_| async { Ok("Hello, world!") });
    app.at("/kpm/_monitor").get(|_| async { Ok("APPLICATION_STATUS: OK\n") });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
