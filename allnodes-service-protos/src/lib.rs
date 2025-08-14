mod error;
mod flags;

#[allow(clippy::default_trait_access)]
mod protos {
    tonic::include_proto!("allnodes.v5");
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
        SnapshotNode as SnapshotNodePb, BootstrapInfoRequest as BootstrapInfoRequestPb,
        BootstrapInfoResponse as BootstrapInfoResponsePb
    },
    server::AllnodesServiceServer as Server,
};

#[derive(Debug, Default)]
pub struct BootstrapInfoRequest {
    pub shred_version: u16,
}

impl From<&BootstrapInfoRequest> for BootstrapInfoRequestPb {
    fn from(from: &BootstrapInfoRequest) -> Self {
        Self {
            shred_version: from.shred_version.into(),
        }
    }
}

impl TryFrom<BootstrapInfoRequestPb> for BootstrapInfoRequest {
    type Error = ConversionError;

    fn try_from(from: BootstrapInfoRequestPb) -> Result<Self, Self::Error> {
        Ok(Self {
            shred_version: u16::try_from(from.shred_version)?,
        })
    }
}

#[derive(Debug)]
pub struct BootstrapInfoResponse {
    pub node: Option<BootstrapSnapshotNode>,
    pub flags: Flags,
    pub contact_info: Vec<u8>,
}

#[derive(Debug)]
pub struct BootstrapSnapshotNode {
    pub rpc: SocketAddr,
    pub pubkey: Pubkey,
    pub snapshot_hash: SnapshotHash,
    pub latency_microseconds: u64,
}

impl TryFrom<BootstrapSnapshotNode> for SnapshotNodePb {
    type Error = ConversionError;

    fn try_from(from: BootstrapSnapshotNode) -> Result<Self, Self::Error> {
        Ok(Self {
            rpc: borsh::to_vec(&from.rpc)?,
            pubkey: from.pubkey.to_bytes().to_vec(),
            snapshot_hash: borsh::to_vec(&from.snapshot_hash)?,
            latency: from.latency_microseconds,
        })
    }
}

impl TryFrom<SnapshotNodePb> for BootstrapSnapshotNode {
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

impl TryFrom<BootstrapInfoResponse> for BootstrapInfoResponsePb {
    type Error = ConversionError;

    fn try_from(from: BootstrapInfoResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            node: from.node.map(TryFrom::try_from).transpose()?,
            flags: from.flags.into(),
            contact_info: from.contact_info,
        })
    }
}

impl TryFrom<BootstrapInfoResponsePb> for BootstrapInfoResponse {
    type Error = ConversionError;

    fn try_from(from: BootstrapInfoResponsePb) -> Result<Self, Self::Error> {
        Ok(Self {
            node: from.node.map(BootstrapSnapshotNode::try_from).transpose()?,
            flags: from.flags.into(),
            contact_info: from.contact_info,
        })
    }
}

#[derive(Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct SnapshotHash {
    pub full: (u64, Hash),
    pub incr: (u64, Hash),
}
