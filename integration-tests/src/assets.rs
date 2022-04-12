use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {	
	ExtBuilder::default().build().execute_with(|| {	
		assert_ok!(Assets::force_create(Origin::root(), 0, 1, true, 1));
		assert_ok!(Assets::mint(Origin::signed(1), 0, 1, 100));
		assert_eq!(Assets::balance(0, 1), 100);
		assert_ok!(Assets::mint(Origin::signed(1), 0, 2, 100));
		assert_eq!(Assets::balance(0, 2), 100);
	});
}
