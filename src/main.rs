use std::{io, env};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    pretty_env_logger::init();
    
    realworld::run().await

}
