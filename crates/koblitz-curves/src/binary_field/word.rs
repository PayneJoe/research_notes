#![allow(dead_code)]

use super::BinaryWord;

impl BinaryWord for u8 {
    // to little ending bits
    fn to_le_bits(&self) -> Vec<u8> {
        (0..8)
            .map(|i| if (self >> i) & 1 == 1 { 1u8 } else { 0u8 })
            .collect::<Vec<_>>()
    }

    // squaring a byte would blow up two times of its capacity
    fn squaring(&self) -> [Self; 2] {
        let mut result = [0 as u8; 2];
        // byte to bits
        let bits = self.to_le_bits();
        // insert zeros in lower byte
        for i in 0..4 {
            if bits[i] == 1u8 {
                result[0] += 1 << (2 * i);
            }
        }
        // insert zeros in higher word
        for i in 0..4 {
            if bits[4 + i] == 1u8 {
                result[1] += 1 << (2 * i);
            }
        }
        result
    }
}

impl BinaryWord for u32 {
    // to big ending bits
    fn to_le_bits(&self) -> Vec<u8> {
        (0..32)
            .map(|i| if (self >> i) & 1 == 1 { 1u8 } else { 0u8 })
            .collect::<Vec<_>>()
    }
    // squaring a byte would blow up two times of its capacity
    fn squaring(&self) -> [Self; 2] {
        let mut result = [0 as u32; 2];
        // byte to bits
        let bits = self.to_le_bits();
        // insert zeros in lower byte
        for i in 0..16 {
            if bits[i] == 1u8 {
                result[0] += 1 << (2 * i);
            }
        }
        // insert zeros in higher word
        for i in 0..16 {
            if bits[4 + i] == 1u8 {
                result[1] += 1 << (2 * i);
            }
        }
        result
    }
}

impl BinaryWord for u64 {
    // to big ending bits
    fn to_le_bits(&self) -> Vec<u8> {
        (0..64)
            .map(|i| if (self >> i) & 1 == 1 { 1u8 } else { 0u8 })
            .collect::<Vec<_>>()
    }
    // squaring a byte would blow up two times of its capacity
    fn squaring(&self) -> [Self; 2] {
        let mut result = [0 as u64; 2];
        // byte to bits
        let bits = self.to_le_bits();
        // insert zeros in lower byte
        for i in 0..32 {
            if bits[i] == 1u8 {
                result[0] += 1 << (2 * i);
            }
        }
        // insert zeros in higher word
        for i in 0..32 {
            if bits[4 + i] == 1u8 {
                result[1] += 1 << (2 * i);
            }
        }
        result
    }
}

pub type WORD8 = u8;
pub type WORD32 = u32;
pub type WORD64 = u64;
