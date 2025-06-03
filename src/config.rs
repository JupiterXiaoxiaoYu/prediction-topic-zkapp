use serde::Serialize;

lazy_static::lazy_static! {
    pub static ref ADMIN_PUBKEY: [u64; 4] = {
        let bytes = include_bytes!("./admin.pubkey");
        let u64s = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, 4) };
        u64s.try_into().unwrap()
    };
}

#[derive(Serialize, Clone)]
pub struct Config {
    actions: [&'static str; 3],
    name: [&'static str; 1],
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        actions: ["bet", "resolve", "claim"],
        name: ["prediction_market"],
    };
}

impl Config {
    pub fn to_json_string() -> String {
        serde_json::to_string(&CONFIG.clone()).unwrap()
    }

    // enable timer tick
    pub fn autotick() -> bool {
        true
    }
}

// Event types
pub const EVENT_MARKET_UPDATE: u64 = 1;
pub const EVENT_BET_UPDATE: u64 = 2;
pub const EVENT_PLAYER_UPDATE: u64 = 3;

// Market constants
pub const INITIAL_LIQUIDITY: u64 = 1000000; // Initial liquidity for AMM
pub const PLATFORM_FEE_RATE: u64 = 25; // 0.25% platform fee (25/10000)

// Default market configuration
pub struct DefaultMarketConfig {
    pub title: &'static str,
    pub description: &'static str,
    pub start_time: u64,
    pub end_time: u64,
    pub resolution_time: u64,
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_MARKET: DefaultMarketConfig = DefaultMarketConfig {
        title: "Bitcoin $100K by 2024",
        description: "Will Bitcoin reach $100,000 USD by December 31, 2024?",
        start_time: 0,      // Start immediately (counter = 0)
        end_time: 17280,    // End after 1 day (17280 ticks = 86400 seconds)
        resolution_time: 17280, // Resolution time same as end time
    };
}

// Time conversion helpers (5 seconds per tick)
pub const SECONDS_PER_TICK: u64 = 5;
pub const TICKS_PER_MINUTE: u64 = 12;
pub const TICKS_PER_HOUR: u64 = 720;
pub const TICKS_PER_DAY: u64 = 17280;

impl DefaultMarketConfig {
    /// Convert seconds to ticks
    pub fn seconds_to_ticks(seconds: u64) -> u64 {
        seconds / SECONDS_PER_TICK
    }
    
    /// Convert ticks to seconds
    pub fn ticks_to_seconds(ticks: u64) -> u64 {
        ticks * SECONDS_PER_TICK
    }
    
    /// Get market duration in ticks
    pub fn duration_ticks(&self) -> u64 {
        self.end_time - self.start_time
    }
    
    /// Get market duration in seconds
    pub fn duration_seconds(&self) -> u64 {
        Self::ticks_to_seconds(self.duration_ticks())
    }
} 