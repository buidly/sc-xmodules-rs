#![no_std]

use multi_token_pay;

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait EmptyTestContract:
    multi_token_pay::MultiTokenPayModule
{
    #[init]
    fn init(&self) {}
}
