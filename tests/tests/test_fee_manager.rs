mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::TestsSetup;

const FEE_TOKEN: &[u8] = b"TKN-1238j5";
const FEE_AMOUNT: u64 = 100_000;

#[test]
fn test_set_fee_settings() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);
    setup.check_accumulated_fees(0);
}

#[test]
fn test_set_fee_settings_and_withdraw() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);
    setup.set_accumulated_fees(FEE_AMOUNT * 4, FEE_TOKEN);
    setup.b_wrapper.check_esdt_balance(setup.c_wrapper.address_ref(), FEE_TOKEN, &rust_biguint!(FEE_AMOUNT * 4));
    setup.check_balance(FEE_TOKEN, 0);
    setup.set_fee_settings(b"NEW-s8k1l0", FEE_AMOUNT);

    setup.check_balance(FEE_TOKEN, FEE_AMOUNT * 4);
    setup.b_wrapper.check_esdt_balance(setup.c_wrapper.address_ref(), FEE_TOKEN, &rust_biguint!(0));
}

#[test]
fn test_withdraw() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);
    setup.set_accumulated_fees(FEE_AMOUNT * 4, FEE_TOKEN);

    setup.withdraw_fees();
    setup.check_balance(FEE_TOKEN, FEE_AMOUNT * 4);
    setup.b_wrapper.check_esdt_balance(setup.c_wrapper.address_ref(), FEE_TOKEN, &rust_biguint!(0));
}