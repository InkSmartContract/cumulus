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
use frame_support::{assert_noop, assert_ok, error::BadOrigin};

#[test]
fn set_charter_works() {
	new_test_ext().execute_with(|| {
		// wrong origin.
		let origin = RuntimeOrigin::signed(OtherAccount::get());
		let cid: Cid = b"ipfs_hash_fail".to_vec().try_into().unwrap();

		assert_noop!(CollectiveContent::set_charter(origin, cid), BadOrigin);

		// success.
		let origin = RuntimeOrigin::signed(CharterManager::get());
		let cid: Cid = b"ipfs_hash_success".to_vec().try_into().unwrap();

		assert_ok!(CollectiveContent::set_charter(origin, cid.clone()));
		assert_eq!(CollectiveContent::charter(), Some(cid.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::NewCharterSet { cid }));

		// reset. success.
		let origin = RuntimeOrigin::signed(CharterManager::get());
		let cid: Cid = b"ipfs_hash_reset_success".to_vec().try_into().unwrap();

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
		let cid: Cid = b"ipfs_hash_fail".to_vec().try_into().unwrap();

		assert_noop!(CollectiveContent::announce(origin, cid, None), BadOrigin);

		// success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success".to_vec().try_into().unwrap();
		let maybe_expire_at = None;

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), maybe_expire_at));
		assert_eq!(NextAnnouncementExpire::<Test>::get(), None);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: None,
		}));

		// one more. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let maybe_expire_at = None;

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), maybe_expire_at));
		assert_eq!(NextAnnouncementExpire::<Test>::get(), None);
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: None,
		}));

		// one more with expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let maybe_expire_at = DispatchTimeFor::<Test>::After(10);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(NextAnnouncementExpire::<Test>::get(), Some(maybe_expire_at.evaluate(now)));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));

		// one more with later expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let prev_maybe_expire_at = DispatchTimeFor::<Test>::After(10);
		let maybe_expire_at = DispatchTimeFor::<Test>::At(now + 20);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(NextAnnouncementExpire::<Test>::get(), Some(prev_maybe_expire_at.evaluate(now)));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));

		// one more with earlier expire. success.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let maybe_expire_at = DispatchTimeFor::<Test>::At(now + 5);

		assert_ok!(CollectiveContent::announce(origin, cid.clone(), Some(maybe_expire_at)));
		assert_eq!(NextAnnouncementExpire::<Test>::get(), Some(maybe_expire_at.evaluate(now)));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementAnnounced {
			cid,
			maybe_expire_at: Some(maybe_expire_at.evaluate(now)),
		}));

		// too many announcements.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let maybe_expire_at = None;

		assert_noop!(
			CollectiveContent::announce(origin, cid, maybe_expire_at),
			Error::<Test>::TooManyAnnouncements
		);
	});
}

#[test]
fn remove_announcement_works() {
	new_test_ext().execute_with(|| {
		// wrong origin.
		let origin = RuntimeOrigin::signed(OtherAccount::get());
		let cid: Cid = b"ipfs_hash_fail".to_vec().try_into().unwrap();

		assert_noop!(CollectiveContent::remove_announcement(origin, cid), BadOrigin);

		// missing announcement.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_missing".to_vec().try_into().unwrap();

		assert_noop!(
			CollectiveContent::remove_announcement(origin, cid),
			Error::<Test>::MissingAnnouncement
		);

		// success.
		// add announcement.
		let origin = RuntimeOrigin::signed(AnnouncementManager::get());
		let cid: Cid = b"ipfs_hash_success".to_vec().try_into().unwrap();
		assert_ok!(CollectiveContent::announce(origin.clone(), cid.clone(), None));

		// one more announcement.
		let cid_2: Cid = b"ipfs_hash_success_2".to_vec().try_into().unwrap();
		let expire_at_2 = DispatchTimeFor::<Test>::At(10);
		assert_ok!(CollectiveContent::announce(origin.clone(), cid_2.clone(), Some(expire_at_2)));
		// two announcements registered.
		assert_eq!(<Announcements<Test>>::get().len(), 2);

		// remove first announcement and assert.
		assert_ok!(CollectiveContent::remove_announcement(origin.clone(), cid.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: cid.clone(),
		}));
		assert_noop!(
			CollectiveContent::remove_announcement(origin.clone(), cid),
			Error::<Test>::MissingAnnouncement
		);
		assert_eq!(<Announcements<Test>>::get().len(), 1);

		// remove second announcement and assert.
		assert_ok!(CollectiveContent::remove_announcement(origin.clone(), cid_2.clone()));
		System::assert_last_event(RuntimeEvent::CollectiveContent(Event::AnnouncementRemoved {
			cid: cid_2.clone(),
		}));
		assert_noop!(
			CollectiveContent::remove_announcement(origin, cid_2),
			Error::<Test>::MissingAnnouncement
		);
	});
}
