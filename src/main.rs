use realworld::{errors::CliError, Settings};
use std::env;

#[actix_rt::main]
async fn main() -> Result<(), CliError> {
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,actix_server=info,realworld::diesel=debug,realworld=debug",
    );
    pretty_env_logger::init();

    let settings = Settings::get();
    realworld::run(settings).await?;

    Ok(())
}
