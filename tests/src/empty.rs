#![no_std]

use xmodules::{
    multi_token_payment,
    referrals,
    fee_manager,
    fee_payment_validator
};

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait EmptyTestContract:
    multi_token_payment::MultiTokenPayModule
    + fee_manager::FeeManagerModule
    + referrals::ReferralModule
    + fee_payment_validator::FeePaymentValidatorModule
{
    #[init]
    fn init(&self) {}
}
