/// Used to communicate intent between the client and the tracker
#[derive(Debug, Clone)]
enum Action {
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
struct ConnectRequest {
    /// Always a magic 0x41727101980
    protocol_id: u64,
    /// For valid requests, always be `Action::Connect`
    action: Action,
    /// The transaction ID identifying this client
    transaction_id: u32
}

/// A random ID identifying a tracker connection
#[derive(Debug, Copy, Clone)]
struct ConnectionID(u64);

/// Represents the tracker response for a `ConnectRequest`
#[derive(Debug, Clone)]
struct ConnectResponse {
    /// Should always be 0
    action: Action,
    /// The transaction ID identifying the client
    transaction_id: u32,
    /// The ID for this connection
    connection_id: ConnectionID
}