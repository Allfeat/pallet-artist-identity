use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

mod fields_test {
    use crate::mock::mock_artists::ALICE;

    use super::*;

    fn assert_last_event(event: Event<Test>) {
        System::assert_last_event(mock::Event::ArtistIdentity(event))
    }

    #[test]
    fn update_field() {
        new_test_ext().execute_with(|| {
            let alice_alias: Option<Vec<u8>> = match ALICE.alias {
                Some(alias) => Some(alias.into()),
                None => None,
            };

            // None should be returned as the artist have never set an alias
            let mut current_infos = ArtistIdentity::get_artist_infos(ALICE.account_id);
            assert!(current_infos.alias.is_none());

            // Should update the alias with the ALICE alias
            assert_ok!(ArtistIdentity::update_alias(
                Origin::signed(ALICE.account_id),
                alice_alias.clone()
            ));

            assert_last_event(Event::<Test>::UpdatedAlias(
                ALICE.account_id,
                match alice_alias.clone() {
                    Some(alias) => Some(alias.try_into().unwrap()),
                    None => None,
                },
            ));

            current_infos = ArtistIdentity::get_artist_infos(ALICE.account_id);
            let unbounded_current_alias: Option<Vec<u8>> = match current_infos.alias {
                Some(bounded_alias) => Some(bounded_alias.into()),
                None => None,
            };
            assert_eq!(unbounded_current_alias, alice_alias);
        })
    }
}
