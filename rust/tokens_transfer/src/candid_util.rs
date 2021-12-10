use std::path::Path;
use candid::types::subtype::{Gamma, subtype};
use candid::types::Type;

pub fn check_candid_interface_compatibility(expected: &Path, actual: &Path) {
    let (mut env1, t1) =
        candid::pretty_check_file(actual).expect("failed to check generated candid interface");
    let (env2, t2) =
        candid::pretty_check_file(expected).expect(&format!("failed to open expected candid file {:?}", expected));

    let (t1_ref, t2) = match (t1.as_ref().unwrap(), t2.unwrap()) {
        (Type::Class(_, s1), Type::Class(_, s2)) => (s1.as_ref(), *s2),
        (Type::Class(_, s1), s2 @ Type::Service(_)) => (s1.as_ref(), s2),
        (s1 @ Type::Service(_), Type::Class(_, s2)) => (s1, *s2),
        (t1, t2) => (t1, t2),
    };

    let mut gamma = Gamma::new();
    let t2 = env1.merge_type(env2, t2);
    let mk_error = || {
        std::fs::read_to_string(actual).map(|actual_interface|
            format!("ledger canister interface is not compatible with \
                  the ledger.did file. Generated candid interface:\n{}\n\n",
                    actual_interface))
            .unwrap_or(format!("Unable to read actual candid interface from file {:?}", actual))
    };
    subtype(&mut gamma, &env1, t1_ref, &t2).unwrap_or_else(|_| panic!("{}", mk_error()));
}
