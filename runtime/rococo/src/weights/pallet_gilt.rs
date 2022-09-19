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
//! Autogenerated weights for `pallet_gilt`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_gilt
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_gilt`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_gilt::WeightInfo for WeightInfo<T> {
	// Storage: Gilt Queues (r:1 w:1)
	// Storage: Gilt QueueTotals (r:1 w:1)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		Weight::from_ref_time(36_050_000 as u64)
			// Standard Error: 377
			.saturating_add(Weight::from_ref_time(67_575 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Gilt Queues (r:1 w:1)
	// Storage: Gilt QueueTotals (r:1 w:1)
	fn place_bid_max() -> Weight {
		Weight::from_ref_time(102_473_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Gilt Queues (r:1 w:1)
	// Storage: Gilt QueueTotals (r:1 w:1)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		Weight::from_ref_time(38_000_000 as u64)
			// Standard Error: 374
			.saturating_add(Weight::from_ref_time(52_686 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Gilt ActiveTotal (r:1 w:1)
	fn set_target() -> Weight {
		Weight::from_ref_time(7_205_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Gilt Active (r:1 w:1)
	// Storage: Gilt ActiveTotal (r:1 w:1)
	fn thaw() -> Weight {
		Weight::from_ref_time(47_658_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Gilt ActiveTotal (r:1 w:0)
	fn pursue_target_noop() -> Weight {
		Weight::from_ref_time(3_161_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Gilt ActiveTotal (r:1 w:1)
	// Storage: Gilt QueueTotals (r:1 w:1)
	// Storage: Gilt Queues (r:1 w:1)
	// Storage: Gilt Active (r:0 w:1)
	/// The range of component `b` is `[1, 1000]`.
	fn pursue_target_per_item(b: u32, ) -> Weight {
		Weight::from_ref_time(40_127_000 as u64)
			// Standard Error: 1_416
			.saturating_add(Weight::from_ref_time(3_962_912 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(b as u64)))
	}
	// Storage: Gilt ActiveTotal (r:1 w:1)
	// Storage: Gilt QueueTotals (r:1 w:1)
	// Storage: Gilt Queues (r:1 w:1)
	// Storage: Gilt Active (r:0 w:1)
	/// The range of component `q` is `[1, 300]`.
	fn pursue_target_per_queue(q: u32, ) -> Weight {
		Weight::from_ref_time(39_231_000 as u64)
			// Standard Error: 3_015
			.saturating_add(Weight::from_ref_time(6_678_147 as u64).saturating_mul(q as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(q as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(q as u64)))
	}
}
