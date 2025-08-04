mod error;
mod flags;

#[allow(clippy::default_trait_access)]
mod protos {
    tonic::include_proto!("allnodes.v4");
}

use {
    crate::error::ConversionError, solana_hash::Hash, solana_pubkey::Pubkey, std::net::SocketAddr,
};
pub use {
    client::AllnodesServiceClient as Client,
    error::Error,
    flags::Flags,
    protos::{
        allnodes_service_client as client, allnodes_service_server as server,
        SnapshotNode as SnapshotNodePb, SnapshotRequest as SnapshotRequestPb,
        SnapshotResponse as SnapshotResponsePb, ValidatorFlagsRequest as ValidatorFlagsRequestPb,
        ValidatorFlagsResponse as ValidatorFlagsResponsePb,
    },
    server::AllnodesServiceServer as Server,
};

#[derive(Debug, Default)]
pub struct SnapshotRequest {
    pub shred_version: u16,
}

impl From<&SnapshotRequest> for SnapshotRequestPb {
    fn from(from: &SnapshotRequest) -> Self {
        Self {
            shred_version: from.shred_version.into(),
        }
    }
}

impl TryFrom<SnapshotRequestPb> for SnapshotRequest {
    type Error = ConversionError;

    fn try_from(from: SnapshotRequestPb) -> Result<Self, Self::Error> {
        Ok(Self {
            shred_version: u16::try_from(from.shred_version)?,
        })
    }
}

#[derive(Debug)]
pub struct SnapshotResponse {
    pub rpc: SocketAddr,
    pub pubkey: Pubkey,
    pub snapshot_hash: SnapshotHash,
    pub latency_microseconds: u64,
}

impl TryFrom<SnapshotResponse> for SnapshotNodePb {
    type Error = ConversionError;

    fn try_from(from: SnapshotResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            rpc: borsh::to_vec(&from.rpc)?,
            pubkey: from.pubkey.to_bytes().to_vec(),
            snapshot_hash: borsh::to_vec(&from.snapshot_hash)?,
            latency: from.latency_microseconds,
        })
    }
}

impl TryFrom<SnapshotNodePb> for SnapshotResponse {
    type Error = ConversionError;

    fn try_from(from: SnapshotNodePb) -> Result<Self, Self::Error> {
        Ok(Self {
            rpc: borsh::from_slice(&from.rpc)?,
            pubkey: Pubkey::try_from(from.pubkey)
                .map_err(ConversionError::PubkeyDeserialization)?,
            snapshot_hash: borsh::from_slice(&from.snapshot_hash)?,
            latency_microseconds: from.latency,
        })
    }
}

impl TryFrom<Option<SnapshotResponse>> for SnapshotResponsePb {
    type Error = ConversionError;

    fn try_from(from: Option<SnapshotResponse>) -> Result<Self, Self::Error> {
        Ok(Self {
            node: from.map(TryFrom::try_from).transpose()?,
        })
    }
}

impl TryFrom<SnapshotResponsePb> for Option<SnapshotResponse> {
    type Error = ConversionError;

    fn try_from(from: SnapshotResponsePb) -> Result<Self, Self::Error> {
        from.node.map(SnapshotResponse::try_from).transpose()
    }
}

#[derive(Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct SnapshotHash {
    pub full: (u64, Hash),
    pub incr: (u64, Hash),
}

#[derive(Debug, Clone, Copy)]
pub struct ValidatorFlagsRequest {
    pub shred_version: u16,
}

impl From<ValidatorFlagsRequest> for ValidatorFlagsRequestPb {
    fn from(from: ValidatorFlagsRequest) -> Self {
        Self {
            shred_version: from.shred_version.into(),
        }
    }
}

impl TryFrom<ValidatorFlagsRequestPb> for ValidatorFlagsRequest {
    type Error = ConversionError;

    fn try_from(from: ValidatorFlagsRequestPb) -> Result<Self, Self::Error> {
        Ok(Self {
            shred_version: u16::try_from(from.shred_version)?,
        })
    }
}

impl From<ValidatorFlagsResponsePb> for Flags {
    fn from(from: ValidatorFlagsResponsePb) -> Self {
        from.flags.into()
    }
}

impl From<Flags> for ValidatorFlagsResponsePb {
    fn from(from: Flags) -> Self {
        Self { flags: from.into() }
    }
}
