use crate::Flags;
use std::fmt::{Display, Formatter};

pub const CAP_MAINNET: usize = 0;
pub const CAP_GET_BOOTSTRAP_INFO: usize = 1;
pub const CAP_PROCESS_POH_CORE_CONFIG: usize = 2;
pub const CAP_RESOLVE_POH_CPU_CORE: usize = 3;
pub const CAP_HEARTBEAT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceCapabilities {
    flags: Flags,
}

impl From<u64> for ServiceCapabilities {
    fn from(capabilities: u64) -> Self {
        Self {
            flags: capabilities.into(),
        }
    }
}

impl From<ServiceCapabilities> for u64 {
    fn from(capabilities: ServiceCapabilities) -> Self {
        capabilities.flags.into()
    }
}

impl ServiceCapabilities {
    pub fn is_supported(&self, capability: usize) -> bool {
        self.flags.is_set(capability)
    }
}

impl Display for ServiceCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:016x}", u64::from(self.flags)))
    }
}
