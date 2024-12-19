pub mod automation {
    include!("generated/automation.rs");
}

use tonic::{transport::Server, Request, Response, Status};
use automation::automation_service_server::{AutomationService, AutomationServiceServer};
use automation::{CommandRequest, CommandResponse};

#[derive(Default)]
pub struct MyAutomationService {}

#[tonic::async_trait]
impl AutomationService for MyAutomationService {
    async fn send_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let command = request.into_inner().command;
        println!("Received command: {}", command);

        let reply = CommandResponse {
            result: format!("Command '{}' executed successfully!", command),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyAutomationService::default();

    println!("AutomationService running at {}", addr);

    Server::builder()
        .add_service(AutomationServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}