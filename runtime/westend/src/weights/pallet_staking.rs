// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking::WeightInfo for WeightInfo<T> {
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn bond() -> Weight {
		// Minimum execution time: 47_864 nanoseconds.
		Weight::from_ref_time(48_374_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra() -> Weight {
		// Minimum execution time: 92_224 nanoseconds.
		Weight::from_ref_time(92_673_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	fn unbond() -> Weight {
		// Minimum execution time: 89_173 nanoseconds.
		Weight::from_ref_time(91_518_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Minimum execution time: 41_525 nanoseconds.
		Weight::from_ref_time(42_785_087 as u64)
			// Standard Error: 689
			.saturating_add(Weight::from_ref_time(16_918 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(_s: u32, ) -> Weight {
		// Minimum execution time: 77_930 nanoseconds.
		Weight::from_ref_time(79_791_309 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(11 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	fn validate() -> Weight {
		// Minimum execution time: 61_039 nanoseconds.
		Weight::from_ref_time(61_864_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	/// The range of component `k` is `[1, 128]`.
	fn kick(k: u32, ) -> Weight {
		// Minimum execution time: 33_744 nanoseconds.
		Weight::from_ref_time(31_656_370 as u64)
			// Standard Error: 7_390
			.saturating_add(Weight::from_ref_time(6_320_562 as u64).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Minimum execution time: 64_103 nanoseconds.
		Weight::from_ref_time(63_617_315 as u64)
			// Standard Error: 5_476
			.saturating_add(Weight::from_ref_time(2_416_112 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		// Minimum execution time: 59_072 nanoseconds.
		Weight::from_ref_time(59_664_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		// Minimum execution time: 16_287 nanoseconds.
		Weight::from_ref_time(16_474_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		// Minimum execution time: 23_295 nanoseconds.
		Weight::from_ref_time(23_742_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		// Minimum execution time: 4_606 nanoseconds.
		Weight::from_ref_time(4_753_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		// Minimum execution time: 4_714 nanoseconds.
		Weight::from_ref_time(4_870_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		// Minimum execution time: 4_811 nanoseconds.
		Weight::from_ref_time(4_915_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		// Minimum execution time: 4_815 nanoseconds.
		Weight::from_ref_time(4_888_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	/// The range of component `v` is `[0, 1000]`.
	fn set_invulnerables(v: u32, ) -> Weight {
		// Minimum execution time: 4_957 nanoseconds.
		Weight::from_ref_time(5_458_970 as u64)
			// Standard Error: 37
			.saturating_add(Weight::from_ref_time(11_689 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn force_unstake(s: u32, ) -> Weight {
		// Minimum execution time: 72_209 nanoseconds.
		Weight::from_ref_time(77_298_663 as u64)
			// Standard Error: 3_183
			.saturating_add(Weight::from_ref_time(888_800 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// The range of component `s` is `[1, 1000]`.
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		// Minimum execution time: 118_262 nanoseconds.
		Weight::from_ref_time(930_422_429 as u64)
			// Standard Error: 58_358
			.saturating_add(Weight::from_ref_time(4_891_639 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[0, 64]`.
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		// Minimum execution time: 79_513 nanoseconds.
		Weight::from_ref_time(93_058_704 as u64)
			// Standard Error: 20_402
			.saturating_add(Weight::from_ref_time(19_965_835 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `n` is `[0, 64]`.
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		// Minimum execution time: 91_791 nanoseconds.
		Weight::from_ref_time(119_133_841 as u64)
			// Standard Error: 31_093
			.saturating_add(Weight::from_ref_time(26_717_359 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	/// The range of component `l` is `[1, 32]`.
	fn rebond(l: u32, ) -> Weight {
		// Minimum execution time: 82_457 nanoseconds.
		Weight::from_ref_time(83_869_512 as u64)
			// Standard Error: 4_174
			.saturating_add(Weight::from_ref_time(46_937 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn reap_stash(s: u32, ) -> Weight {
		// Minimum execution time: 82_568 nanoseconds.
		Weight::from_ref_time(83_999_520 as u64)
			// Standard Error: 2_098
			.saturating_add(Weight::from_ref_time(884_734 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: VoterList ListBags (r:178 w:0)
	// Storage: VoterList ListNodes (r:101 w:0)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	/// The range of component `v` is `[1, 10]`.
	/// The range of component `n` is `[0, 100]`.
	fn new_era(v: u32, n: u32, ) -> Weight {
		// Minimum execution time: 433_425 nanoseconds.
		Weight::from_ref_time(435_379_000 as u64)
			// Standard Error: 1_833_800
			.saturating_add(Weight::from_ref_time(60_252_147 as u64).saturating_mul(v as u64))
			// Standard Error: 182_728
			.saturating_add(Weight::from_ref_time(13_188_805 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(186 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: VoterList ListBags (r:178 w:0)
	// Storage: VoterList ListNodes (r:1500 w:0)
	// Storage: Staking Nominators (r:1500 w:0)
	// Storage: Staking Validators (r:500 w:0)
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	/// The range of component `v` is `[500, 1000]`.
	/// The range of component `n` is `[500, 1000]`.
	/// The range of component `s` is `[1, 20]`.
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 26_306_369 nanoseconds.
		Weight::from_ref_time(26_515_009_000 as u64)
			// Standard Error: 478_602
			.saturating_add(Weight::from_ref_time(11_814_244 as u64).saturating_mul(v as u64))
			// Standard Error: 478_602
			.saturating_add(Weight::from_ref_time(10_546_073 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(181 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:501 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	/// The range of component `v` is `[500, 1000]`.
	fn get_npos_targets(v: u32, ) -> Weight {
		// Minimum execution time: 3_513_761 nanoseconds.
		Weight::from_ref_time(168_738_313 as u64)
			// Standard Error: 55_974
			.saturating_add(Weight::from_ref_time(7_130_301 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_set() -> Weight {
		// Minimum execution time: 8_232 nanoseconds.
		Weight::from_ref_time(8_510_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_remove() -> Weight {
		// Minimum execution time: 7_257 nanoseconds.
		Weight::from_ref_time(7_425_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill_other() -> Weight {
		// Minimum execution time: 69_220 nanoseconds.
		Weight::from_ref_time(70_070_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	fn force_apply_min_commission() -> Weight {
		// Minimum execution time: 16_197 nanoseconds.
		Weight::from_ref_time(16_518_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
