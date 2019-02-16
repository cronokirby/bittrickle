/// Reads a u32 from a sequence of bytes, without checking length
/// If the length is insufficient, subsequent bytes will be 0
fn read_i32(bytes: &[u8]) -> i32 {
    let mut acc = 0;
    for &byte in &bytes[..4] {
        acc |= byte as i32;
        acc <<= 8;
    }
    acc
}

/// See `read_i32`
fn read_i64(bytes: &[u8]) -> i64 {
    let mut acc = 0;
    for &byte in &bytes[..8] {
        acc |= byte as i64;
        acc <<= 8;
    }
    acc
}


/// Represents different parse errors for the protocol
pub enum ParseError {
    /// This action was unkown
    UnknownAction,
    /// This announce event was unkown
    UnkownAnnounceEvent,
    /// The byte size for the data was insufficient
    InsufficientBytes
}


/// A specialized `Result` for ParseErrors
pub type ParseResult<T> = Result<T, ParseError>;


/// Used to communicate intent between the client and the tracker
/// The `Error` branch is removed, since it's only present in tracker responses
#[derive(Debug, Clone)]
pub enum Action {
    /// The client wishes to connect to the tracker
    Connect,
    /// The client wants announce information from the tracker
    Announce,
    /// The client wants to scrape from the tracker
    Scrape,
}

impl Action {
    fn from_i32(id: i32) -> ParseResult<Self> {
        match id {
            0 => Ok(Action::Connect),
            1 => Ok(Action::Announce),
            2 => Ok(Action::Scrape),
            _ => Err(ParseError::UnknownAction)
        }
    }
}


/// The transaction ID used by the client
#[derive(Debug, Copy, Clone)]
pub struct TransactionID(i32);


/// A random ID used to confirm the identity of the client
#[derive(Debug, Copy, Clone)]
pub struct ConnectionID(i64);


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
        let connection_id = ConnectionID(read_i64(bytes));
        let action = Action::from_i32(read_i32(&bytes[8..]))?;
        Ok(RequestHeader { connection_id, action })
    }
}


/// Represents an initial request from the client
#[derive(Debug, Clone)]
pub struct ConnectRequest {
    /// Always a magic 0x41727101980
    connection_id: ConnectionID,
    /// The transaction ID identifying this client
    transaction_id: TransactionID
}


/// Represents the tracker response for a `ConnectRequest`
#[derive(Debug, Clone)]
pub struct ConnectResponse {
    /// The transaction ID identifying the client
    transaction_id: TransactionID,
    /// The ID for this connection
    connection_id: ConnectionID
}


/// Represents the event type for an Announce
#[derive(Debug, Clone)]
pub enum AnnounceEvent {
    /// Nothing new to report
    Nothing,
    /// The client has successfully downloaded the file
    Completed,
    /// The client has started to download the file
    Started,
    /// The client has stopped downloading the file
    Stopped
}

impl AnnounceEvent {
    fn from_i32(num: i32) -> ParseResult<Self > {
        match num {
            0 => Ok(AnnounceEvent::Nothing),
            1 => Ok(AnnounceEvent::Completed),
            2 => Ok(AnnounceEvent::Started),
            3 => Ok(AnnounceEvent::Stopped),
            _ => Err(ParseError::UnkownAnnounceEvent)
        }
    }
}


#[derive(Debug, Clone)]
pub struct AnnounceRequest {
    /// The ID identifying this connection
    connection_id: ConnectionID,
    /// The ID identifying this transaction
    transaction_id: TransactionID,
    /// Any bytes are valid for the info hash
    info_hash: [u8; 20],
    /// The ID the peer wishes to use
    peer_id: [u8; 20],
    /// How many bytes the client has downloaded
    downloaded: i64,
    /// How many bytes the client has left to download
    left: i64,
    /// How many bytes the client has uploaded this session
    uploaded: i64,
    /// The event the client is reporting
    event: AnnounceEvent,
    /// The 4 byte ip address the client would like us to use
    ip: u32,
    /// A 4 byte key to help identify the user
    key: u32,
    /// The number of peers to send to the client.
    /// Negative indicates no preference
    num_want: i32,
    /// The port the client would like us to use
    port: u16,
    /// Unused extension bytes
    extensions: u16
}


/// Represents a client's request to scrape
#[derive(Debug, Clone)]
pub struct ScrapeRequest {
    /// The id identifying this connection
    connection_id: ConnectionID,
    /// The id identifying this transaction
    transaction_id: TransactionID,
    /// Info hashes to scrape
    info_hashes: Vec<[u8; 20]>
}


/// An enum for the different types of requests the client can make
#[derive(Debug, Clone)]
pub enum Request {
    ConnectRequest(ConnectRequest),
    AnnounceRequest(AnnounceRequest),
    ScrapeRequest(ScrapeRequest)
}
