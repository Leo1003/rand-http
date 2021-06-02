use rand::prelude::*;
use warp::Filter;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let index = warp::path::end()
        .map(|| {
            let mut rng = thread_rng();
            let r: u32 = rng.gen();
            format!("{}", r)
        });

    let argv: Vec<String> = std::env::args().collect();
    let port: u16 = argv.get(1).map(|s| s.parse().ok()).flatten().unwrap_or(8000);

    eprintln!("Server served at 0.0.0.0:{}", port);

    warp::serve(index)
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await;
}

