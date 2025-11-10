pub mod field;
pub mod polynomial;
pub mod word;

#[allow(dead_code)]
pub trait BinaryWord: Sized {
    fn squaring(&self) -> [Self; 2];
    fn to_be_bits(&self) -> Vec<u8>;
}
