use cosmwasm_std::Addr;

#[test]
fn sample_test() {
    let owner = Addr::unchecked("owner");

    assert_eq!(owner.to_string(), "owner")
}
