#![allow(deprecated)]

use cosmwasm_std::Addr;
use sylvia::multitest::App;

use crate::{contract::sv::multitest_utils::CodeId, error::ContractError, whitelist_impl::sv::test_utils::Whitelist};

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app); // identifies our contract on the blockchain

    let owner = "owner";

    let contract = code_id.instantiate(42, vec![]).call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 42);

    contract.increment_count().call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 43);
}
#[test]
fn decrement_below_zero() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner";

    let contract = code_id.instantiate(1, vec![]).call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 1);

    contract.decrement_count().call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 0);

    let err = contract.decrement_count().call(owner).unwrap_err();
    assert_eq!(err, ContractError::CannotDecrementCount);
}
#[test]
fn manage_admins1() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner";
    let admin = "admin";
    let random_user = "random";

    let contract = code_id.instantiate(1, vec![]).call(owner).unwrap();

    // Admins list is empty
    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert!(admins.is_empty());

    // Admin can be added
    contract
        .whitelist_proxy()
        .add_admin(admin.to_owned())
        .call(owner)
        .unwrap();

    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert_eq!(admins, &[Addr::unchecked(admin)]);

    // Admin can NOT be removed nor added by a random Joe
    let err = contract
        .whitelist_proxy()
        .remove_admin(admin.to_owned())
        .call(random_user)
        .unwrap_err();
    assert_eq!(err, ContractError::NotTheOwner(Addr::unchecked(random_user)));

    // Admin can be removed
    contract
        .whitelist_proxy()
        .remove_admin(admin.to_owned())
        .call(owner)
        .unwrap();

    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert!(admins.is_empty());
}

#[test]
fn manage_admins() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner";
    let admin = "admin";

    let contract = code_id.instantiate(1,vec![]).call(owner).unwrap();

    // Admins list is empty
    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert!(admins.is_empty());

    // Admin can be added
    contract
        .whitelist_proxy()
        .add_admin(admin.to_owned())
        .call(owner)
        .unwrap();

    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert_eq!(admins, &[Addr::unchecked(admin)]);

    // Admin can be removed
    contract
        .whitelist_proxy()
        .remove_admin(admin.to_owned())
        .call(owner)
        .unwrap();

    let admins = contract.whitelist_proxy().admins().unwrap().admins;
    assert!(admins.is_empty());
}

