elrond_wasm::imports!();


#[elrond_wasm::module]
pub trait FeeManagerModule {

    #[only_owner]
    #[endpoint(setFeeSettings)]
    fn set_fee_settings(&self, fee_token: TokenIdentifier, fee_amount: BigUint) {
        let current_fee_token = self.fee_token().get();
        let accumulated_fees = self.accumulated_fees().get();
        if current_fee_token != fee_token && accumulated_fees > 0 {
            self.withdraw_fees();
        }
        self.fee_token().set(fee_token);
        self.fee_amount().set(fee_amount);
    }

    #[only_owner]
    #[endpoint(withdrawFees)]
    fn withdraw_fees(&self) {
        let fee_token = self.fee_token().get();
        let owner = self.blockchain().get_owner_address();
        let accumulated_fees = self.accumulated_fees().get();
        if accumulated_fees == 0 {
            return
        }

        self.send().direct_esdt(&owner, &fee_token, 0, &accumulated_fees);
        self.accumulated_fees().clear();
    }

    #[view(getFeeToken)]
    #[storage_mapper("fee_token")]
    fn fee_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getFeeAmount)]
    #[storage_mapper("fee_amount")]
    fn fee_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getAccumulatedFees)]
    #[storage_mapper("accumulated_fees")]
    fn accumulated_fees(&self) -> SingleValueMapper<BigUint>;
}
