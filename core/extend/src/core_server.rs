use anyhow::Context;
use common::error::AppResult;
use register::{call_core_server::{CallCore, CallCoreServer}, RegisterRequest, RegisterResponse};
use tonic::{transport::Server, Request, Response, Status};

use crate::connection::{add_write_extend, get_read_extend_list};

pub mod register {
    tonic::include_proto!("call_core");
}

pub async fn start_server() -> AppResult<()> {
    let addr = "[::1]:50051".parse().context("Failed to parse address")?;

    let register = RegisterService::default();

    tokio::spawn(async move {
        if let Err(err) = Server::builder()
            .add_service(CallCoreServer::new(register))
            .serve(addr)
            .await
        {
            panic!("{}", err)
        }
    });

    println!("Starting RegisterServer!");

    Ok(())
}

#[derive(Debug, Default)]
pub struct RegisterService;

#[tonic::async_trait]
impl CallCore for RegisterService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();

        match add_write_extend(req.name.clone(), req.addr.clone())? {
            Some(v) => println!("old address {} is replaced", v),
            None => println!("add extend success"),
        };

        println!("{:?}", get_read_extend_list()?);

        Ok(Response::new(RegisterResponse {
            success: true,
            message: format!("extend: {} address: {}", req.name, req.addr),
        }))
    }
}
