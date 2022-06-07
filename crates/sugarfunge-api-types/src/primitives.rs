use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Seed(String);

impl From<String> for Seed {
    fn from(seed: String) -> Seed {
        Seed(seed.clone())
    }
}

impl From<&Seed> for String {
    fn from(seed: &Seed) -> String {
        seed.0.clone()
    }
}

impl Seed {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account(String);

impl From<String> for Account {
    fn from(account: String) -> Account {
        Account(account.clone())
    }
}

impl From<sp_core::crypto::AccountId32> for Account {
    fn from(account: sp_core::crypto::AccountId32) -> Account {
        Account(account.to_string())
    }
}

impl TryFrom<&Account> for sp_core::crypto::AccountId32 {
    type Error = sp_core::crypto::PublicError;

    fn try_from(
        account: &Account,
    ) -> Result<sp_core::crypto::AccountId32, sp_core::crypto::PublicError> {
        let account = sp_core::sr25519::Public::from_str(account.as_str());
        match account {
            Ok(account) => Ok(sp_core::crypto::AccountId32::from(account)),
            Err(err) => Err(err),
        }
    }
}

impl From<&Account> for String {
    fn from(account: &Account) -> String {
        account.0.clone()
    }
}

impl Account {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MarketId(u64);

impl From<u64> for MarketId {
    fn from(id: u64) -> MarketId {
        MarketId(id)
    }
}

impl From<MarketId> for u64 {
    fn from(id: MarketId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ClassId(u64);

impl From<u64> for ClassId {
    fn from(id: u64) -> ClassId {
        ClassId(id)
    }
}

impl From<ClassId> for u64 {
    fn from(id: ClassId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct AssetId(u64);

impl From<u64> for AssetId {
    fn from(id: u64) -> AssetId {
        AssetId(id)
    }
}

impl From<AssetId> for u64 {
    fn from(id: AssetId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Balance(u128);

impl From<u128> for Balance {
    fn from(id: u128) -> Balance {
        Balance(id)
    }
}

impl From<Balance> for u128 {
    fn from(id: Balance) -> u128 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Amount(i128);

impl From<i128> for Amount {
    fn from(id: i128) -> Amount {
        Amount(id)
    }
}

impl From<Amount> for i128 {
    fn from(id: Amount) -> i128 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BundleId(String);

impl From<String> for BundleId {
    fn from(bundleid: String) -> BundleId {
        BundleId(bundleid.clone())
    }
}

impl From<&BundleId> for String {
    fn from(bundleid: &BundleId) -> String {
        bundleid.0.clone()
    }
}

impl BundleId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidatorId(String);

impl From<String> for ValidatorId {
    fn from(validatorid: String) -> ValidatorId {
        ValidatorId(validatorid.clone())
    }
}

impl From<&ValidatorId> for String {
    fn from(validatorid: &ValidatorId) -> String {
        validatorid.0.clone()
    }
}

impl ValidatorId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
