use candid::types::{Serializer, Type};
use candid::CandidType;
use ic_cdk::export::candid;
use parking_lot::MappedRwLockReadGuard;

pub struct Wrapper<T>(pub T);

impl<'a, T> CandidType for Wrapper<MappedRwLockReadGuard<'a, T>>
where
    T: CandidType,
{
    fn _ty() -> Type {
        T::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        (*self.0).idl_serialize(serializer)
    }
}
