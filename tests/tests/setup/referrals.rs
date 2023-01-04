use elrond_wasm::{types::{MultiValueEncoded, Address}, elrond_codec::multi_types::MultiValue3};
use elrond_wasm_debug::{DebugApi, rust_biguint, managed_buffer, managed_biguint, managed_token_id};

use super::TestsSetup;

use xmodules::referrals::{ReferralModule};

pub const REWARDS_TOKEN_ID: &[u8] = b"RWD-8j5bp0";

impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    pub fn register_referral_tag(&mut self, tag: &[u8], expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.register_referral_tag(managed_buffer!(tag));
            });

        self.assert_result(tx, expected_err);
    }

    pub fn register_referral_tag_with_caller(&mut self, tag: &[u8], caller: &Address, expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_tx(&caller, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.register_referral_tag(managed_buffer!(tag));
            });

        self.assert_result(tx, expected_err);
    }

    pub fn claim_referral_fees(&mut self, expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.claim_referral_fees();
            });

        self.assert_result(tx, expected_err);
    }

    pub fn set_rewards_token(&mut self) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.rewards_token().set(managed_token_id!(REWARDS_TOKEN_ID));
            })
            .assert_ok();
    }

    pub fn mock_accumulating_rewards(&mut self, tag: &[u8]) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.collected_tag_fees(&managed_buffer!(tag)).set(managed_biguint!(100u64));
            })
            .assert_ok();

        self.b_wrapper.set_esdt_balance(self.c_wrapper.address_ref(), REWARDS_TOKEN_ID, &rust_biguint!(100u64));
    }

    pub fn check_collected_tag_fees(&mut self, tag: &[u8], expected: u64) {
        self.b_wrapper
            .execute_query(&self.c_wrapper, |sc| {
                let value = sc.collected_tag_fees(&managed_buffer!(tag)).get();
                assert_eq!(value, managed_biguint!(expected));
            })
            .assert_ok();
    }

    pub fn add_tier_details(&mut self, tiers: Vec<(&[u8], u64, u64)>, expected_err: Option<&str>) {
        let tx =self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                let mut tiers_wrapped = MultiValueEncoded::new();
                for tier in tiers {
                    tiers_wrapped.push(MultiValue3((managed_buffer!(tier.0), managed_biguint!(tier.1), tier.2)))
                }
                sc.add_tier_details(tiers_wrapped);
            });

        self.assert_result(tx, expected_err);
    }

    pub fn update_tier(&mut self, expected_err: Option<&str>) {
        let tx = self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.update_tier();
            });

        self.assert_result(tx, expected_err);
    }

    pub fn mock_set_accumulated_volume(&mut self, volume: u64, tag: &[u8]) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.accumulated_volume(&managed_buffer!(tag)).set(managed_biguint!(volume));
            })
            .assert_ok();
    }

    pub fn remove_tier_details(&mut self, tier_id: &[u8]) {
        self.b_wrapper
            .execute_tx(&self.owner, &self.c_wrapper, &rust_biguint!(0u64), |sc| {
                sc.remove_tier_details(managed_buffer!(tier_id));
            })
            .assert_ok();
    }

    pub fn check_user_percentage(&mut self, tag: &[u8], expected: u64) {
        self.b_wrapper
            .execute_query(&self.c_wrapper, |sc| {
                let value = sc.referral_tag_percent(&managed_buffer!(tag)).get();
                assert_eq!(value, expected);
            })
            .assert_ok();
    }
}