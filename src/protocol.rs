/// Represents different parse errors for the protocol
pub enum ParseError {
    /// This action was unkown
    UnknownAction,
    /// The byte size for these bytes wasn't correct
    BadSize {
        expected: usize,
        got: usize
    }
}


/// A specialized `Result` for ParseErrors
pub type ParseResult<T> = Result<T, ParseError>;


/// Represents types that can be parsed from bytes
pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> where Self : Sized;
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> ParseResult<u32> {
        let len = bytes.len();
        if len != 4 {
            return Err(ParseError::BadSize { expected: 4, got: len });
        }
        let mut acc = 0;
        for &byte in &bytes[..4] {
            acc |= byte as u32;
            acc <<= 8;
        }
        Ok(acc)
    }
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> ParseResult<u64> {
        let len = bytes.len();
        if len != 8 {
            return Err(ParseError::BadSize { expected: 8, got: len });
        }
        let mut acc = 0;
        for &byte in &bytes[..8] {
            acc |= byte as u64;
            acc <<= 8;
        }
        Ok(acc)
    }
}


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

impl FromBytes for Action {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Action> {
        let action_id: u32 = FromBytes::from_bytes(bytes)?;
        match action_id {
            0 => Ok(Action::Connect),
            1 => Ok(Action::Announce),
            2 => Ok(Action::Scrape),
            3 => Ok(Action::Error),
            _ => Err(ParseError::UnknownAction)
        }
    }
}


/// Represents an initial request from the client
#[derive(Debug, Clone)]
pub struct ConnectRequest {
    /// Always a magic 0x41727101980
    protocol_id: u64,
    /// For valid requests, always be `Action::Connect`
    action: Action,
    /// The transaction ID identifying this client
    transaction_id: u32
}

impl FromBytes for ConnectRequest {
    fn from_bytes(bytes: &[u8]) -> ParseResult<ConnectRequest> {
        let len = bytes.len();
        if len != 16 {
            return Err(ParseError::BadSize { expected: 16, got: len });
        }
        let protocol_id = FromBytes::from_bytes(bytes)?;
        let action = FromBytes::from_bytes(&bytes[8..])?;
        let transaction_id = FromBytes::from_bytes(&bytes[12..])?;
        Ok(ConnectRequest { protocol_id, action, transaction_id })
    }
}


/// A random ID identifying a tracker connection
#[derive(Debug, Copy, Clone)]
pub struct ConnectionID(u64);

impl FromBytes for ConnectionID {
    fn from_bytes(bytes: &[u8]) -> ParseResult<ConnectionID> {
        FromBytes::from_bytes(bytes).map(ConnectionID)
    }
}


/// Represents the tracker response for a `ConnectRequest`
#[derive(Debug, Clone)]
pub struct ConnectResponse {
    /// Should always be 0
    action: Action,
    /// The transaction ID identifying the client
    transaction_id: u32,
    /// The ID for this connection
    connection_id: ConnectionID
}
