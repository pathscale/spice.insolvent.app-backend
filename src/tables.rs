use worktable::prelude::*;
use worktable::worktable;

worktable!(
    name: Block,
    columns: {
        id: u64 primary_key autoincrement,
        number: u64,
        status: String,
        timestamp_s: u64,
        transactions: String,
        eth_price_usd_cents: u32,
    }
    indexes: {
        number_idx: number
    }
);