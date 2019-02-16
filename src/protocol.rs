/// Reads a u32 from a sequence of bytes, without checking length
/// If the length is insufficient, subsequent bytes will be 0
fn read_u32(bytes: &[u8]) -> u32 {
    let mut acc = 0;
    for &byte in &bytes[..4] {
        acc |= byte as u32;
        acc <<= 8;
    }
    acc
}

/// See `read_u32`
fn read_u64(bytes: &[u8]) -> u64 {
    let mut acc = 0;
    for &byte in &bytes[..8] {
        acc |= byte as u64;
        acc <<= 8;
    }
    acc
}


/// Represents different parse errors for the protocol
pub enum ParseError {
    /// This action was unkown
    UnknownAction,
    /// The byte size for the data was insufficient
    InsufficientBytes
}


/// A specialized `Result` for ParseErrors
pub type ParseResult<T> = Result<T, ParseError>;


/// Used to communicate intent between the client and the tracker
#[derive(Debug, Clone)]
pub enum Action {
    /// The client wishes to connect to the tracker
    Connect,
    /// The client wants announce information from the tracker
    Announce,
    /// The client wants to scrape from the tracker
    Scrape,
    /// The tracker is reporting an error back to the client
    Error
}

impl Action {
    fn from_u32(id: u32) -> ParseResult<Self> {
        match id {
            0 => Ok(Action::Connect),
            1 => Ok(Action::Announce),
            2 => Ok(Action::Scrape),
            3 => Ok(Action::Error),
            _ => Err(ParseError::UnknownAction)
        }
    }
}


/// The transaction ID used by the client
#[derive(Debug, Copy, Clone)]
pub struct TransactionID(u32);


/// A random ID used to confirm the identity of the client
#[derive(Debug, Copy, Clone)]
pub struct ConnectionID(u64);


/// Useful for identifying which request we're dealing with
#[derive(Debug, Clone)]
pub struct RequestHeader {
    connection_id: ConnectionID,
    action: Action
}

impl RequestHeader {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 12 {
            return Err(ParseError::InsufficientBytes)
        }
        let connection_id = ConnectionID(read_u64(bytes));
        let action = Action::from_u32(read_u32(&bytes[8..]))?;
        Ok(RequestHeader { connection_id, action })
    }
}


/// Represents an initial request from the client
#[derive(Debug, Clone)]
pub struct ConnectRequest {
    /// Always a magic 0x41727101980
    protocol_id: u64,
    /// The transaction ID identifying this client
    transaction_id: u32
}


/// Represents the tracker response for a `ConnectRequest`
#[derive(Debug, Clone)]
pub struct ConnectResponse {
    /// The transaction ID identifying the client
    transaction_id: u32,
    /// The ID for this connection
    connection_id: ConnectionID
}
