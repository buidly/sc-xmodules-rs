
use elrond_wasm::require;
use elrond_wasm::storage::mappers::SingleValueMapper;


/// This module contains the logic for accepting multiple ESDT payments.
#[elrond_wasm::module]
pub trait MultiTokenPayModule {

    /// Adds a payment token to the list of accepted tokens.
    #[only_owner]
    #[endpoint(addPaymentToken)]
    fn add_payment_token(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        require!(amount > 0, "Amount must be greater than 0");
        self.accepted_tokens(&token).set(amount);
    }

    /// Removes a payment token from the list of accepted tokens.
    #[only_owner]
    #[endpoint(removePaymentToken)]
    fn remove_payment_token(&self, token: EgldOrEsdtTokenIdentifier) {
        self.accepted_tokens(&token).clear();
    }

    /// Require clause used for validating a single ESDT payment. It checks if
    /// the payment received contains the correct token and amount.
    fn require_valid_payment(&self) {
        let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(!self.accepted_tokens(&token).is_empty(), "Payment token not accepted");

        let expected_amount = self.accepted_tokens(&token).get();
        require!(amount == expected_amount, "Invalid payment amount");
    }

    /// Require clause used for validating multiple ESDT payments. It checks if
    /// the payments received contain the correct tokens and amounts.
    ///
    /// The generic parameter X is used to specify the maximum number of payments
    /// that can be received.
    ///
    /// NOTE: This can be easily used for when the contract accepts NFT and ESDT
    /// payments as well.
    fn require_valid_payments<const X: usize>(&self) {
        let payments = self.call_value().multi_esdt::<X>();
        for payment in payments.iter() {
            let token_id = EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier.clone());
            require!(!self.accepted_tokens(&token_id).is_empty(), "Invalid payment token");

            let expected_amount = self.accepted_tokens(&token_id).get();
            require!(payment.amount == expected_amount, "Invalid payment amount");
        }
    }

    /// Storage mapper used for storing the accepted tokens. All tokens stored
    /// must contain the amount that is accepted for payment.
    #[storage_mapper("accepted_tokens")]
    fn accepted_tokens(&self, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;
}