use ic_cdk::export::candid::{
    types::{internal::Type, Serializer},
    CandidType, Deserialize,
};
use serde::de::Deserializer;
use serde_bytes::ByteBuf;
use std::convert::AsRef;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct RcBytes(Rc<ByteBuf>);

impl CandidType for RcBytes {
    fn _ty() -> Type {
        Type::Vec(Box::new(Type::Nat8))
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_blob(&*self.0)
    }
}

impl<'de> Deserialize<'de> for RcBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ByteBuf::deserialize(deserializer).map(Self::from)
    }
}

impl From<ByteBuf> for RcBytes {
    fn from(b: ByteBuf) -> Self {
        Self(Rc::new(b))
    }
}

impl AsRef<[u8]> for RcBytes {
    fn as_ref(&self) -> &[u8] {
        &*self.0
    }
}

impl Deref for RcBytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.0
    }
}
