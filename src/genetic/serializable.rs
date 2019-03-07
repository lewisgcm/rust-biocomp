pub trait Serializable {
    fn to_bytes(&mut self) -> &mut Vec<u8>;
}
