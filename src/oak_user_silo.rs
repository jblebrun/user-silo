use crate::{
    orchestrator_client::OrchestratorClient,
    proto::oak::containers::user_silo::{
        t_oak_user_silo_server::TOakUserSilo, TInvokeRequest, TInvokeResponse,
    },
};

struct OakUserSiloImplementation {
    orchestrator_client: OrchestratorClient,
    application_config: Vec<u8>,
}

impl OakUserSiloImplementation {
    pub fn new(orchestrator_client: OrchestratorClient, application_config: Vec<u8>) -> Self {
        Self {
            orchestrator_client,
            application_config,
        }
    }
}

#[tonic::async_trait]
impl TOakUserSilo for OakUserSiloImplementation {
    async fn t_invoke(
        &self,
        request: tonic::Request<TInvokeRequest>,
    ) -> Result<tonic::Response<TInvokeResponse>, tonic::Status> {
    }
}
