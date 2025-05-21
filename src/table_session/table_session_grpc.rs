use std::str::FromStr;

use tonic::Request;
use tonic::Response;
use tonic::Status;
use uuid::Uuid;

use super::TableSessionService;
use super::proto;

pub struct TableSessionGrpc {
    table_session_service: TableSessionService,
}

impl TableSessionGrpc {
    pub fn new(table_session_service: TableSessionService) -> Self {
        Self { table_session_service }
    }
}

#[tonic::async_trait]
impl proto::table_session_service_server::TableSessionService for TableSessionGrpc {
    async fn create_table_session(
        &self,
        request: Request<proto::CreateTableSessionRequest>,
    ) -> Result<Response<proto::CreateTableSessionResponse>, Status> {
        let request = request.into_inner();
        let table_id = Uuid::from_str(&request.table_id)
            .map_err(|_| Status::invalid_argument("table_id not a UUID"))?;

        // TODO: Check if table is occupied

        match self.table_session_service.create_session(table_id).await {
            Ok(table_session) => {
                let session_id = table_session.id.to_string();
                Ok(Response::new(proto::CreateTableSessionResponse { session_id }))
            }
            Err(e) => Err(Status::internal(format!("Failed to create session: {e}")))
        }
    }

    async fn get_table_session(
        &self,
        request: Request<proto::GetTableSessionRequest>,
    ) -> Result<Response<proto::GetTableSessionResponse>, Status> {
        let request = request.into_inner();
        let session_id = Uuid::from_str(&request.session_id)
            .map_err(|_| Status::invalid_argument("session_id not a UUID"))?;

        match self.table_session_service.find_by_id(session_id).await {
            Ok(Some(table_session)) => {
                Ok(Response::new(proto::GetTableSessionResponse {
                    table_session: Some(proto::TableSession::from(table_session))
                }))
            },
            Ok(None) => Err(Status::not_found("Table Session not found")),
            Err(e) => Err(Status::unauthenticated(format!("Unable to get Table Session: {e}"))),
        }
    }
}
