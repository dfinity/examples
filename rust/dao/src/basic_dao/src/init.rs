use crate::env::CanisterEnvironment;
use crate::SERVICE;
use crate::service::BasicDaoService;
use ic_cdk_macros::init;
use crate::types::BasicDaoStableStorage;

#[init]
fn init(init_state: BasicDaoStableStorage) {
    ic_cdk::setup();

    let mut init_service = BasicDaoService::from(init_state);
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
