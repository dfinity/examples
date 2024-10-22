use assert_matches::assert_matches;
use candid::Principal;
use candid::{Decode, Encode};
use ic_cdk::api::management_canister::main::CanisterId;
use ic_vetkd_utils::TransportSecretKey;
use password_manager_rust::vault::{
    AccessRights, EncryptedPassword, PasswordId, PasswordVersion, VaultId, VaultVersion,
};
use password_manager_rust::vetkd_api_types::{VetKDPublicKeyReply, VetKDPublicKeyRequest};
use pocket_ic::{PocketIc, PocketIcBuilder, UserError};

mod smoke {
    use super::*;
    use ic_cdk::api::management_canister::main::CanisterStatusType;
    use password_manager_rust::utils::bls12_381_test_key_1;

    #[test]
    fn vetkd_canister_exists() {
        let pic = pic_with_vetkd();
        let status = pic
            .canister_status(vetkd_system_api_canister_id(), None)
            .expect("failed to get canister status");
        assert_eq!(status.status, CanisterStatusType::Running);
    }

    #[test]
    fn vetkd_api_is_accessible() {
        let pic = pic_with_vetkd();

        let request = VetKDPublicKeyRequest {
            canister_id: None,
            derivation_path: derivation_path(),
            key_id: bls12_381_test_key_1(),
        };

        let reply = pic
            .update_call(
                vetkd_system_api_canister_id(),
                Principal::anonymous(),
                "vetkd_public_key",
                Encode!(&request).unwrap(),
            )
            .expect("failed to call vetkd_public_key");
        let typed_reply = match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), VetKDPublicKeyReply).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("vetkd_public_key call was rejected"),
        };
        assert_eq!(typed_reply.public_key.len(), 96);
    }
}

mod password_manager {
    use super::*;
    use password_manager_rust::vault::{PasswordId, PasswordVersion, VaultId};

    const DUMMY_PASSWORD: &str = "Hello, password!";

    mod key_management {
        use super::*;

        #[test]
        fn can_get_public_key_for_vault() {
            let test = Test::new();
            assert_matches!(test.get_verification_key(), s if s.len() == 2 * 96);
        }

        #[test]
        fn can_get_encrypted_key_for_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let encrypted_symmetric_key = test
                .get_encrypted_symmetric_key(
                    principal_1(),
                    vault_id,
                    dummy_transport_key().public_key(),
                )
                .expect("failed to get encrypted symmetric key");
            assert_eq!(encrypted_symmetric_key.0, 0);
            const EXPECTED_KEY_BYTE_LENGTH: usize = 192;
            assert_eq!(
                encrypted_symmetric_key.1.len(),
                2 * EXPECTED_KEY_BYTE_LENGTH
            )
        }

        #[test]
        fn user_gets_same_encryption_key_for_vault_with_same_version_in_different_calls() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let transport_key = dummy_transport_key();
            let derived_public_key_bytes = hex::decode(&test.get_verification_key())
                .expect("failed to decode verification key");
            // the encryption key is also verified for validity in `test.get_symmetric_encryption_key`
            assert_eq!(
                test.get_symmetric_encryption_key(
                    principal_1(),
                    vault_id,
                    derived_public_key_bytes.as_slice(),
                    &transport_key
                )
                .expect("failed to get encrypted symmetric key"),
                test.get_symmetric_encryption_key(
                    principal_1(),
                    vault_id,
                    derived_public_key_bytes.as_slice(),
                    &transport_key
                )
                .expect("failed to get encrypted symmetric key")
            )
        }

        #[test]
        fn user_gets_different_encryption_keys_for_different_vault_versions() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let transport_key = dummy_transport_key();
            let derived_public_key_bytes = hex::decode(&test.get_verification_key())
                .expect("failed to decode verification key");
            for _iteration in 0..3 {
                let (vault_version_1, encryption_key_1, derivation_id_1) = test
                    .get_symmetric_encryption_key(
                        principal_1(),
                        vault_id,
                        derived_public_key_bytes.as_slice(),
                        &transport_key,
                    )
                    .expect("failed to get encrypted symmetric key");
                test.rotate_vault_key(principal_1(), vault_id)
                    .expect("failed to rotate vault key");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                let (vault_version_2, encryption_key_2, derivation_id_2) = test
                    .get_symmetric_encryption_key(
                        principal_1(),
                        vault_id,
                        derived_public_key_bytes.as_slice(),
                        &transport_key,
                    )
                    .expect("failed to get encrypted symmetric key");

                assert_eq!(vault_version_1 + 1, vault_version_2);
                assert_ne!(encryption_key_1, encryption_key_2);
                assert_ne!(derivation_id_1, derivation_id_2);
            }
        }

        #[test]
        fn different_users_get_same_encryption_key_for_same_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.add_user(principal_1(), vault_id, principal_2(), AccessRights::Read)
                .expect("failed to add user");
            test.dummy_reencrypt_vault(principal_1(), vault_id)
                .expect("reencrypt vault");
            let transport_key = dummy_transport_key();
            let derived_public_key_bytes = hex::decode(&test.get_verification_key())
                .expect("failed to decode verification key");
            assert_eq!(
                test.get_symmetric_encryption_key(
                    principal_1(),
                    vault_id,
                    derived_public_key_bytes.as_slice(),
                    &transport_key
                )
                .expect("failed to get encrypted symmetric key"),
                test.get_symmetric_encryption_key(
                    principal_2(),
                    vault_id,
                    derived_public_key_bytes.as_slice(),
                    &transport_key
                )
                .expect("failed to get encrypted symmetric key")
            )
        }

        #[test]
        fn fails_to_get_encryption_key_if_user_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let result = test.get_encrypted_symmetric_key(
                principal_2(),
                vault_id,
                dummy_transport_key().public_key(),
            );
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }
    }

    mod vault {
        use super::*;

        #[test]
        fn can_create_vault() {
            let test = Test::new();

            const NUM_VAULTS: usize = 3;
            for i in 0..NUM_VAULTS {
                let vault_id = test.create_vault(principal_1());
                assert_eq!(vault_id, i as VaultId);
            }
        }

        #[test]
        fn can_get_vault() {
            let test = Test::new();

            const NUM_VAULTS: usize = 3;
            for _ in 0..NUM_VAULTS {
                let vault_id = test.create_vault(principal_1());
                assert_eq!(
                    test.get_vault(principal_1(), vault_id),
                    Ok((0 as VaultId, vec![]))
                );
            }
        }

        #[test]
        fn can_get_accessible_vault_ids() {
            let test = Test::new();
            assert_eq!(
                test.get_accessible_vault_ids(principal_1()),
                Vec::<u128>::new()
            );
            test.create_vault(principal_1());
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0]);
            test.create_vault(principal_2());
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0]);
            test.create_vault(principal_1());
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2]);

            test.add_user(principal_2(), 1, principal_1(), AccessRights::Read)
                .expect("failed to add user");
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2]);
            test.reencrypt_vault(principal_2(), 1, 0, vec![])
                .expect("failed to reencrypt vault");
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2, 1]);

            test.create_vault(principal_1());
            assert_eq!(
                test.get_accessible_vault_ids(principal_1()),
                vec![0, 2, 1, 3]
            );

            test.remove_user(principal_2(), 1, principal_1())
                .expect("failed to remove user");
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2, 3]);
            test.reencrypt_vault(principal_2(), 1, 1, vec![])
                .expect("failed to reencrypt vault");
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2, 3]);

            test.add_user(principal_2(), 1, principal_1(), AccessRights::Read)
                .expect("failed to add user");
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2, 3]);
            test.reencrypt_vault(principal_2(), 1, 2, vec![])
                .expect("failed to reencrypt vault");
            assert_eq!(
                test.get_accessible_vault_ids(principal_1()),
                vec![0, 2, 3, 1]
            );

            assert_eq!(test.get_accessible_vault_ids(principal_2()), vec![1]);
            test.delete_vault(principal_2(), 1)
                .expect("failed to delete vault");
            assert_eq!(
                test.get_accessible_vault_ids(principal_2()),
                Vec::<u128>::new()
            );
            assert_eq!(test.get_accessible_vault_ids(principal_1()), vec![0, 2, 3]);
        }

        #[test]
        fn fails_to_get_vault_if_user_unknown() {
            let test = Test::new();

            let vault_id = test.create_vault(principal_1());
            assert_matches!(
                test.get_vault(principal_2(), vault_id),
                Err(e) if
                e.description
                .contains("unauthorized user")
            );
        }

        #[test]
        fn fails_to_get_vault_if_user_known_but_not_authorized() {
            let test = Test::new();
            // make sure the user is known to the canister
            let _ = test.create_vault(principal_2());

            let vault_id = test.create_vault(principal_1());
            assert_matches!(
                test.get_vault(principal_2(), vault_id),
                Err(e) if
                e.description
                .contains("unauthorized user")
            );
        }

        #[test]
        fn can_delete_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            assert_eq!(
                test.get_vault(principal_1(), vault_id),
                Ok((0 as VaultId, vec![]))
            );
            assert_eq!(test.delete_vault(principal_1(), vault_id), Ok(()));
            assert_matches!(test
            .get_vault(principal_1(), vault_id),
            Err(e) if
            e.description
            .contains("vault_id does not exist"));
            assert_matches!(test
            .delete_vault(principal_1(), vault_id),
            Err(e) if
            e.description
            .contains("vault_id does not exist"));
        }

        #[test]
        fn fails_to_delete_vault_if_user_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let result = test.delete_vault(principal_2(), vault_id);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
            let _ = test.create_vault(principal_2());
            let result = test.delete_vault(principal_2(), vault_id);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }

        #[test]
        fn deleted_vault_does_not_decrement_vault_id_counter() {
            let test = Test::new();
            let vault_id_0 = test.create_vault(principal_1());
            assert_eq!(
                test.get_vault(principal_1(), vault_id_0),
                Ok((0 as VaultId, vec![]))
            );
            test.delete_vault(principal_1(), vault_id_0)
                .expect("could not delete vault");

            let vault_id_1 = test.create_vault(principal_1());
            assert_eq!(vault_id_1, 1);
        }

        #[test]
        fn can_reencrypt_empty_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            assert_eq!(test.rotate_vault_key(principal_1(), vault_id), Ok(()));
            let result = test.reencrypt_vault(principal_1(), vault_id, 0, vec![]);
            assert_eq!(result, Ok(()));
        }

        #[test]
        fn can_reencrypt_non_empty_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());

            for _num_passwords in 1..=3 {
                let password_id = test
                    .create_password(principal_1(), vault_id)
                    .expect("failed to create password");
                test.update_password(principal_1(), password_id, 0, DUMMY_PASSWORD)
                    .expect("failed to update password");
                let (vault_version, mut encrypted_passwords) = test
                    .get_vault(principal_1(), vault_id)
                    .expect("failed to get vault");

                for encrypted_password in encrypted_passwords.iter_mut() {
                    encrypted_password.encrypted_text.push_str(" reencrypted");
                }

                assert_eq!(test.rotate_vault_key(principal_1(), vault_id), Ok(()));
                let result = test.reencrypt_vault(
                    principal_1(),
                    vault_id,
                    vault_version,
                    encrypted_passwords
                        .iter()
                        .map(|p| p.encrypted_text.clone())
                        .collect(),
                );
                assert_eq!(result, Ok(()));

                for encrypted_password in encrypted_passwords.iter_mut() {
                    encrypted_password.password_version += 1;
                }

                let computed_encrypted_passwords: Vec<_> = encrypted_passwords
                    .iter()
                    .map(|p| {
                        test.get_password(principal_1(), p.password_id)
                            .expect("failed to get password")
                    })
                    .collect();

                assert_eq!(computed_encrypted_passwords, encrypted_passwords);
            }
        }

        #[test]
        fn another_user_can_finish_vault_reencryption() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.add_user(
                principal_1(),
                vault_id,
                principal_2(),
                AccessRights::ReadWriteManage,
            )
            .expect("failed to add user");
            test.dummy_reencrypt_vault(principal_1(), vault_id)
                .expect("failed to reencrypt vault");

            for (alice, bob) in [
                (principal_1(), principal_2()),
                (principal_2(), principal_1()),
            ] {
                test.add_user(alice, vault_id, principal_3(), AccessRights::Read)
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(bob, vault_id)
                    .expect("failed to reencrypt vault");
                test.remove_user(alice, vault_id, principal_3())
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(bob, vault_id)
                    .expect("failed to reencrypt vault");
            }
        }

        #[test]
        fn fails_to_reencrypt_vault_with_wrong_vault_version() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.rotate_vault_key(principal_1(), vault_id)
                .expect("failed to rotate vault key");
            assert_matches!(
                test.reencrypt_vault(principal_1(), vault_id, 1, vec![]),
                Err(e) if e.description.contains("vault version mismatch")
            );
        }

        #[test]
        fn fails_to_reencrypt_vault_if_user_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.rotate_vault_key(principal_1(), vault_id)
                .expect("failed to rotate vault key");
            assert_matches!(
                test.reencrypt_vault(principal_2(), vault_id, 0, vec![]),
                Err(e) if e.description.contains("unauthorized user")
            );
        }

        #[test]
        fn fails_to_reencrypt_vault_if_vault_not_locked() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            assert_matches!(
                test.reencrypt_vault(principal_1(), vault_id, 0, vec![]),
                Err(e) if e.description.contains("The vault is NOT locked")
            );
        }
    }

    mod access_control {
        use super::*;

        #[test]
        fn can_add_user_to_vault() {
            let test = Test::new();
            for access_rights in [
                AccessRights::Read,
                AccessRights::ReadWrite,
                AccessRights::ReadWriteManage,
            ] {
                let vault_id = test.create_vault(principal_1());
                test.add_user(principal_1(), vault_id, principal_2(), access_rights)
                    .expect("failed to add user");
            }
        }

        #[test]
        fn cannot_add_owner_as_user_to_vault() {
            let test = Test::new();
            for access_rights in [
                AccessRights::Read,
                AccessRights::ReadWrite,
                AccessRights::ReadWriteManage,
            ] {
                let vault_id = test.create_vault(principal_1());
                assert_matches!(
                    test.add_user(principal_1(), vault_id, principal_1(), access_rights),
                    Err(e) if e.description.contains("user already has access to the vault")
                );
            }
        }

        #[test]
        fn added_user_can_always_read_vault() {
            let test = Test::new();
            for access_rights in [
                AccessRights::Read,
                AccessRights::ReadWrite,
                AccessRights::ReadWriteManage,
            ] {
                let vault_id = test.create_vault(principal_1());
                assert_matches!(test.get_vault(principal_2(), vault_id), Err(e) if e.description.contains("unauthorized user"));
                test.add_user(principal_1(), vault_id, principal_2(), access_rights)
                    .expect("failed to add user");
                assert_matches!(test.get_vault(principal_2(), vault_id), Err(e) if e.description.contains("unauthorized user"));
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                assert_matches!(test.get_vault(principal_2(), vault_id), Ok(_));
            }
        }

        #[test]
        fn can_remove_user_from_vault() {
            let test = Test::new();
            for access_rights in [
                AccessRights::Read,
                AccessRights::ReadWrite,
                AccessRights::ReadWriteManage,
            ] {
                let vault_id = test.create_vault(principal_1());
                assert_matches!(test.get_vault(principal_2(), vault_id), Err(e) if e.description.contains("unauthorized user"));
                test.add_user(principal_1(), vault_id, principal_2(), access_rights)
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                assert_matches!(test.get_vault(principal_2(), vault_id), Ok(_));
                assert_eq!(
                    test.remove_user(principal_1(), vault_id, principal_2()),
                    Ok(())
                );
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                assert_matches!(test.get_vault(principal_2(), vault_id), Err(e) if e.description.contains("unauthorized user"));
                assert_matches!(
                    test.remove_user(principal_1(), vault_id, principal_2()),
                    Err(e) if e.description.contains("to-be-removed user does not have access to the vault")
                );
                assert_matches!(test.get_vault(principal_2(), vault_id), Err(e) if e.description.contains("unauthorized user"));
            }
        }

        #[test]
        fn fails_to_remove_owner_from_vault() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.add_user(
                principal_1(),
                vault_id,
                principal_2(),
                AccessRights::ReadWriteManage,
            )
            .expect("failed to add user");
            test.dummy_reencrypt_vault(principal_1(), vault_id)
                .expect("failed to reencrypt vault");

            assert_matches!(
                test.remove_user(principal_1(), vault_id, principal_1()),
                Err(e) if e.description.contains("cannot remove the creator of the vault")
            );
            assert_matches!(
                test.remove_user(principal_2(), vault_id, principal_1()),
                Err(e) if e.description.contains("cannot remove the creator of the vault")
            );
        }

        #[test]
        fn added_user_with_write_access_can_update_password() {
            let test = Test::new();
            for access_rights in [AccessRights::ReadWrite, AccessRights::ReadWriteManage] {
                let vault_id = test.create_vault(principal_1());
                test.add_user(principal_1(), vault_id, principal_2(), access_rights)
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                let password_id = test
                    .create_password(principal_1(), vault_id)
                    .expect("failed to create password");
                assert_matches!(
                    test.update_password(principal_2(), password_id, 0, DUMMY_PASSWORD),
                    Ok(())
                );
                let encrypted_password = test
                    .get_password(principal_1(), password_id)
                    .expect("failed to get password");
                assert_eq!(encrypted_password.encrypted_text, DUMMY_PASSWORD);
            }
        }

        #[test]
        fn fails_to_change_access_rights_if_user_not_authorized() {
            let test = Test::new();
            for access_rights in [AccessRights::Read, AccessRights::ReadWrite] {
                let vault_id = test.create_vault(principal_1());
                test.add_user(principal_1(), vault_id, principal_2(), access_rights)
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                assert_matches!(test.add_user(principal_2(), vault_id, principal_3(), AccessRights::Read), Err(e) if e.description.contains("unauthorized user"));
                test.add_user(principal_1(), vault_id, principal_3(), access_rights)
                    .expect("failed to add user");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                assert_matches!(test.remove_user(principal_2(), vault_id, principal_3()), Err(e) if e.description.contains("unauthorized user"));
                assert_matches!(test.remove_user(principal_2(), vault_id, principal_2()), Err(e) if e.description.contains("unauthorized user"));
            }
        }

        #[test]
        fn fails_to_add_same_user_twice() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.add_user(principal_1(), vault_id, principal_2(), AccessRights::Read)
                .expect("failed to add user");
            test.dummy_reencrypt_vault(principal_1(), vault_id)
                .expect("failed to reencrypt vault");
            assert_matches!(
                test.add_user(principal_1(), vault_id, principal_2(), AccessRights::Read),
                Err(e) if e.description.contains("user already has access to the vault")
            );
        }

        #[test]
        fn access_rights_change_increments_vault_version_after_reencrypt() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            for current_vault_version in 0..3 {
                test.add_user(principal_1(), vault_id, principal_2(), AccessRights::Read)
                    .expect("failed to add user");
                test.reencrypt_vault(principal_1(), vault_id, 2 * current_vault_version, vec![])
                    .expect("failed to reencrypt vault");
                test.remove_user(principal_1(), vault_id, principal_2())
                    .expect("failed to remove user");
                test.reencrypt_vault(
                    principal_1(),
                    vault_id,
                    2 * current_vault_version + 1,
                    vec![],
                )
                .expect("failed to reencrypt vault");
            }
        }

        #[test]
        fn fails_to_change_access_rights_twice_in_a_row_without_vault_reencryption() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            test.add_user(principal_1(), vault_id, principal_2(), AccessRights::Read)
                .expect("failed to add user");
            assert_matches!(
                test.add_user(principal_1(), vault_id, principal_3(), AccessRights::Read),
                Err(e) if e.description.contains("vault is already locked for reencryption")
            );
        }
    }

    mod password {
        use super::*;

        #[test]
        fn can_create_password() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());

            const NUM_PASSWORDS: usize = 3;

            for i in 0..NUM_PASSWORDS {
                let password_id = test.create_password(principal_1(), vault_id);
                assert_eq!(password_id, Ok(i as VaultId));
            }
        }

        #[test]
        fn fails_to_create_password_if_user_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let result = test.create_password(principal_2(), vault_id);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }

        #[test]
        fn can_update_password() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");

            const DUMMY_PASSWORD: &str = "Hello, password!";

            let current_password_version: PasswordVersion = 0;
            let dummy_encrypted_text: String = DUMMY_PASSWORD.to_string();

            test.update_password(
                principal_1(),
                password_id,
                current_password_version,
                dummy_encrypted_text.as_str(),
            )
            .expect("failed to update password");

            let encrypted_password = test
                .get_password(principal_1(), password_id)
                .expect("failed to get password");

            assert_eq!(encrypted_password.encrypted_text, DUMMY_PASSWORD);
            assert_eq!(encrypted_password.password_version, 1);
        }

        #[test]
        fn fails_to_update_password_if_user_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");
            let result = test.update_password(principal_2(), password_id, 0, DUMMY_PASSWORD);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }

        #[test]
        fn updating_password_with_wrong_version_fails() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");

            let wrong_password_version: PasswordVersion = 1;

            let result = test.update_password(
                principal_1(),
                password_id,
                wrong_password_version,
                DUMMY_PASSWORD,
            );

            assert_matches!(result,
                Err(e) if
                e.description.contains("password version mismatch, someone likely has modified the password in the meantime.")
            );
        }

        #[test]
        fn can_get_password() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());

            const NUM_PASSWORDS: usize = 3;

            for i in 0..NUM_PASSWORDS {
                let password_id = test
                    .create_password(principal_1(), vault_id)
                    .expect("failed to create password");
                assert_eq!(password_id, i as VaultId);
            }

            for i in 0..NUM_PASSWORDS {
                let encrypted_password = test
                    .get_password(principal_1(), i as PasswordId)
                    .expect("failed to get password");
                assert_eq!(encrypted_password.encrypted_text, String::new());
                assert_eq!(encrypted_password.owner_vault_id, vault_id);
                assert_eq!(encrypted_password.password_version, 0);
            }
        }

        #[test]
        fn fails_to_get_password_if_user_unknown() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");

            let result = test.get_password(principal_2(), password_id);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }

        #[test]
        fn fails_to_get_password_if_user_known_but_not_authorized() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");

            // Create a second vault and password to make sure that canister
            // know this `principal_2`
            test.create_password(principal_2(), test.create_vault(principal_2()))
                .expect("failed to create password");

            let result = test.get_password(principal_2(), password_id);
            assert_matches!(result, Err(e) if e.description.contains("unauthorized user"));
        }

        #[test]
        fn fails_to_get_password_if_password_id_does_not_exist() {
            let test = Test::new();
            assert_matches!(
                test.get_password(principal_1(), 0),
                Err(e) if e.description.contains("password_id does not exist")
            );
            let vault_id = test.create_vault(principal_1());
            let _ = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");
            assert_matches!(
                test.get_password(principal_1(), 1),
                Err(e) if e.description.contains("password_id does not exist")
            );
        }

        #[test]
        fn can_delete_password() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");
            let _ = test
                .get_password(principal_1(), password_id)
                .expect("failed to get password");
            test.delete_password(principal_1(), password_id)
                .expect("failed to delete password");
        }

        #[test]
        fn deleted_password_does_not_decrement_vault_id_counter() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            for _iteration in 0..3 {
                let password_id_1 = test
                    .create_password(principal_1(), vault_id)
                    .expect("failed to create password");
                let password_1 = test
                    .get_password(principal_1(), password_id_1)
                    .expect("failed to get password");
                test.delete_password(principal_1(), password_id_1)
                    .expect("failed to delete password");
                let password_id_2 = test
                    .create_password(principal_1(), vault_id)
                    .expect("failed to create password");
                let password_2 = test
                    .get_password(principal_1(), password_id_2)
                    .expect("failed to get password");
                assert_eq!(password_1.password_id + 1, password_2.password_id);
            }
        }

        #[test]
        fn vault_reencryption_increments_password_version() {
            let test = Test::new();
            let vault_id = test.create_vault(principal_1());
            let password_id = test
                .create_password(principal_1(), vault_id)
                .expect("failed to create password");
            for _iteration in 0..3 {
                let encrypted_password = test
                    .get_password(principal_1(), password_id)
                    .expect("failed to get password");
                test.rotate_vault_key(principal_1(), vault_id)
                    .expect("failed to rotate vault key");
                test.dummy_reencrypt_vault(principal_1(), vault_id)
                    .expect("failed to reencrypt vault");
                let encrypted_password_reencrypted = test
                    .get_password(principal_1(), password_id)
                    .expect("failed to get password");
                assert_eq!(
                    encrypted_password_reencrypted.password_version,
                    encrypted_password.password_version + 1
                );
            }
        }
    }
}

fn pic_with_vetkd() -> PocketIc {
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_nns_subnet()
        .with_ii_subnet()
        .build();
    install_vetkd_mock(&pic);
    pic
}

fn pic_with_password_manager() -> (CanisterId, PocketIc) {
    let pic = pic_with_vetkd();
    (install_password_manager(&pic), pic)
}

fn install_vetkd_mock(pic: &PocketIc) {
    let canister_id = vetkd_system_api_canister_id();
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("failed to read environment variable CARGO_MANIFEST_DIR");
    let wasm_path = std::format!("{cargo_dir}/../../vetkd_system_api.wasm");
    let wasm_module = std::fs::read(wasm_path).expect("failed to read vetkd_system_api.wasm");

    pic.create_canister_with_id(None, None, canister_id)
        .expect("failed to create canister");
    pic.add_cycles(canister_id, 2_000_000_000_000);
    pic.install_canister(canister_id, wasm_module, vec![], None);
}

fn install_password_manager(pic: &PocketIc) -> CanisterId {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("failed to read environment variable CARGO_MANIFEST_DIR");
    let wasm_path: String = std::format!(
        "{cargo_dir}/../../target/wasm32-unknown-unknown/release/password_manager_rust.wasm"
    );
    println!("wasm_path: {}", wasm_path);
    let wasm_module = std::fs::read(wasm_path).expect("failed to read password_manager_rust.wasm");

    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    pic.install_canister(canister_id, wasm_module, vec![], None);

    canister_id
}

fn vetkd_system_api_canister_id() -> CanisterId {
    CanisterId::from_text("s55qq-oqaaa-aaaaa-aaakq-cai").unwrap()
}

fn derivation_path() -> Vec<Vec<u8>> {
    vec![b"test_application".to_vec()]
}

pub const PUBKEY_1: &str = "test_1";
pub const PUBKEY_2: &str = "test_2";
pub const PUBKEY_3: &str = "test_3";

fn principal_1() -> Principal {
    Principal::self_authenticating(PUBKEY_1)
}

fn principal_2() -> Principal {
    Principal::self_authenticating(PUBKEY_2)
}

fn principal_3() -> Principal {
    Principal::self_authenticating(PUBKEY_3)
}

fn dummy_transport_key() -> TransportSecretKey {
    TransportSecretKey::from_seed(vec![0; 32]).unwrap()
}

struct Test {
    pic: PocketIc,
    password_manager_canister_id: CanisterId,
}

impl Test {
    fn new() -> Self {
        let (password_manager_canister_id, pic) = pic_with_password_manager();
        Self {
            pic,
            password_manager_canister_id,
        }
    }

    fn create_vault(&self, caller: Principal) -> VaultId {
        let reply = self
            .pic
            .update_call(
                self.password_manager_canister_id,
                caller,
                "create_vault",
                Encode!(&()).unwrap(),
            )
            .expect("failed to create_vault");
        match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), VaultId).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("create_vault call was rejected"),
        }
    }

    fn get_vault(
        &self,
        caller: Principal,
        vault_id: VaultId,
    ) -> Result<(VaultVersion, Vec<EncryptedPassword>), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "get_vault",
            Encode!(&(vault_id)).unwrap(),
        )?;

        let result = match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), VaultVersion, Vec<EncryptedPassword>).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("get_vault call was rejected"),
        };
        Ok(result)
    }

    fn delete_vault(&self, caller: Principal, vault_id: VaultId) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "delete_vault",
            Encode!(&vault_id).unwrap(),
        )?;
        match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("delete_vault call was rejected"),
        };
        Ok(())
    }

    fn reencrypt_vault(
        &self,
        caller: Principal,
        vault_id: VaultId,
        current_vault_version: VaultVersion,
        reencrypted_passwords: Vec<String>,
    ) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "reencrypt_vault",
            Encode!(&vault_id, &current_vault_version, &reencrypted_passwords).unwrap(),
        )?;

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("reencrypt_vault call was rejected"),
        };
        Ok(())
    }

    fn create_password(
        &self,
        caller: Principal,
        vault_id: VaultId,
    ) -> Result<PasswordId, UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "create_password",
            Encode!(&(vault_id)).unwrap(),
        )?;

        let result = match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), VaultId).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("create_password call was rejected"),
        };
        Ok(result)
    }

    fn update_password(
        &self,
        caller: Principal,
        password_id: PasswordId,
        current_password_version: PasswordVersion,
        encrypted_text: &str,
    ) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "update_password",
            Encode!(&password_id, &current_password_version, &encrypted_text).unwrap(),
        )?;

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), ()).unwrap();
            }
            pocket_ic::WasmResult::Reject(_) => panic!("update_password call was rejected"),
        };
        Ok(())
    }

    fn get_password(
        &self,
        caller: Principal,
        password_id: PasswordId,
    ) -> Result<EncryptedPassword, UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "get_password",
            Encode!(&(password_id as PasswordId)).unwrap(),
        )?;

        let result = match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), EncryptedPassword).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("get_password call was rejected"),
        };
        Ok(result)
    }

    fn delete_password(&self, caller: Principal, password_id: PasswordId) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "delete_password",
            Encode!(&(password_id as PasswordId)).unwrap(),
        )?;

        let result = match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("get_password call was rejected"),
        };
        Ok(result)
    }

    fn get_verification_key(&self) -> String {
        let reply = self
            .pic
            .update_call(
                self.password_manager_canister_id,
                Principal::anonymous(),
                "verification_key_for_vault",
                Encode!(&()).unwrap(),
            )
            .expect("failed to get_verification_key");

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), String).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => panic!("get_verification_key call was rejected"),
        }
    }

    fn get_encrypted_symmetric_key(
        &self,
        caller: Principal,
        vault_id: VaultId,
        encryption_public_key: Vec<u8>,
    ) -> Result<(VaultVersion, String, Vec<u8>), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "encrypted_symmetric_key_for_vault",
            Encode!(&vault_id, &encryption_public_key).unwrap(),
        )?;

        let result = match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), VaultVersion, String, Vec<u8>).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => {
                panic!("get_encrypted_symmetric_key call was rejected")
            }
        };
        Ok(result)
    }

    fn add_user(
        &self,
        caller: Principal,
        vault_id: VaultId,
        user: Principal,
        access_rights: AccessRights,
    ) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "add_user",
            Encode!(&vault_id, &user.to_text(), &access_rights).unwrap(),
        )?;

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("add_user call was rejected"),
        };
        Ok(())
    }

    fn dummy_reencrypt_vault(&self, caller: Principal, vault_id: VaultId) -> Result<(), UserError> {
        let (vault_version, mut dummy_encrypted_passwords) = self
            .get_vault(caller, vault_id)
            .expect("failed to get vault");

        for encrypted_password in dummy_encrypted_passwords.iter_mut() {
            encrypted_password.encrypted_text.push_str(" reencrypted");
        }

        self.reencrypt_vault(
            caller,
            vault_id,
            vault_version,
            dummy_encrypted_passwords
                .iter()
                .map(|p| p.encrypted_text.clone())
                .collect(),
        )
    }

    fn remove_user(
        &self,
        caller: Principal,
        vault_id: VaultId,
        user: Principal,
    ) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "remove_user",
            Encode!(&vault_id, &user.to_text()).unwrap(),
        )?;

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("remove_user call was rejected"),
        };
        Ok(())
    }

    fn rotate_vault_key(&self, caller: Principal, vault_id: VaultId) -> Result<(), UserError> {
        let reply = self.pic.update_call(
            self.password_manager_canister_id,
            caller,
            "rotate_vault_key",
            Encode!(&vault_id).unwrap(),
        )?;

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => candid::Decode!(bytes.as_slice(), ()).unwrap(),
            pocket_ic::WasmResult::Reject(_) => panic!("rotate_vault_key call was rejected"),
        };
        Ok(())
    }

    fn get_symmetric_encryption_key(
        &self,
        caller: Principal,
        vault_id: VaultId,
        derived_public_key_bytes: &[u8],
        transport_key: &TransportSecretKey,
    ) -> Result<(VaultVersion, Vec<u8>, Vec<u8>), UserError> {
        let (vault_version, encrypted_symmetric_key, derivation_id) =
            self.get_encrypted_symmetric_key(caller, vault_id, transport_key.public_key())?;
        let encryption_key = transport_key
            .decrypt(
                hex::decode(encrypted_symmetric_key)
                    .expect("failed to decode encrypted key")
                    .as_slice(),
                derived_public_key_bytes,
                derivation_id.as_slice(),
            )
            .expect("failed to decrypt key");
        Ok((vault_version, encryption_key, derivation_id))
    }

    fn get_accessible_vault_ids(&self, caller: Principal) -> Vec<VaultId> {
        let reply = self
            .pic
            .update_call(
                self.password_manager_canister_id,
                caller,
                "get_accessible_vault_ids",
                Encode!(&()).unwrap(),
            )
            .expect("failed to get_accessible_vault_ids");

        match reply {
            pocket_ic::WasmResult::Reply(bytes) => {
                candid::Decode!(bytes.as_slice(), Vec<VaultId>).unwrap()
            }
            pocket_ic::WasmResult::Reject(_) => {
                panic!("get_accessible_vault_ids call was rejected")
            }
        }
    }
}
