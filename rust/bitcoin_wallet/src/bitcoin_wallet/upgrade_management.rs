use crate::types::{
    from_bitcoin_network_to_types_network, from_types_network_to_bitcoin_network,
    AddressUsingPrimitives,
};
use bitcoin::{Address, Network};
use std::str::FromStr;

/// Returns the `AddressUsingPrimitives` associated with a given `bitcoin::Address`.
pub(crate) fn get_address_using_primitives(address: &Address) -> AddressUsingPrimitives {
    (
        address.to_string(),
        from_bitcoin_network_to_types_network(address.network),
    )
}

/// Returns the `bitcoin::Address` associated with a given `AddressUsingPrimitives`.
pub(crate) fn get_address((address_string, address_network): AddressUsingPrimitives) -> Address {
    let mut address = Address::from_str(&address_string).unwrap();
    address.network = if cfg!(all(not(test), locally)) {
        Network::Regtest
    } else {
        from_types_network_to_bitcoin_network(address_network)
    };
    address
}
