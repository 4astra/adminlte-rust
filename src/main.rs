use tide::{Redirect, Request, Result};

pub mod models;
pub mod routes;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();

    // server public directory for assets
    app.at("/public").serve_dir("./public/")?;

    // force go to admin page
    app.at("/").get(Redirect::new("/admin"));

    app.at("/admin").get(routes::admin);

    app.at("/hello")
        .get(|_req: Request<()>| async move { Ok("Hello world!") });
    // listen
    println!("Server listen on 127.0.0.1:1987");
    app.listen("127.0.0.1:1987").await?;

    Ok(())
}
