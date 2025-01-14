// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-proxy
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// proxy.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_proxy::weights::WeightInfo;

/// Weights for pallet_proxy using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32) -> Weight {
		// Minimum execution time: 21_677 nanoseconds.
		Weight::from_ref_time(21_692_373 as u64) // Standard Error: 5_242
			.saturating_add(Weight::from_ref_time(117_691 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	// Storage: Proxy Announcements (r:1 w:1)
	// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32) -> Weight {
		// Minimum execution time: 44_286 nanoseconds.
		Weight::from_ref_time(45_544_640 as u64) // Standard Error: 5_443
			.saturating_add(Weight::from_ref_time(164_483 as u64).saturating_mul(a as u64))
			// Standard Error: 5_660
			.saturating_add(Weight::from_ref_time(24_257 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32) -> Weight {
		// Minimum execution time: 28_480 nanoseconds.
		Weight::from_ref_time(29_279_330 as u64) // Standard Error: 4_280
			.saturating_add(Weight::from_ref_time(192_685 as u64).saturating_mul(a as u64))
			// Standard Error: 4_450
			.saturating_add(Weight::from_ref_time(7_973 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32) -> Weight {
		// Minimum execution time: 28_308 nanoseconds.
		Weight::from_ref_time(28_999_683 as u64) // Standard Error: 4_300
			.saturating_add(Weight::from_ref_time(196_827 as u64).saturating_mul(a as u64))
			// Standard Error: 4_471
			.saturating_add(Weight::from_ref_time(14_183 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	// Storage: Proxy Announcements (r:1 w:1)
	// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32) -> Weight {
		// Minimum execution time: 39_239 nanoseconds.
		Weight::from_ref_time(39_072_787 as u64) // Standard Error: 4_455
			.saturating_add(Weight::from_ref_time(190_030 as u64).saturating_mul(a as u64))
			// Standard Error: 4_632
			.saturating_add(Weight::from_ref_time(42_751 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32) -> Weight {
		// Minimum execution time: 29_935 nanoseconds.
		Weight::from_ref_time(30_909_501 as u64) // Standard Error: 5_224
			.saturating_add(Weight::from_ref_time(63_272 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32) -> Weight {
		// Minimum execution time: 29_577 nanoseconds.
		Weight::from_ref_time(30_710_353 as u64) // Standard Error: 7_177
			.saturating_add(Weight::from_ref_time(88_813 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32) -> Weight {
		// Minimum execution time: 24_462 nanoseconds.
		Weight::from_ref_time(25_078_965 as u64) // Standard Error: 4_049
			.saturating_add(Weight::from_ref_time(43_040 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32) -> Weight {
		// Minimum execution time: 32_673 nanoseconds.
		Weight::from_ref_time(33_296_803 as u64) // Standard Error: 3_055
			.saturating_add(Weight::from_ref_time(10_014 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32) -> Weight {
		// Minimum execution time: 26_331 nanoseconds.
		Weight::from_ref_time(26_958_368 as u64) // Standard Error: 2_427
			.saturating_add(Weight::from_ref_time(961 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
