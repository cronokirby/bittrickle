/// Reads a u32 from a sequence of bytes, without checking length
/// If the length is insufficient, subsequent bytes will be 0
fn read_i32(bytes: &[u8]) -> i32 {
    let mut acc = 0;
    for &byte in &bytes[..4] {
        acc <<= 8;
        acc |= byte as i32;
    }
    acc
}

/// See `read_i32`
fn read_i64(bytes: &[u8]) -> i64 {
    let mut acc = 0;
    for &byte in &bytes[..8] {
        acc <<= 8;
        acc |= byte as i64;
    }
    acc
}

/// See `read_i32`
fn read_u16(bytes: &[u8]) -> u16 {
    let mut acc = 0;
    for &byte in &bytes[..2] {
        acc <<= 8;
        acc |= byte as u16;
    }
    acc
}

/// See `read_i32`
fn read_u32(bytes: &[u8]) -> u32 {
    let mut acc = 0;
    for &byte in &bytes[..4] {
        acc <<= 8;
        acc |= byte as u32;
    }
    acc
}


/// Represents different parse errors for the protocol
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransactionID(i32);


/// A random ID used to confirm the identity of the client
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ConnectionID(i64);


/// Useful for identifying which request we're dealing with
#[derive(Debug, Clone)]
struct RequestHeader {
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
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectRequest {
    /// Always a magic 0x41727101980
    pub connection_id: ConnectionID,
    /// The transaction ID identifying this client
    pub transaction_id: TransactionID
}

impl ConnectRequest {
    fn from_bytes(connection_id: ConnectionID, bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 16 {
            return Err(ParseError::InsufficientBytes);
        }
        let transaction_id = TransactionID(read_i32(&bytes[12..]));
        Ok(ConnectRequest { connection_id, transaction_id })
    }
}


/// Represents the tracker response for a `ConnectRequest`
#[derive(Debug, Clone)]
pub struct ConnectResponse {
    /// The transaction ID identifying the client
    pub transaction_id: TransactionID,
    /// The ID for this connection
    pub connection_id: ConnectionID
}


/// Represents the event type for an Announce
#[derive(Debug, Clone, PartialEq)]
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


#[derive(Debug, Clone, PartialEq)]
pub struct AnnounceRequest {
    /// The ID identifying this connection
    pub connection_id: ConnectionID,
    /// The ID identifying this transaction
    pub transaction_id: TransactionID,
    /// Any bytes are valid for the info hash
    pub info_hash: [u8; 20],
    /// The ID the peer wishes to use
    pub peer_id: [u8; 20],
    /// How many bytes the client has downloaded
    pub downloaded: i64,
    /// How many bytes the client has left to download
    pub left: i64,
    /// How many bytes the client has uploaded this session
    pub uploaded: i64,
    /// The event the client is reporting
    pub event: AnnounceEvent,
    /// The 4 byte ip address the client would like us to use
    pub ip: u32,
    /// A 4 byte key to help identify the user
    pub key: u32,
    /// The number of peers to send to the client.
    /// Negative indicates no preference
    pub num_want: i32,
    /// The port the client would like us to use
    pub port: u16
}

impl AnnounceRequest {
    fn from_bytes(connection_id: ConnectionID, bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 98 {
            return Err(ParseError::InsufficientBytes);
        }
        let mut info_hash = [0; 20];
        info_hash.copy_from_slice(&bytes[16..]);
        let mut peer_id = [0; 20];
        peer_id.copy_from_slice(&bytes[36..]);
        let event = AnnounceEvent::from_i32(read_i32(&bytes[80..]))?;
        Ok(AnnounceRequest {
            connection_id,
            transaction_id: TransactionID(read_i32(&bytes[12..])),
            info_hash,
            peer_id,
            downloaded: read_i64(&bytes[56..]),
            left: read_i64(&bytes[64..]),
            uploaded: read_i64(&bytes[72..]),
            event,
            ip: read_u32(&bytes[84..]),
            key: read_u32(&bytes[88..]),
            num_want: read_i32(&bytes[92..]),
            port: read_u16(&bytes[96..])
        })
    }
}


/// Represents a client's request to scrape
#[derive(Debug, Clone, PartialEq)]
pub struct ScrapeRequest {
    /// The id identifying this connection
    pub connection_id: ConnectionID,
    /// The id identifying this transaction
    pub transaction_id: TransactionID,
    /// Info hashes to scrape
    pub info_hashes: Vec<[u8; 20]>
}

impl ScrapeRequest {
    fn from_bytes(connection_id: ConnectionID, bytes: &[u8]) -> ParseResult<Self> {
        let len = bytes.len();
        if bytes.len() < 16  || (len - 16) % 20 != 0 {
            return Err(ParseError::InsufficientBytes)
        }
        let transaction_id = TransactionID(read_i32(&bytes[12..]));
        let mut info_hashes = Vec::with_capacity((len - 16) / 20);
        let mut i = 16;
        while i < len {
            let mut hash = [0; 20];
            hash.copy_from_slice(&bytes[i..]);
            info_hashes.push(hash);
            i += 20;
        }
        Ok(ScrapeRequest { connection_id, transaction_id, info_hashes })
    }
}


/// An enum for the different types of requests the client can make
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    ConnectRequest(ConnectRequest),
    AnnounceRequest(AnnounceRequest),
    ScrapeRequest(ScrapeRequest)
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        let header = RequestHeader::from_bytes(bytes)?;
        match header.action {
            Action::Connect =>
                ConnectRequest::from_bytes(header.connection_id, bytes)
                    .map(Request::ConnectRequest),
            Action::Announce =>
                AnnounceRequest::from_bytes(header.connection_id, bytes)
                    .map(Request::AnnounceRequest),
            Action::Scrape =>
                ScrapeRequest::from_bytes(header.connection_id, bytes)
                    .map(Request::ScrapeRequest)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_connect() {
        let bytes = [
            0x00, 0x00, 0x04, 0x17, 0x27, 0x10, 0x19, 0x80,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x10
        ];
        let request = Request::from_bytes(&bytes);
        let connect_request = Request::ConnectRequest(ConnectRequest {
            connection_id: ConnectionID(0x41727101980),
            transaction_id: TransactionID(16)
        });
        assert!(request.is_ok());
        assert_eq!(request.unwrap(), connect_request);
    }
}