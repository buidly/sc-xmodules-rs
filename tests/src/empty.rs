#![no_std]

use xmodules;

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait EmptyTestContract:
    xmodules::multi_token_payment::MultiTokenPayModule
    + xmodules::referrals::ReferralModule
{
    #[init]
    fn init(&self) {}
}
