#![no_std]

use xmodules::{multi_token_payment, referrals};

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait EmptyTestContract:
    multi_token_payment::MultiTokenPayModule
    + referrals::ReferralModule
{
    #[init]
    fn init(&self) {}
}
