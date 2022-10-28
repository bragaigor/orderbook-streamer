/// Buffer limit of boradcast channel
pub const CHANNEL_BUFFER_LIMIT: usize = 1024;
/// Binance Web Socket URL endpoint
pub const BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
/// Bitstamp Web Socket URL endpoint
pub const BITSTAMP_WS_API: &str = "wss://ws.bitstamp.net";
/// Broadcast error count limit before we display a warning
pub const ERR_COUNT_LOG: i32 = 100;
/// Binance depth level. We set to 20
pub const DEPTH_LEVEL_BINANCE: &str = "depth20";
/// Binance web socket stream speed. We set to 100 ms
pub const UPDATE_SPEED_BINANCE: &str = "100ms";
/// Port at which our gRPC Server will be running
pub const SERVER_PORT: &str = "50505";
/// IP Address at which our gRPC Server will be running
pub const IP_ADDRESS: &str = "[::1]";
