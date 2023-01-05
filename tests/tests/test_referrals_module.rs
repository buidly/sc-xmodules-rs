mod setup;

use elrond_wasm_debug::rust_biguint;

use setup::TestsSetup;
use setup::referrals::REWARDS_TOKEN_ID;


#[test]
fn test_add_tiers() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier1", 100, 10),
        (b"tier2", 200, 20),
        (b"tier3", 300, 30),
    ], None);
}

#[test]
fn test_add_tiers_invalid_fee() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier1", 100, 10),
        (b"tier2", 200, 20),
        (b"tier3", 300, 10_000),
    ], Some("Invalid fee percentage"));
}

#[test]
fn test_add_tiers_already_exists() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier1", 100, 10),
        (b"tier2", 200, 20),
        (b"tier3", 300, 30),
    ], None);

    setup.add_tier_details(vec![
        (b"tier1", 100, 10),
    ], Some("Tier already exists"));
}

#[test]
fn test_register_tag_tires_not_set() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.register_referral_tag(b"tag1", Some("Tiers not set"));
}

#[test]
fn test_register_tag() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier1", 0, 10),
        (b"tier2", 10, 20),
        (b"tier3", 20, 30),
    ], None);

    setup.register_referral_tag(b"tag1", None);
}

#[test]
fn test_register_tag_incorrect_tiers_added() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier1", 50, 10),
    ], None);

    setup.register_referral_tag(b"tag1", Some("No tier with min volume 0"));
}

#[test]
fn test_register_tag_random_order_on_tiers() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![
        (b"tier2", 10, 20),
        (b"tier3", 0, 30),
        (b"tier1", 20, 10),
    ], None);

    setup.register_referral_tag(b"tag1", None);
}

#[test]
fn test_register_tag_already_has_owner() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![(b"tier2", 10, 20),(b"tier3", 0, 30),(b"tier1", 20, 10)], None);
    setup.register_referral_tag(b"tag1", None);
    setup.register_referral_tag(b"tag1", Some("Tag already registered"));
}

#[test]
fn test_register_tag_already_registered_by_another_user() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![(b"tier2", 10, 20),(b"tier3", 0, 30),(b"tier1", 20, 10)], None);
    let user = setup.create_empty_account();
    setup.register_referral_tag(b"tag1", None);
    setup.register_referral_tag_with_caller(b"tag1", &user, Some("Tag already registered"))
}

#[test]
fn test_register_tag_user_already_has_a_tag() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.add_tier_details(vec![(b"tier2", 10, 20),(b"tier3", 0, 30),(b"tier1", 20, 10)], None);
    setup.register_referral_tag(b"tag1", None);
    setup.register_referral_tag(b"tag2", Some("User already owns a tag"));
}

#[test]
fn test_claim_referral_fee() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    let tag = b"tag1";
    setup.add_tier_details(vec![(b"tier2", 10, 20),(b"tier3", 0, 30),(b"tier1", 20, 10)], None);
    setup.register_referral_tag(tag, None);

    setup.set_rewards_token();
    setup.mock_accumulating_rewards(tag);
    setup.check_collected_tag_fees(tag, 100);
    setup.b_wrapper.check_esdt_balance(&setup.owner, REWARDS_TOKEN_ID, &rust_biguint!(0));

    setup.claim_referral_fees(None);
    setup.check_collected_tag_fees(tag, 0);
    setup.b_wrapper.check_esdt_balance(&setup.owner, REWARDS_TOKEN_ID, &rust_biguint!(100));
}

#[test]
fn test_claim_referral_fee_not_a_tag_owner() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    setup.claim_referral_fees(Some("Not a tag owner"));
}

#[test]
fn test_update_tier() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    let tag = b"tag1";
    setup.add_tier_details(vec![
        (b"tier3", 30, 2_000),(b"tier1", 0, 500),(b"tier2", 10, 1_000)],
        None
    );
    setup.register_referral_tag(tag, None);

    setup.mock_set_accumulated_volume(15, tag);
    setup.update_tier(None);
}

#[test]
fn test_update_tier_no_upgrade() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    let tag = b"tag1";
    setup.add_tier_details(vec![
        (b"tier3", 30, 2_000),(b"tier1", 0, 500),(b"tier2", 10, 1_000)],
        None
    );
    setup.register_referral_tag(tag, None);
    setup.mock_set_accumulated_volume(5, tag);

    setup.update_tier(Some("No tier upgrade found"));
}

#[test]
fn test_remove_tier_with_active_users() {
    let mut setup = TestsSetup::init(tests::contract_obj);
    let tag = b"tag1";
    setup.add_tier_details(vec![
        (b"tier3", 30, 2_000),(b"tier1", 0, 500),(b"tier2", 10, 1_000)],
        None
    );
    setup.register_referral_tag(tag, None);
    setup.mock_set_accumulated_volume(15, tag);

    setup.update_tier(None);
    setup.check_user_percentage(tag, 1_000);

    setup.remove_tier_details(b"tier2");
    setup.update_tier(Some("No tier upgrade found"));

    setup.check_user_percentage(tag, 1_000);
}
