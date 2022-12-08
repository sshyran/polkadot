// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/polkadot/.git/.artifacts/bench.json
// --pallet=pallet_xcm_benchmarks::generic
// --chain=westend-dev
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/westend/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn report_holding() -> Weight {
		Weight::from_ref_time(34_852_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn buy_execution() -> Weight {
		Weight::from_ref_time(5_595_000 as u64)
	}
	// Storage: XcmPallet Queries (r:1 w:0)
	pub(crate) fn query_response() -> Weight {
		Weight::from_ref_time(17_809_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	pub(crate) fn transact() -> Weight {
		Weight::from_ref_time(22_057_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	pub(crate) fn refund_surplus() -> Weight {
		Weight::from_ref_time(5_798_000 as u64)
	}
	pub(crate) fn set_error_handler() -> Weight {
		Weight::from_ref_time(5_720_000 as u64)
	}
	pub(crate) fn set_appendix() -> Weight {
		Weight::from_ref_time(5_711_000 as u64)
	}
	pub(crate) fn clear_error() -> Weight {
		Weight::from_ref_time(5_661_000 as u64)
	}
	pub(crate) fn descend_origin() -> Weight {
		Weight::from_ref_time(6_533_000 as u64)
	}
	pub(crate) fn clear_origin() -> Weight {
		Weight::from_ref_time(5_585_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn report_error() -> Weight {
		Weight::from_ref_time(30_086_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XcmPallet AssetTraps (r:1 w:1)
	pub(crate) fn claim_asset() -> Weight {
		Weight::from_ref_time(21_518_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	pub(crate) fn trap() -> Weight {
		Weight::from_ref_time(5_630_000 as u64)
	}
	// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn subscribe_version() -> Weight {
		Weight::from_ref_time(37_991_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
	pub(crate) fn unsubscribe_version() -> Weight {
		Weight::from_ref_time(8_117_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn initiate_reserve_withdraw() -> Weight {
		Weight::from_ref_time(33_823_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn burn_asset() -> Weight {
		Weight::from_ref_time(7_247_000 as u64)
	}
	pub(crate) fn expect_asset() -> Weight {
		Weight::from_ref_time(5_753_000 as u64)
	}
	pub(crate) fn expect_origin() -> Weight {
		Weight::from_ref_time(5_825_000 as u64)
	}
	pub(crate) fn expect_error() -> Weight {
		Weight::from_ref_time(5_619_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn query_pallet() -> Weight {
		Weight::from_ref_time(34_821_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn expect_pallet() -> Weight {
		Weight::from_ref_time(8_803_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	pub(crate) fn report_transact_status() -> Weight {
		Weight::from_ref_time(30_064_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn clear_transact_status() -> Weight {
		Weight::from_ref_time(5_588_000 as u64)
	}
	pub(crate) fn set_topic() -> Weight {
		Weight::from_ref_time(5_706_000 as u64)
	}
	pub(crate) fn clear_topic() -> Weight {
		Weight::from_ref_time(5_566_000 as u64)
	}
	pub(crate) fn set_fees_mode() -> Weight {
		Weight::from_ref_time(5_537_000 as u64)
	}
	pub(crate) fn unpaid_execution() -> Weight {
		Weight::from_ref_time(5_710_000 as u64)
	}
}
