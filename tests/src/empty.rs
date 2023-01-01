#![no_std]

use open_modules;

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait EmptyTestContract:
    open_modules::multi_token_payment::MultiTokenPayModule
{
    #[init]
    fn init(&self) {}
}
