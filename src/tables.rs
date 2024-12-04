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

worktable!(
    name: Transaction,
    columns: {
        id: u64 primary_key autoincrement,
        hash: String,
        status: String,
        block_number: u64,
        timestamp_s: u64,
        from_address: String,
        to_address: String,
        internal_transactions: String,
        value: u64,
        fee: u64,
        gas_price: u64
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
