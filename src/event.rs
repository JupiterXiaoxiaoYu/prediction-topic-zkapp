use zkwasm_rest_abi::StorageData;
use zkwasm_rest_convention::IndexedObject;
use std::convert::From;
use crate::market::MarketData;

/// External Events that are handled by external handler
pub static mut EVENTS: Vec<u64> = vec![];

pub fn clear_events(a: Vec<u64>) -> Vec<u64> {
    let mut c = a;
    unsafe {
        c.append(&mut EVENTS);
    }
    return c;
}

pub fn insert_event(typ: u64, data: &mut Vec<u64>) {
    unsafe {
        EVENTS.push((typ << 32) + data.len() as u64);
        EVENTS.append(data);
    }
}

// Event type constants for prediction market
pub const EVENT_PLAYER_UPDATE: u64 = 1;
pub const EVENT_MARKET_UPDATE: u64 = 2;
pub const EVENT_BET_UPDATE: u64 = 3;
pub const EVENT_INDEXED_OBJECT: u64 = 4;

// Market info constants for IndexedObject
pub const MARKET_INFO: u64 = 1;
pub const LIQUIDITY_HISTORY_INFO: u64 = 2;

pub struct MarketEvent {
    // Virtual liquidity for AMM pricing
    yes_liquidity: u64,
    no_liquidity: u64,
    // Market status
    resolved: u64, // 0 = false, 1 = true
    outcome: u64,  // 0 = NO won, 1 = YES won, meaningless if not resolved
}

impl StorageData for MarketEvent {
    fn from_data(u64data: &mut std::slice::IterMut<u64>) -> Self {
        MarketEvent {
            yes_liquidity: *u64data.next().unwrap(),
            no_liquidity: *u64data.next().unwrap(),
            resolved: *u64data.next().unwrap(),
            outcome: *u64data.next().unwrap(),
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.yes_liquidity);
        data.push(self.no_liquidity);
        data.push(self.resolved);
        data.push(self.outcome);
    }
}

impl From<&MarketData> for MarketEvent {
    fn from(m: &MarketData) -> MarketEvent {
        MarketEvent {
            yes_liquidity: m.yes_liquidity,
            no_liquidity: m.no_liquidity,
            resolved: if m.resolved { 1 } else { 0 },
            outcome: if m.outcome == Some(true) { 1 } else { 0 },
        }
    }
}

// New struct for historical liquidity tracking
#[derive(Debug, Clone)]
pub struct LiquidityHistoryEntry {
    pub market_id: u64,
    pub counter: u64,
    pub yes_liquidity: u64,
    pub no_liquidity: u64,
    pub total_volume: u64,
    pub action_type: u64, // 0 = creation, 1 = bet, 2 = sell, 3 = resolution
}

impl StorageData for LiquidityHistoryEntry {
    fn from_data(u64data: &mut std::slice::IterMut<u64>) -> Self {
        LiquidityHistoryEntry {
            market_id: *u64data.next().unwrap(),
            counter: *u64data.next().unwrap(),
            yes_liquidity: *u64data.next().unwrap(),
            no_liquidity: *u64data.next().unwrap(),
            total_volume: *u64data.next().unwrap(),
            action_type: *u64data.next().unwrap(),
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.market_id);
        data.push(self.counter);
        data.push(self.yes_liquidity);
        data.push(self.no_liquidity);
        data.push(self.total_volume);
        data.push(self.action_type);
    }
}

impl IndexedObject<LiquidityHistoryEntry> for LiquidityHistoryEntry {
    const PREFIX: u64 = 0x2dd4;
    const POSTFIX: u64 = 0xfee4;
    const EVENT_NAME: u64 = 0x03;
}

// Helper function to emit IndexedObject events for market data
pub fn emit_market_indexed_object(market: &MarketData, market_id: u64) {
    let mut data = Vec::new();
    data.push(MARKET_INFO); // object index
    data.push(market_id); // market ID for proper indexing
    
    // Add market data - this will be the structure stored in IndexedObject
    market.to_data(&mut data);
    
    insert_event(EVENT_INDEXED_OBJECT, &mut data);
}

// Helper function to emit liquidity history
pub fn emit_liquidity_history(market_id: u64, counter: u64, yes_liquidity: u64, no_liquidity: u64, total_volume: u64, action_type: u64) {
    let history_entry = LiquidityHistoryEntry {
        market_id,
        counter,
        yes_liquidity,
        no_liquidity,
        total_volume,
        action_type,
    };
    
    let mut data = Vec::new();
    data.push(LIQUIDITY_HISTORY_INFO); // object index
    history_entry.to_data(&mut data);
    
    insert_event(EVENT_INDEXED_OBJECT, &mut data);
}
