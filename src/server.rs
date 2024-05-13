use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::demo::demo_interface_server::DemoInterface;
use crate::demo::{PingRequest, PingResponse};

#[derive(Default)]
pub struct PingPong;

#[tonic::async_trait]
impl DemoInterface for PingPong {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        log::info!("request: {:?}", request);
        Ok(Response::new(PingResponse {
            message: "pong".into(),
        }))
    }
}