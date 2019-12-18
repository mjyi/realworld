use std::{env, io};
use dotenv::dotenv;
use realworld::errors::CliError;

#[actix_rt::main]
async fn main() -> Result<(), CliError> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    pretty_env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    realworld::run(&database_url).await?;

    Ok(())
}
