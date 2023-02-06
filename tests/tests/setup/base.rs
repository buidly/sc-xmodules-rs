use elrond_wasm::types::Address;

use elrond_wasm_debug::tx_mock::TxResult;
use elrond_wasm_debug::{DebugApi, rust_biguint};
use elrond_wasm_debug::testing_framework::{BlockchainStateWrapper, ContractObjWrapper};

use tests::EmptyTestContract;

const WASM_PATH: &str = "../output/tests.wasm";


/// This struct is used to hold the state of the blockchain and the contract.
pub struct TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    /// The address of the contract owner
    pub owner: Address,

    /// The blockchain state wrapper used for building all the tx and query contexts
    pub b_wrapper: BlockchainStateWrapper,

    /// The contract wrapper
    pub c_wrapper: ContractObjWrapper<tests::ContractObj<DebugApi>, Builder>
}

impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    /// Deploy the contract and create the tests setup
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

    /// Used for asserting the result of a tx based on the expected error message
    pub fn assert_result(&self, tx: TxResult, expected_err: Option<&str>) {
        if let Some(expected_msg) = expected_err {
            tx.assert_error(4, expected_msg);
        } else {
            tx.assert_ok();
        }
    }

    /// Creates an empty account and returns its address
    pub fn create_empty_account(&mut self) -> Address {
        self.b_wrapper.create_user_account(&rust_biguint!(0u64))
    }

    pub fn check_balance(&mut self, token: &[u8], expected_amount: u64) {
        self.b_wrapper.check_esdt_balance(&self.owner, token, &rust_biguint!(expected_amount));
    }
}
