elrond_wasm::imports!();


#[elrond_wasm::module]
pub trait FeePaymentValidatorModule:
    crate::fee_manager::FeeManagerModule
{

    /// This function helps with identifying the fee payment within the call
    /// payments and subtract the amount from it returning the list of other
    /// payments sent by the caller.
    ///
    /// It has the capabilities of processing any amount of payments provided
    /// with the condition of having a fee amount set in the storage.
    fn process_fee_from_payments(&self) -> ManagedVec<EsdtTokenPayment> {
        let mut all_call_payments = self.call_value().all_esdt_transfers();
        match all_call_payments.len() {
            0 => {
                require!(self.fee_amount().get() == 0, "Invalid fee amount provided");
                ManagedVec::new()
            },
            1 => self._process_one_payment(&mut all_call_payments.get(0)),
            _ => self._process_multiple_payments(&mut all_call_payments)
        }
    }

    /// Subtracting the fee amount from the call payments and returning what
    /// is left as a list of payments.
    fn _process_one_payment(&self, payment: &mut EsdtTokenPayment) -> ManagedVec<EsdtTokenPayment> {
        let fee_amount = self.fee_amount().get();

        if payment.amount == fee_amount {
            self.accumulated_fees().update(|x| *x += fee_amount);
            return ManagedVec::new()
        }

        if payment.amount > fee_amount {
            self.accumulated_fees().update(|x| *x += &fee_amount);
            payment.amount -= &fee_amount;

            let mut result_payments = ManagedVec::new();
            result_payments.push(payment.clone());

            return result_payments
        }

        sc_panic!("Not enough amount to pay the fee")
    }

    /// Subtracts the fee payment from the list of payments returning the
    /// remaining payments back.
    fn _process_multiple_payments(&self, payments: &mut ManagedVec<EsdtTokenPayment>) -> ManagedVec<EsdtTokenPayment> {
        let fee_amount = self.fee_amount().get();
        let fee_token = self.fee_token().get();
        let mut has_fee_payment = false;

        for i in 0..payments.len() {
            let mut payment = payments.get(i);
            if payment.token_identifier == fee_token {
                has_fee_payment = true;
                self.accumulated_fees().update(|x| *x += &fee_amount);

                if payment.amount > fee_amount {
                    payment.amount -= &fee_amount;
                    break
                }

                if payment.amount == fee_amount {
                    payments.remove(i);
                    break
                }

                sc_panic!("Not enough amount to pay the fee");
            }
        }

        require!(has_fee_payment, "No fee payment provided");

        payments.clone()
    }
}