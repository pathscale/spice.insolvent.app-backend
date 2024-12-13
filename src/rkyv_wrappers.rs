use alloy_primitives::{Address, U256};
use derive_more::derive::IntoIterator;
use rkyv::{Archive, Deserialize, Serialize};
use web3::types::H160;
use std::fmt::Debug;



#[derive(Archive, Serialize, Deserialize, IntoIterator, PartialEq, PartialOrd, Debug, Clone)]
#[rkyv(remote = Address)]
#[rkyv(archived = ArchivedAddress)]
pub struct AddressDef {
    #[rkyv(getter = get_bytes)]
    bytes: [u8; 20]
}

fn get_bytes(address: &Address) -> [u8; 20] {
    address.0.into()
}
    

impl From<AddressDef> for Address {
    fn from(value: AddressDef) -> Self {
        Address::new(value.bytes)
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
pub struct WrappedAddress {
    #[rkyv(with = AddressDef)]
    address: Address,
}

impl From<H160> for WrappedAddress {
    fn from(value: H160) -> Self {
        Self {
            address: Address::from_slice(value.as_bytes())
        }
    }
}

impl Debug for ArchivedWrappedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArchivedWrappedAddress")
            .field("address", &self.address)
            .finish()
    }
}

impl Debug for ArchivedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArchivedAddress")
            .field("address", &self.bytes)
            .finish()
    }
}


#[derive(Archive, Serialize, Deserialize, IntoIterator, PartialEq, PartialOrd, Debug, Clone)]
#[rkyv(remote = U256)]
#[rkyv(archived = ArchivedU256)]
pub struct U256Def {
    #[rkyv(getter = get_u256_bytes)]
    be_bytes: [u8; 32]
}

fn get_u256_bytes(value: &U256) -> [u8; 32] {
    value.to_be_bytes()
}
    

impl From<U256Def> for U256 {
    fn from(value: U256Def) -> Self {
        U256::from_be_bytes(value.be_bytes)
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
pub struct WrappedU256 {
    #[rkyv(with = U256Def)]
    value: U256,
}

impl From<ethers::types::U256> for WrappedU256 {
    fn from(value: ethers::types::U256) -> Self {
        let mut be_bytes = [0u8; 32];
        value.to_big_endian(&mut be_bytes);
        Self {
            value: U256::from_be_bytes(be_bytes)
        }
    }
}

impl Debug for ArchivedWrappedU256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArchivedWrappedU256")
            .field("value", &self.value)
            .finish()
    }
}

impl Debug for ArchivedU256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArchivedU256")
            .field("be_bytes", &self.be_bytes)
            .finish()
    }
}