use elrond_wasm::{types::{Address, EgldOrEsdtTokenIdentifier}};
use elrond_wasm_debug::{
    DebugApi,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper}, rust_biguint, managed_biguint, tx_mock::TxTokenTransfer
};

use tests;
use tests::EmptyTestContract;
use open_modules::multi_token_payment::MultiTokenPayModule;
const WASM_PATH: &str = "../output/tests.wasm";


pub struct TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    pub owner: Address,
    pub b_wrapper: BlockchainStateWrapper,
    pub c_wrapper: ContractObjWrapper<tests::ContractObj<DebugApi>, Builder>
}

impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    pub fn init(builder: Builder) -> Self {
        let mut blockchain_wrapper = BlockchainStateWrapper::new();

        let owner_account = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
        let contract_wrapper = blockchain_wrapper.create_sc_account(
            &rust_biguint!(0u64),
            Some(&owner_account),
            builder,
            WASM_PATH
        );

        blockchain_wrapper
            .execute_tx(&owner_account, &contract_wrapper, &rust_biguint!(0u64), |sc| {
                sc.init();
            })
            .assert_ok();

        Self {
            owner: owner_account,
            b_wrapper: blockchain_wrapper,
            c_wrapper: contract_wrapper
        }
    }

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
            .execute_esdt_transfer(&self.owner, &self.c_wrapper, &token, 0, &rust_biguint!(amount), |sc| {
                sc.require_valid_payment();
            });

        if let Some(expected_msg) = expected_err {
            tx.assert_error(4, expected_msg);
        } else {
            tx.assert_ok();
        }
    }

    pub fn require_valid_payments<const X: usize>(&mut self, payments: &[TxTokenTransfer], expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_esdt_multi_transfer(&self.owner, &self.c_wrapper, payments, |sc| {
                sc.require_valid_payments::<X>();
            });

        if let Some(expected_msg) = expected_err {
            tx.assert_error(4, expected_msg);
        } else {
            tx.assert_ok();
        }
    }
}