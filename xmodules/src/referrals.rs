elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const MAX_PERCENTAGE: u64 = 10_000u64;
pub const MAX_FEE_PERCENTAGE: u64 = 9_000u64;
pub const DEFAULT_REFERRAL_PERCENTAGE: u64 = 500u64;

pub type TierDetailsArg<M> = MultiValue3<ManagedBuffer<M>, BigUint<M>, u64>;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, Debug, PartialEq)]
pub struct TierDetails<M: ManagedTypeApi>  {
    pub name: ManagedBuffer<M>,
    pub min_volume: BigUint<M>,
    pub fee_percent: u64,
}

#[elrond_wasm::module]
pub trait ReferralModule {

    /// Registers a new referral tag. The tag must be unique and the caller
    /// must not already own a tag.
    #[endpoint(registerReferralTag)]
    fn register_referral_tag(&self, tag: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(self.referral_tag_percent(&tag).is_empty(), "Tag already registered");
        require!(self.user_tag_mapping(&caller).is_empty(), "User already owns a tag");
        require!(!self.tier_details().is_empty(), "Tiers not set");

        self.user_tag_mapping(&caller).set(tag.clone());
        self.accumulated_volume(&tag).clear();

        for tier in self.tier_details().iter() {
            if tier.min_volume == BigUint::zero() {
                self.referral_tag_percent(&tag).set(tier.fee_percent);
                return;
            }
        }
        sc_panic!("No tier with min volume 0");
    }

    /// Claims the referral fees for the caller. The caller must own a tag.
    #[endpoint(claimReferralFees)]
    fn claim_referral_fees(&self) {
        let caller = self.blockchain().get_caller();
        require!(!self.user_tag_mapping(&caller).is_empty(), "Not a tag owner");
        let user_tag = self.user_tag_mapping(&caller).get();

        let amount = self.collected_tag_fees(&user_tag).get();
        require!(amount > 0, "No fees to claim");

        self.send().direct_esdt(&caller, &self.rewards_token().get(), 0, &amount);
        self.collected_tag_fees(&user_tag).clear();
    }

    /// Updates the tier for the caller. The caller must own a tag.
    /// Updating the tier is done by comparing the caller's accumulated volume
    /// with the tiers' minimum volumes.
    #[endpoint(updateTier)]
    fn update_tier(&self) -> ManagedBuffer {
        let caller = self.blockchain().get_caller();
        require!(!self.user_tag_mapping(&caller).is_empty(), "Not a tag owner");

        let user_tag = self.user_tag_mapping(&caller).get();
        let user_volume = self.accumulated_volume(&user_tag).get();
        let mut tier_name = ManagedBuffer::new();
        let mut fee_percent = self.referral_tag_percent(&user_tag).get();

        for tier in self.tier_details().iter() {
            if user_volume >= tier.min_volume && tier.fee_percent > fee_percent {
                fee_percent = tier.fee_percent;
                tier_name = tier.name;
            }
        }

        require!(fee_percent != self.referral_tag_percent(&user_tag).get(), "No tier upgrade found");
        self.referral_tag_percent(&user_tag).set(fee_percent);

        tier_name
    }

    /// Adds multiple tiers at once. The tiers must be unique.
    #[endpoint(addTierDetails)]
    fn add_tier_details(&self, tiers: MultiValueEncoded<TierDetailsArg<Self::Api>>) {
        for tier in tiers.into_iter() {
            let (name, min_volume, fee_percent) = tier.into_tuple();
            require!(fee_percent < MAX_FEE_PERCENTAGE, "Invalid fee percentage");
            let is_new = self.tier_details().insert(TierDetails {
                name,
                min_volume,
                fee_percent
            });

            require!(is_new, "Tier already exists");
        }
    }

    /// Removes a tier by name. The tier must exist.
    #[only_owner]
    #[endpoint(removeTierDetails)]
    fn remove_tier_details(&self, name: ManagedBuffer) {
        let mut tier_details = self.tier_details();

        for tier in tier_details.iter() {
            if tier.name == name {
                tier_details.swap_remove(&tier);
                return;
            }
        }

        sc_panic!("Tier not found");
    }

    /// Sets the referral fee percentage for a tag. The tag must exist.
    #[only_owner]
    #[endpoint(setReferralFeePercentage)]
    fn set_referral_fee_percentage(&self, tag: ManagedBuffer, new_percentage: u64) {
        require!(new_percentage < MAX_FEE_PERCENTAGE, "Invalid new percentage given");
        require!(!self.referral_tag_percent(&tag).is_empty(), "Tag not found");
        self.referral_tag_percent(&tag).set(new_percentage);
    }

    /// Removes a tag. The tag must exist. When the tag is removed the user
    /// automatically receives all the collected fees.
    #[only_owner]
    #[endpoint(removeReferralTag)]
    fn remove_referral_tag(&self, user_address: ManagedAddress) {
        let rewards_token = self.rewards_token().get();
        let tag = self.user_tag_mapping(&user_address).get();
        let collected_amount = self.collected_tag_fees(&tag).get();
        if collected_amount > 0 {
            self.send().direct_esdt(&user_address, &rewards_token, 0, &collected_amount);
        }

        self.accumulated_volume(&tag).clear();
        self.referral_tag_percent(&tag).clear();
        self.collected_tag_fees(&tag).clear();
        self.user_tag_mapping(&user_address).clear();
    }

    #[view(getCollectedFeeAmount)]
    fn get_collected_fee_amount(&self, address: ManagedAddress) -> BigUint {
        let tag_mapping = self.user_tag_mapping(&address);
        self.collected_tag_fees(&tag_mapping.get()).get()
    }


    fn subtract_referral_fee_and_update_collected_fees(&self, fee_amount: BigUint, tag: ManagedBuffer) -> BigUint {
        let tag_percentage = self.referral_tag_percent(&tag).get();
        if tag_percentage == 0 {
            return fee_amount;
        }

        let referral_amount = &fee_amount * tag_percentage / MAX_PERCENTAGE;
        self.collected_tag_fees(&tag).update(|x| *x += &referral_amount);

        fee_amount - referral_amount
    }

    #[view(getUserTag)]
    #[storage_mapper("user_tag_mapping")]
    fn user_tag_mapping(&self, user: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("collected_tag_fees")]
    fn collected_tag_fees(&self, tag: &ManagedBuffer) -> SingleValueMapper<BigUint>;

    #[view(getReferralFeePercentage)]
    #[storage_mapper("referral_tags_percent")]
    fn referral_tag_percent(&self, tag: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[view(getTierDetails)]
    #[storage_mapper("tier_details")]
    fn tier_details(&self) -> UnorderedSetMapper<TierDetails<Self::Api>>;

    #[view(getTagAccumulatedVolume)]
    #[storage_mapper("accumulated_volume")]
    fn accumulated_volume(&self, tag: &ManagedBuffer) -> SingleValueMapper<BigUint>;

    #[view(getWrappedToken)]
    #[storage_mapper("rewards_token")]
    fn rewards_token(&self) -> SingleValueMapper<TokenIdentifier>;
}