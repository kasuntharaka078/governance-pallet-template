//! Weights for pallet_simple_governance
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `substrate-benchmark`, CPU: `Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_simple_governance
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/simple-governance/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_simple_governance.
pub trait WeightInfo {
    fn propose() -> Weight;
    fn vote() -> Weight;
    fn close_proposal() -> Weight;
}

/// Weights for pallet_simple_governance using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: SimpleGovernance NextProposalId (r:1 w:1)
    /// Proof: SimpleGovernance NextProposalId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance Proposals (r:0 w:1)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:0 w:1)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn propose() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `76`
        //  Estimated: `3593`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3593)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    
    /// Storage: SimpleGovernance Proposals (r:1 w:0)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance Votes (r:1 w:1)
    /// Proof: SimpleGovernance Votes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:1 w:1)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn vote() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `312`
        //  Estimated: `3777`
        // Minimum execution time: 18_000_000 picoseconds.
        Weight::from_parts(19_000_000, 3777)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    
    /// Storage: SimpleGovernance Proposals (r:1 w:1)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:1 w:0)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn close_proposal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `312`
        //  Estimated: `3777`
        // Minimum execution time: 12_000_000 picoseconds.
        Weight::from_parts(13_000_000, 3777)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: SimpleGovernance NextProposalId (r:1 w:1)
    /// Proof: SimpleGovernance NextProposalId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance Proposals (r:0 w:1)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:0 w:1)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn propose() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `76`
        //  Estimated: `3593`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
    
    /// Storage: SimpleGovernance Proposals (r:1 w:0)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance Votes (r:1 w:1)
    /// Proof: SimpleGovernance Votes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:1 w:1)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn vote() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `312`
        //  Estimated: `3777`
        // Minimum execution time: 18_000_000 picoseconds.
        Weight::from_parts(19_000_000, 3777)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    
    /// Storage: SimpleGovernance Proposals (r:1 w:1)
    /// Proof: SimpleGovernance Proposals (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: SimpleGovernance VoteTallies (r:1 w:0)
    /// Proof: SimpleGovernance VoteTallies (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
    fn close_proposal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `312`
        //  Estimated: `3777`
        // Minimum execution time: 12_000_000 picoseconds.
        Weight::from_parts(13_000_000, 3777)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}