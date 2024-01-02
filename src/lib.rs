mod oak_user_silo;
mod orchestrator_client;

mod proto {
    pub mod oak {
        pub mod containers {
            pub mod user_silo {
                tonic::include_proto!("oak.user_silo");
            }
            use oak_crypto::proto::oak::crypto;
            pub use oak_remote_attestation::proto::oak::{attestation, session};
        }
    }
}
