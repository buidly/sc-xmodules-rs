use elrond_wasm::types::EgldOrEsdtTokenIdentifier;

use elrond_wasm_debug::{DebugApi, rust_biguint, managed_biguint};
use elrond_wasm_debug::tx_mock::TxTokenTransfer;

use xmodules::multi_token_payment::MultiTokenPayModule;

use super::base::TestsSetup;


impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    pub fn add_payment_token_esdt(&mut self, token: &[u8], amount: u64) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.add_payment_token(EgldOrEsdtTokenIdentifier::esdt(token), managed_biguint!(amount));
            })
            .assert_ok();
    }

    pub fn add_payment_token_egld(&mut self, amount: u64) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.add_payment_token(EgldOrEsdtTokenIdentifier::egld(), managed_biguint!(amount));
            })
            .assert_ok();
    }

    pub fn remove_payment_token(&mut self, token: &[u8]) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.remove_payment_token(EgldOrEsdtTokenIdentifier::esdt(token));
            })
            .assert_ok();
    }

    pub fn require_valid_payment(&mut self, token: &[u8], amount: u64, expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_esdt_transfer(&self.owner, &self.c_wrapper, token, 0, &rust_biguint!(amount), |sc| {
                sc.require_valid_payment();
            });

        self.assert_result(tx, expected_err);
    }

    pub fn require_valid_payments<const X: usize>(&mut self, payments: &[TxTokenTransfer], expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_esdt_multi_transfer(&self.owner, &self.c_wrapper, payments, |sc| {
                sc.require_valid_payments::<X>();
            });

        self.assert_result(tx, expected_err);
    }
}