// Copyright (C) 2023 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests.

use super::{mock::*, *};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::Hooks, weights::Weight};

/// returns CID hash of 68 bytes of given `i`.
fn create_cid(i: u8) -> OpaqueCid {
	let cid: OpaqueCid = [i; 68].to_vec().try_into().unwrap();
	cid
}

#[test]
fn set_charter_works() {
	new_test_ext().execute_with(|| {
		// wrong origin.
		let origin = RuntimeOrigin::signed(OtherAccount::get());
		let cid = create_cid(1);
		assert_noop!(CollectiveContent::set_charter(origin, cid), BadOrigin);

		// success.
		let origin = RuntimeOrigin::signed(CharterManager::get());
		let cid = create_cid(2);

		assert_ok!(CollectiveContent::set_charter(origin, cid.clone()));
		assert_eq!(CollectiveContent::charter(), Some(cid.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::NewCharterSet { cid }));

		// reset. success.
		let origin = RuntimeOrigin::signed(CharterManager::get());
		let cid = create_cid(3);

		assert_ok!(CollectiveContent::set_charter(origin, cid.clone()));
		assert_eq!(CollectiveContent::charter(), Some(cid.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::NewCharterSet { cid }));
	});
}

#[test]
fn announce_works() {
	new_test_ext().execute_with(|| {
		let now = frame_system::Pallet::<Test>::block_number();
		// wrong origin.
		let origin = RuntimeOrigin::signed(OtherAccount::get());
		let cid = create_cid(1);

		assert_noop!(CollectiveContent::announce(origin, cid, None), BadOrigin);
		assert_eq!(AnnouncementsCount::<Test>::get(), 0);

		// success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(2);
		let maybe_expire_at = None;

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), maybe_expire_at));
		assert_eq!(NextAnnouncementExpireAt::<Test>::get(), None);
		assert_eq!(AnnouncementsCount::<Test>::get(), 1);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: None,
		}));

		// one more. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(3);
		let maybe_expire_at = None;

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), maybe_expire_at));
		assert_eq!(NextAnnouncementExpireAt::<Test>::get(), None);
		assert_eq!(AnnouncementsCount::<Test>::get(), 2);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: None,
		}));

		// one more with expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(4);
		let maybe_expire_at = DispatchTimeFor::<Test>::After(10);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(NextAnnouncementExpireAt::<Test>::get(), Some(maybe_expire_at.evaluate(now)));
		assert_eq!(AnnouncementsCount::<Test>::get(), 3);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));

		// one more with later expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(5);
		let prev_maybe_expire_at = DispatchTimeFor::<Test>::After(10);
		let maybe_expire_at = DispatchTimeFor::<Test>::At(now + 20);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(
			NextAnnouncementExpireAt::<Test>::get(),
			Some(prev_maybe_expire_at.evaluate(now))
		);
		assert_eq!(AnnouncementsCount::<Test>::get(), 4);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));

		// one more with earlier expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(6);
		let maybe_expire_at = DispatchTimeFor::<Test>::At(now + 5);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(NextAnnouncementExpireAt::<Test>::get(), Some(maybe_expire_at.evaluate(now)));
		assert_eq!(AnnouncementsCount::<Test>::get(), 5);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));
	});
}

#[test]
fn remove_announcement_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(AnnouncementsCount::<Test>::get(), 0);
		// wrong origin.
		let origin = RuntimeOrigin::signed(OtherAccount::get());
		let cid = create_cid(8);

		assert_noop!(CollectiveContent::remove_announcement(origin, cid), BadOrigin);

		// missing announcement.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(9);

		assert_noop!(
			CollectiveContent::remove_announcement(origin, cid),
			Error::<Test>::MissingAnnouncement
		);

		// success.
		// add announcement.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid = create_cid(10);
		assert_ok!(CollectiveContent::announce(origin.clone(), cid.clone(), None));

		// one more announcement.
		let cid_2 = create_cid(11);
		let expire_at_2 = DispatchTimeFor::<Test>::At(10);
		assert_ok!(CollectiveContent::announce(origin.clone(), cid_2.clone(), Some(expire_at_2)));
		// two announcements registered.
		assert!(<Announcements<Test>>::contains_key(cid.clone()));
		assert!(<Announcements<Test>>::contains_key(cid_2.clone()));
		assert_eq!(AnnouncementsCount::<Test>::get(), 2);

		// remove first announcement and assert.
		assert_ok!(CollectiveContent::remove_announcement(origin.clone(), cid.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: cid.clone(),
		}));
		assert_noop!(
			CollectiveContent::remove_announcement(origin.clone(), cid.clone()),
			Error::<Test>::MissingAnnouncement
		);
		assert!(!<Announcements<Test>>::contains_key(cid));
		assert!(<Announcements<Test>>::contains_key(cid_2.clone()));
		assert_eq!(AnnouncementsCount::<Test>::get(), 1);

		// remove second announcement and assert.
		assert_ok!(CollectiveContent::remove_announcement(origin.clone(), cid_2.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: cid_2.clone(),
		}));
		assert_eq!(AnnouncementsCount::<Test>::get(), 0);
		assert_noop!(
			CollectiveContent::remove_announcement(origin, cid_2),
			Error::<Test>::MissingAnnouncement
		);
	});
}

#[test]
fn clean_announcements_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let cleanup_block = 5;

		let announcements: [(_, _); 8] = [
			(create_cid(1), Some(cleanup_block + 5)),
			// expired
			(create_cid(2), Some(cleanup_block - 1)),
			(create_cid(3), Some(cleanup_block + 2)),
			// expired
			(create_cid(4), Some(cleanup_block)),
			(create_cid(5), None),
			// expired
			(create_cid(6), Some(cleanup_block - 2)),
			(create_cid(7), Some(cleanup_block + 3)),
			(create_cid(8), Some(cleanup_block + 4)),
		];
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		for (cid, maybe_expire_at) in announcements.into_iter() {
			assert_ok!(CollectiveContent::announce(
				origin.clone(),
				cid,
				maybe_expire_at
					.map_or(None, |expire_at| Some(DispatchTimeFor::<Test>::At(expire_at)))
			));
		}
		assert_eq!(<Announcements<Test>>::iter_keys().count(), 8);
		assert_eq!(AnnouncementsCount::<Test>::get(), 8);
		System::set_block_number(cleanup_block);

		// invoke `clean_announcements` through the on_idle hook.
		assert_eq!(
			<CollectiveContent as Hooks<_>>::on_idle(cleanup_block, Weight::from_ref_time(20)),
			Weight::from_ref_time(10)
		);
		assert_eq!(<Announcements<Test>>::iter_keys().count(), 5);
		assert_eq!(AnnouncementsCount::<Test>::get(), 5);
		assert_eq!(<NextAnnouncementExpireAt<Test>>::get(), Some(cleanup_block + 2));
		System::assert_has_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: create_cid(2),
		}));
		System::assert_has_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: create_cid(4),
		}));
		System::assert_has_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: create_cid(6),
		}));

		// on_idle. not enough weight.
		assert_eq!(
			<CollectiveContent as Hooks<_>>::on_idle(cleanup_block, Weight::from_ref_time(9)),
			Weight::from_ref_time(0)
		);
	});
}
