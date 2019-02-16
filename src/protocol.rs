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


/// A random ID identifying a tracker connection
#[derive(Debug, Copy, Clone)]
pub struct ConnectionID(u64);


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
