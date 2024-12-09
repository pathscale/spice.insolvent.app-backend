use worktable::prelude::*;
use worktable::worktable;
type Address = [u8;  20];
type TxHash = [u8; 32];
worktable!(
    name: Block,
    columns: {
        id: u32 primary_key autoincrement,
        number: u32,
        status: u8,
        timestamp_s: u32,
        transactions: String,
        eth_price_usd_cents: u32,
    }
    indexes: {
        number_idx: number
    }
);

worktable!(
    name: Transaction,
    columns: {
        id: u32 primary_key autoincrement,
        hash: TxHash,
        status: String,
        block_number: u32,
        timestamp_s: u32,
        from_address: Address,
        to_address: Address,
        internal_transactions: String,
        // Splitting U256 into four u64 parts for value
        value_high: u64,
        value_mid_high: u64,
        value_mid_low: u64,
        value_low: u64,
        // Similarly for fee and gas_price
        fee_high: u64,
        fee_mid_high: u64,
        fee_mid_low: u64,
        fee_low: u64,
        gas_price_high: u64,
        gas_price_mid_high: u64,
        gas_price_mid_low: u64,
        gas_price_low: u64,
    }
    indexes: {
        hash_idx: hash
    }
);
worktable!(
    name: Address,
    columns: {
        id: u64 primary_key autoincrement,
        hash: u64,
        address_type: String,
        type_id: u64,
        tag: String optional,
    }
    indexes: {
        hash_idx: hash
    }
);

worktable!(
    name: Wallet,
    columns: {
        id: u64 primary_key autoincrement,
        balance: u64,
        token_holdings: String optional,
        transactions: String,
    }
    indexes: {
        id_idx: id
    }
);

worktable!(
    name: Contract,
    columns: {
        id: u64 primary_key autoincrement,
        balance: u64,
        creator: String,
        tracker: String optional,
        code: String optional,
        transactions: String,
    }
    indexes: {
        id_idx: id
    }
);

worktable!(
    name: Token,
    columns: {
        id: u64 primary_key autoincrement,
        token_contract_hash: String,
        max_supply: u64,
        price_usd: f64,
        onchain_cap: u64,
        circulating_cap: u64,
        transfers: String,
        holders: String,
    }
    indexes: {
        token_contract_hash_idx: token_contract_hash
    }
);

