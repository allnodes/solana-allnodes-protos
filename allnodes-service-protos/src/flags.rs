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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let flags = Flags::from(0b1101_1010);

        // Test getting 4 bits starting from bit 0
        assert_eq!(flags.value(0, 4), 0b1010);

        // Test getting 4 bits starting from bit 2
        assert_eq!(flags.value(2, 4), 0b0110);

        // Test getting 2 bits starting from bit 5
        assert_eq!(flags.value(6, 2), 0b11);
    }

    #[test]
    fn test_bit() {
        let flags = Flags::from(0b1110_1010);

        // Test individual bits
        assert_eq!(flags.bit(0), 0);
        assert_eq!(flags.bit(1), 1);
        assert_eq!(flags.bit(2), 0);
        assert_eq!(flags.bit(3), 1);
        assert_eq!(flags.bit(4), 0);
        assert_eq!(flags.bit(5), 1);
        assert_eq!(flags.bit(6), 1);
        assert_eq!(flags.bit(7), 1);
    }

    #[test]
    fn test_is_set() {
        let flags = Flags::from(0b1110_1010);

        // Test individual bits
        assert!(!flags.is_set(0));
        assert!(flags.is_set(1));
        assert!(!flags.is_set(2));
        assert!(flags.is_set(3));
        assert!(!flags.is_set(4));
        assert!(flags.is_set(5));
        assert!(flags.is_set(6));
        assert!(flags.is_set(7));
    }

    #[test]
    fn test_edge_cases() {
        // Test with all bits set
        let all_ones = Flags::from(u64::MAX);
        assert_eq!(all_ones.value(0, 64), u64::MAX);
        assert!(all_ones.is_set(31));

        // Test with no bits set
        let all_zeros = Flags::from(0);
        assert_eq!(all_zeros.value(0, 32), 0);
        assert!(!all_zeros.is_set(0));

        // Test with only highest bit set
        let highest_bit = Flags::from(1 << 31);
        assert!(highest_bit.is_set(31));
        assert_eq!(highest_bit.value(31, 1), 1);
    }
}
