use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(u64);

impl From<u64> for Flags {
    fn from(value: u64) -> Self {
        Flags(value)
    }
}

impl From<Flags> for u64 {
    fn from(value: Flags) -> Self {
        value.0
    }
}

impl Flags {
    #[inline]
    pub fn value(&self, offset: usize, bits: u32) -> u64 {
        (self.0 >> offset)
            & (u64::MAX >> 64_u32.checked_sub(bits).expect("bits must not exceed 64"))
    }

    #[inline]
    pub fn bit(&self, offset: usize) -> u64 {
        (self.0 >> offset) & 1
    }

    #[inline]
    pub fn is_set(&self, bit: usize) -> bool {
        self.bit(bit) != 0
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        )
    }
}
