mod setup;

use elrond_wasm_debug::{rust_biguint, tx_mock::TxTokenTransfer};
use setup::TestsSetup;

const ESDT_TOKEN: &[u8] = b"TST-j28ml0";
const SECOND_TOKEN: &[u8] = b"TKN-9b4sdo";

#[test]
fn test_add_new_payment_token() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_payment_token_esdt(ESDT_TOKEN, 1_000_000_000u64);
    setup.add_payment_token_egld(1_000_000_000_000_000_000u64);
}

#[test]
fn test_remove_payment_token() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_payment_token_esdt(ESDT_TOKEN, 1_000_000_000u64);
    setup.remove_payment_token(ESDT_TOKEN);
    setup.remove_payment_token(SECOND_TOKEN);
}

#[test]
fn test_require_valid_payment() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.b_wrapper.set_esdt_balance(&setup.owner, ESDT_TOKEN, &rust_biguint!(1_000_000_000u64));
    setup.require_valid_payment(ESDT_TOKEN, 1_000_000u64, Some("Payment token not accepted"));

    setup.add_payment_token_esdt(ESDT_TOKEN, 1_000_000_000u64);
    setup.require_valid_payment(ESDT_TOKEN, 1_000_000u64, Some("Invalid payment amount"));

    setup.require_valid_payment(ESDT_TOKEN, 1_000_000_000u64, None);
}

#[test]
fn test_require_valid_payments() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.b_wrapper.set_esdt_balance(&setup.owner, ESDT_TOKEN, &rust_biguint!(1_000_000_000u64));
    setup.b_wrapper.set_esdt_balance(&setup.owner, SECOND_TOKEN, &rust_biguint!(1_000_000_000u64));

    setup.add_payment_token_esdt(ESDT_TOKEN, 1_000_000u64);
    setup.add_payment_token_esdt(SECOND_TOKEN, 1_000_000u64);


    let payments = [
        TxTokenTransfer {
            token_identifier: ESDT_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(1_000_000u64),
        },
        TxTokenTransfer {
            token_identifier: SECOND_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(1_000_000u64),
        },
    ];

    setup.require_valid_payments::<3>(&payments, Some("incorrect number of ESDT transfers"));
    setup.require_valid_payments::<2>(&payments, None);
}