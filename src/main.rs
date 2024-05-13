use tonic::transport::Server;
use crate::server::PingPong;
use demo::demo_interface_server::DemoInterfaceServer;
use std::io::Write;


pub mod server;
pub mod demo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();

    let addr = "127.0.0.1:8080".parse()?;
    let pingpong = PingPong::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    Server::builder()
        .add_service(DemoInterfaceServer::new(pingpong))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
