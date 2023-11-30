use sylvia::multitest::App; // to simulate blockchain

use crate::{contract::multitest_utils::CodeId, error::ContractError};

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app); // identifies our contract on the blockchain

    let owner = "owner";

    let contract = code_id.instantiate(42).call(owner).unwrap();

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

    let contract = code_id.instantiate(1).call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 1);

    contract.decrement_count().call(owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 0);

    let err = contract.decrement_count().call(owner).unwrap_err();
    assert_eq!(err, ContractError::CannotDecrementCount);
}
