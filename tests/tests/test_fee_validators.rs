mod setup;

use std::vec;

use elrond_wasm_debug::{tx_mock::TxTokenTransfer, rust_biguint};
use setup::TestsSetup;

const FEE_TOKEN: &[u8] = b"TKN-1238j5";
const FEE_AMOUNT: u64 = 100_000;

#[test]
fn test_no_fee_payment() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    setup.process_fee_payment_context(
        vec![],
        vec![],
        Some("Invalid fee amount provided")
    );
    setup.check_accumulated_fees(0);
}

#[test]
fn test_fee_is_zero() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, 0);

    setup.process_fee_payment_context(
        vec![],
        vec![],
        None
    );
    setup.check_accumulated_fees(0);
}

#[test]
fn test_one_payment_is_fee_total() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(FEE_AMOUNT)
        }
    ];
    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT));

    setup.process_fee_payment_context(
        payments,
        vec![],
        None
    );
    setup.check_accumulated_fees(FEE_AMOUNT);
}

#[test]
fn test_one_payment_subtract_fee() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(FEE_AMOUNT + 50_00)
        }
    ];

    let expected_payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(50_00)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT * 2));

    setup.process_fee_payment_context(payments, expected_payments, None);
    setup.check_accumulated_fees(FEE_AMOUNT);
}

#[test]
fn test_one_payment_not_enough_fees() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(FEE_AMOUNT - 50_000)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT));

    setup.process_fee_payment_context(
        payments,
        vec![],
        Some("Not enough amount to pay the fee")
    );
    setup.check_accumulated_fees(0);
}

#[test]
fn test_multiple_payments_subtract_fee_payment() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(FEE_AMOUNT)
        },
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    let expected_payments = vec![
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"TKN-aji8kl", &rust_biguint!(300_000));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"STK-aji8kl", &rust_biguint!(300_000));

    setup.process_fee_payment_context(
        payments,
        expected_payments,
        None
    );
    setup.check_accumulated_fees(100_000);
}


#[test]
fn test_multiple_payments_subtract_fee_from_payment() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(FEE_AMOUNT + 50_000)
        },
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    let expected_payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(50_000)
        },
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT * 2));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"TKN-aji8kl", &rust_biguint!(300_000));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"STK-aji8kl", &rust_biguint!(300_000));

    setup.process_fee_payment_context(
        payments,
        expected_payments,
        None
    );
    setup.check_accumulated_fees(100_000);
}


#[test]
fn test_multiple_payments_not_enough_fees() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: FEE_TOKEN.to_vec(),
            nonce: 0,
            value: rust_biguint!(50_000)
        },
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, FEE_TOKEN, &rust_biguint!(FEE_AMOUNT * 2));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"TKN-aji8kl", &rust_biguint!(300_000));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"STK-aji8kl", &rust_biguint!(300_000));


    setup.process_fee_payment_context(
        payments,
        vec![],
        Some("Not enough amount to pay the fee")
    );

    setup.check_accumulated_fees(0);
}

#[test]
fn test_multiple_payments_no_fee_payment() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.set_fee_settings(FEE_TOKEN, FEE_AMOUNT);

    let payments = vec![
        TxTokenTransfer {
            token_identifier: b"TKN-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(100_000)
        },
        TxTokenTransfer {
            token_identifier: b"STK-aji8kl".to_vec(),
            nonce: 0,
            value: rust_biguint!(300_000)
        }
    ];

    setup.b_wrapper.set_esdt_balance(&setup.owner, b"TKN-aji8kl", &rust_biguint!(300_000));
    setup.b_wrapper.set_esdt_balance(&setup.owner, b"STK-aji8kl", &rust_biguint!(300_000));


    setup.process_fee_payment_context(
        payments,
        vec![],
        Some("No fee payment provided")
    );

    setup.check_accumulated_fees(0);
}