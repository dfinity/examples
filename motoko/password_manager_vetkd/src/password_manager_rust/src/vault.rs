use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

pub type PrincipalName = String;
pub type PasswordId = u128;
pub type PasswordVersion = u128;
pub type VaultId = u128;
pub type VaultVersion = u128;

#[derive(CandidType, Deserialize)]
pub enum VaultStatus {
    UpToDate,
    RequiresReencryption(Option<AccessChange>),
}

impl Storable for VaultStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize)]
pub enum AccessChange {
    AddUser(PrincipalName, AccessRights),
    RemoveUser(PrincipalName),
}

impl Storable for AccessChange {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct EncryptedPassword {
    pub owner_vault_id: VaultId,
    pub password_id: PasswordId,
    pub encrypted_text: String,
    pub password_version: PasswordVersion,
}

impl Storable for EncryptedPassword {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Default)]
pub struct Vault {
    pub id: VaultId,
    pub version: VaultVersion,
    pub passwords: Vec<PasswordId>,
    pub creator: PrincipalName,
    pub access: Vec<(PrincipalName, AccessRights)>,
}

impl Vault {
    pub fn can_read(&self, user: &PrincipalName) -> bool {
        self.creator == *user || self.access.iter().any(|(p, _a)| p == user)
    }

    pub fn can_write(&self, user: &PrincipalName) -> bool {
        self.creator == *user
            || self.access.iter().any(|(p, a)| {
                p == user
                    && match a {
                        AccessRights::ReadWrite | AccessRights::ReadWriteManage => true,
                        AccessRights::Read => false,
                    }
            })
    }

    pub fn can_manage(&self, user: &PrincipalName) -> bool {
        self.creator == *user
            || self.access.iter().any(|(p, a)| {
                p == user
                    && match a {
                        AccessRights::ReadWriteManage => true,
                        AccessRights::Read | AccessRights::ReadWrite => false,
                    }
            })
    }
}

impl Storable for Vault {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[repr(u8)]
#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum AccessRights {
    Read = 0,
    ReadWrite = 1,
    ReadWriteManage = 2,
}

impl Storable for AccessRights {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub struct VaultIds {
    pub vault_ids: Vec<VaultId>,
}

impl Storable for VaultIds {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}
