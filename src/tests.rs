use super::*;
use crate::{mock::*, Event};
use frame_support::assert_ok;

mod fields_test {
    use crate::mock::mock_artists::ALICE;
    use frame_support::assert_noop;

    use super::*;

    fn assert_last_event(event: Event<Test>) {
        System::assert_last_event(mock::Event::ArtistIdentity(event))
    }

    #[test]
    fn update_field() {
        new_test_ext().execute_with(|| {
            let expected_cost = ALICE.alias.len() as u64 * mock::CostPerByte::get();

            let balance_before = Balances::free_balance(ALICE.account_id);

            // None should be returned as the artist have never set an alias
            let mut current_metadata = ArtistIdentity::get_artist_metadata(ALICE.account_id);
            assert_eq!(current_metadata.alias, vec![]);

            // Should update the alias with the ALICE alias
            assert_ok!(ArtistIdentity::update_alias(
                Origin::signed(ALICE.account_id),
                ALICE.alias.into()
            ));

            assert_last_event(Event::<Test>::UpdatedMetadata(
                ALICE.account_id,
                ALICE.alias.into(),
            ));

            let new_balance = Balances::free_balance(ALICE.account_id);
            current_metadata = ArtistIdentity::get_artist_metadata(ALICE.account_id);

            let unbounded_current_alias: Vec<u8> = current_metadata.alias.into();

            assert_eq!(new_balance, balance_before - expected_cost);
            assert_eq!(unbounded_current_alias, Vec::<u8>::from(ALICE.alias));
        })
    }

    #[test]
    fn update_styles() {
        new_test_ext().execute_with(|| {
            let alice_styles = vec![b"Electro".to_vec(), b"Hardcore".to_vec()];
            let mut expected_cost: BalanceOf<Test> = Default::default();
            for style in &alice_styles {
                expected_cost = expected_cost + Pallet::<Test>::compute_cost(style.clone());
            }

            let before_metadata: MetadataOf<Test> =
                ArtistIdentity::get_artist_metadata(&ALICE.account_id);
            let before_balance = Balances::free_balance(ALICE.account_id);

            assert!(before_metadata.music_styles.is_empty());

            assert_noop!(
                ArtistIdentity::update_music_styles(
                    Origin::signed(ALICE.account_id),
                    vec![b"Non existant".to_vec()]
                ),
                pallet_music_styles::Error::<Test>::StyleNotFound
            );

            assert_ok!(ArtistIdentity::update_music_styles(
                Origin::signed(ALICE.account_id),
                alice_styles
            ));

            let after_metadata: MetadataOf<Test> =
                ArtistIdentity::get_artist_metadata(&ALICE.account_id);
            let new_balance = Balances::free_balance(ALICE.account_id);

            // check if mutated
            assert_eq!(after_metadata.music_styles.len(), 2);
            assert_eq!(new_balance, before_balance - expected_cost);
        })
    }
}
