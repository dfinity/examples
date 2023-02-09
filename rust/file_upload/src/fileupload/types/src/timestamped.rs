use crate::TimestampMillis;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timestamped<T> {
    pub value: T,
    pub timestamp: TimestampMillis,
}

impl<T> Timestamped<T> {
    pub fn new(value: T, now: TimestampMillis) -> Timestamped<T> {
        Timestamped {
            value,
            timestamp: now,
        }
    }

    pub fn if_set_after(&self, timestamp: TimestampMillis) -> Option<&T> {
        if self.timestamp > timestamp {
            Some(&self.value)
        } else {
            None
        }
    }
}

impl<T: Default> Default for Timestamped<T> {
    fn default() -> Self {
        Timestamped {
            value: T::default(),
            timestamp: TimestampMillis::default(),
        }
    }
}

impl<T> Deref for Timestamped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Taken from macro expansion
impl<T: CandidType> CandidType for Timestamped<T> {
    fn _ty() -> ::candid::types::Type {
        ::candid::types::Type::Record(vec![
            ::candid::types::Field {
                id: ::candid::types::Label::Named("value".to_string()),
                ty: <T as ::candid::types::CandidType>::ty(),
            },
            ::candid::types::Field {
                id: ::candid::types::Label::Named("timestamp".to_string()),
                ty: <TimestampMillis as ::candid::types::CandidType>::ty(),
            },
        ])
    }

    fn idl_serialize<__S>(&self, __serializer: __S) -> ::std::result::Result<(), __S::Error>
    where
        __S: ::candid::types::Serializer,
    {
        let mut ser = __serializer.serialize_struct()?;
        ::candid::types::Compound::serialize_element(&mut ser, &self.value)?;
        ::candid::types::Compound::serialize_element(&mut ser, &self.timestamp)?;
        Ok(())
    }
}
