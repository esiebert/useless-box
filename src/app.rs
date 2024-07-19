use anyhow::Result;
use thiserror::Error;
use tonic_reflection::server::Builder as ReflectionServerBuilder;

use crate::api::{
    grpc::{BuildError as GRPCBuildError, Builder as GRPCBuilder, GRPC},
    SQLite,
};
use crate::cycling_tracker;
use crate::service::{CyclingTrackerService, SessionAuthService};
use crate::FILE_DESCRIPTOR_SET;

pub struct App {
    grpc: GRPC,
    sqlite: SQLite,
}

impl App {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn run(mut self) -> Result<()> {
        tokio::select! {
            e = self.grpc.run() => {
                e
            }
            e = self.sqlite.run() => {
                e
            }
        }
    }
}

pub struct Builder {
    grpc: Option<GRPC>,
    sqlite: Option<SQLite>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            grpc: None,
            sqlite: None,
        }
    }

    pub fn setup_grpc(mut self, host_url: String) -> Result<Self, BuildError> {
        let auth = cycling_tracker::SessionAuthServer::new(SessionAuthService {});

        let sqlite = self.sqlite.as_ref().ok_or(BuildError::SQLiteNotSet(
            "Failed building cycling tracker server",
        ))?;

        let cts = cycling_tracker::CyclingTrackerServer::new(
            CyclingTrackerService::new(sqlite.handler()),
        );

        let refl = ReflectionServerBuilder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .map_err(|e| BuildError::ReflectionBuildError(e.to_string()))?;

        let grpc = GRPCBuilder::new()
            .with_addr(host_url)?
            .with_tls()?
            .add_auth_service(auth)
            .add_reflection_service(refl)
            .add_ct_service(cts, false)
            .build()?;

        self.grpc = Some(grpc);
        Ok(self)
    }

    pub fn setup_sqlite(mut self) -> Self {
        self.sqlite = Some(SQLite::new());
        self
    }

    pub fn build(self) -> Result<App, BuildError> {
        let grpc = self.grpc.ok_or(BuildError::GRPCNotSet)?;
        let sqlite = self
            .sqlite
            .ok_or(BuildError::SQLiteNotSet("Failed building app"))?;

        Ok(App {
            grpc: grpc,
            sqlite: sqlite,
        })
    }
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Failed to build gRPC: {0}")]
    GRPCBuildFailure(#[from] GRPCBuildError),
    #[error("SQLite not set when it was needed: {0}")]
    SQLiteNotSet(&'static str),
    #[error("Failed to build reflection server: {0}")]
    ReflectionBuildError(String),
    #[error("gRPC service not set")]
    GRPCNotSet,
}
