use elrond_wasm_debug::{DebugApi, rust_biguint, managed_token_id, managed_biguint};
use xmodules::fee_manager::FeeManagerModule;

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

        self.b_wrapper.set_esdt_balance(&self.c_wrapper.address_ref(), fee_token, &rust_biguint!(amount));
    }
}