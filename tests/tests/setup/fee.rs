use elrond_wasm_debug::{DebugApi, rust_biguint, managed_token_id, managed_biguint, tx_mock::TxTokenTransfer};
use xmodules::{fee_manager::FeeManagerModule, fee_payment_validator::FeePaymentValidatorModule};

use super::TestsSetup;


impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{

    pub fn set_fee_settings(&mut self, token: &[u8], fee: u64) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0), |sc| {
                sc.set_fee_settings(
                    managed_token_id!(token),
                    managed_biguint!(fee)
                );
            })
            .assert_ok();
    }

    pub fn check_accumulated_fees(&mut self, expected_amount: u64) {
        self.b_wrapper
            .execute_query(&self.c_wrapper, |sc| {
                let accumulated_fees = sc.accumulated_fees().get();
                assert_eq!(accumulated_fees, managed_biguint!(expected_amount));
            })
            .assert_ok();
    }

    pub fn withdraw_fees(&mut self) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0), |sc| {
                sc.withdraw_fees();
            })
            .assert_ok();
    }

    pub fn set_accumulated_fees(&mut self, amount: u64, fee_token: &[u8]) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0), |sc| {
                sc.accumulated_fees().set(managed_biguint!(amount));
            })
            .assert_ok();

        self.b_wrapper.set_esdt_balance(self.c_wrapper.address_ref(), fee_token, &rust_biguint!(amount));
    }

    pub fn process_fee_payment_context(
        &mut self,
        payments: Vec<TxTokenTransfer>,
        expected_out_payment: Vec<TxTokenTransfer>,
        expected_err: Option<&str>,
    ) {
        let tx = self.b_wrapper
            .execute_esdt_multi_transfer(&self.owner, &self.c_wrapper, &payments[..], |sc| {
                let out_payments = sc.process_fee_from_payments();
                for (i, out_payment) in out_payments.iter().enumerate() {
                    let expected_out_payment = &expected_out_payment[i];
                    assert_eq!(
                        rust_biguint!(out_payment.amount.to_u64().unwrap()),
                        expected_out_payment.value
                    );
                }
            });

        match expected_err {
            Some(err) => tx.assert_error(4, err),
            None => tx.assert_ok()
        };
    }

}