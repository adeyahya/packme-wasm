#[derive(Debug)]
pub enum Error {
    ParseError,
    InvalidInput,
    SerializeError,
    PackError,
    AllocError,
    DeallocError,
}
