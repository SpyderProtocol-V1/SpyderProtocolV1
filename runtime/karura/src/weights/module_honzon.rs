// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-04, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-latest"), DB CACHE: 128

// Executed Command:
// target/release/Spyder
// benchmark
// --chain=karura-latest
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_honzon.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon::WeightInfo for WeightInfo<T> {
	fn authorize() -> Weight {
		(59_102_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize() -> Weight {
		(61_468_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize_all(c: u32, ) -> Weight {
		(29_009_000 as Weight)
			// Standard Error: 73_000
			.saturating_add((46_110_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	fn adjust_loan() -> Weight {
		(277_577_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(20 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn transfer_loan_from() -> Weight {
		(184_868_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn close_loan_has_debit_by_dex() -> Weight {
		(444_583_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(22 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
}