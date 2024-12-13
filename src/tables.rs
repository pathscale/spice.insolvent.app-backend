use worktable::prelude::*;
use worktable::worktable;

use crate::rkyv_wrappers::WrappedAddress;
use crate::rkyv_wrappers::WrappedU256;




type TxHash = [u8; 32];
type  TransactionId = Vec<u32>;
worktable!(
    name: Block,
    columns: {
        id: u32 primary_key autoincrement,
        number: u32,
        status: u8,
        timestamp_s: u32,
        transactions: TransactionId,
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
        status: u8,
        block_number: u32,
        timestamp_s: u32,
        from_address: WrappedAddress,
        to_address: WrappedAddress optional,
        //internal_transactions: String,
        value: WrappedU256,
        fee: WrappedU256,
        gas: WrappedU256 optional,
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

