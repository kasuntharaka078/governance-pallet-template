//! # Simple Governance Pallet
//!
//! A basic on-chain governance pallet that allows any account to create proposals
//! and have other accounts vote on them. This pallet implements a simple voting
//! mechanism suitable for learning FRAME development.
//!
//! ## Overview
//!
//! This pallet provides the following functionality:
//! - Any user can propose a new vote with a short description
//! - Each proposal has a voting period defined by block numbers
//! - Network members can vote 'for' or 'against' each proposal
//! - Each account may vote once per proposal
//! - Results (for/against counts) are stored on-chain permanently
//! - Proposals automatically close when their end block is reached
//! - Anyone can manually close a proposal once the voting period has ended
//! - Events are emitted for proposing, voting, and closing proposals
//!
//! ## Usage
//!
//! ### Creating a Proposal
//! ```ignore
//! // Create a proposal with description "Increase block rewards"
//! let description = b"Increase block rewards".to_vec();
//! SimpleGovernance::propose(origin, description)?;
//! ```
//!
//! ### Voting on a Proposal
//! ```ignore
//! // Vote 'for' proposal with ID 0
//! SimpleGovernance::vote(origin, 0, true)?;
//! 
//! // Vote 'against' proposal with ID 1  
//! SimpleGovernance::vote(origin, 1, false)?;
//! ```
//!
//! ### Closing a Proposal
//! ```ignore
//! // Manually close proposal with ID 0 (only works if voting period ended)
//! SimpleGovernance::close_proposal(origin, 0)?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Get, ConstU32},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Saturating, Zero};
    use alloc::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// Maximum length of a proposal description.
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        /// Default voting period in blocks.
        #[pallet::constant]
        type DefaultVotingPeriod: Get<BlockNumberFor<Self>>;

        /// Maximum number of proposals that can be auto-closed per block.
        #[pallet::constant]
        type MaxProposalsPerBlock: Get<u32>;
    }

    /// Represents a single governance proposal.
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalId,
        ProposalInfo<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// Tracks votes for each proposal.
    /// Double map: ProposalId -> AccountId -> Vote (true = for, false = against)
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ProposalId,
        Blake2_128Concat,
        T::AccountId,
        bool,
        OptionQuery,
    >;

    /// The next available proposal ID.
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T> = StorageValue<_, ProposalId, ValueQuery>;

    /// Vote tallies for each proposal.
    #[pallet::storage]
    #[pallet::getter(fn vote_tallies)]
    pub type VoteTallies<T> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalId,
        VoteTally,
        OptionQuery,
    >;

    /// Events emitted by the governance pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new proposal was created.
        ProposalCreated {
            proposal_id: ProposalId,
            proposer: T::AccountId,
            description: BoundedVec<u8, ConstU32<256>>,
            end_block: BlockNumberFor<T>,
        },

        /// A vote was cast on a proposal.
        Voted {
            proposal_id: ProposalId,
            voter: T::AccountId,
            vote: bool, // true = for, false = against
        },

        /// A proposal was closed.
        ProposalClosed {
            proposal_id: ProposalId,
            for_votes: u32,
            against_votes: u32,
        },
    }

    /// Errors that can be returned by the governance pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// The proposal description exceeds the maximum allowed length.
        DescriptionTooLong,
        
        /// The specified proposal does not exist.
        ProposalNotFound,
        
        /// The proposal's voting period has not yet ended.
        VotingPeriodNotEnded,
        
        /// The account has already voted on this proposal.
        AlreadyVoted,
        
        /// The proposal is already closed and cannot be voted on.
        ProposalClosed,
        
        /// The voting period has ended and the proposal can no longer be voted on.
        VotingPeriodEnded,
    }

    /// The pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new governance proposal.
        ///
        /// Parameters:
        /// - `origin`: The account creating the proposal
        /// - `description`: A description of the proposal (bounded by MaxDescriptionLength)
        ///
        /// Emits `ProposalCreated` event on success.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::propose())]
        pub fn propose(
            origin: OriginFor<T>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Validate description length against the configured maximum
            ensure!(
                description.len() <= T::MaxDescriptionLength::get() as usize,
                Error::<T>::DescriptionTooLong
            );

            // Create bounded description with fixed size for storage
            let bounded_description: BoundedVec<u8, ConstU32<256>> = 
                description.try_into().map_err(|_| Error::<T>::DescriptionTooLong)?;

            let proposal_id = Self::next_proposal_id();
            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = current_block.saturating_add(T::DefaultVotingPeriod::get());

            let proposal = ProposalInfo {
                proposer: who.clone(),
                description: bounded_description.clone(),
                start_block: current_block,
                end_block,
                is_closed: false,
            };

            // Store the proposal
            Proposals::<T>::insert(&proposal_id, &proposal);
            
            // Initialize vote tally
            VoteTallies::<T>::insert(&proposal_id, VoteTally::default());
            
            // Increment proposal ID for next proposal
            NextProposalId::<T>::mutate(|id| *id = id.saturating_add(1));

            // Emit event
            Self::deposit_event(Event::ProposalCreated {
                proposal_id,
                proposer: who,
                description: bounded_description,
                end_block,
            });

            Ok(())
        }

        /// Vote on an existing proposal.
        ///
        /// Parameters:
        /// - `origin`: The account casting the vote
        /// - `proposal_id`: The ID of the proposal to vote on
        /// - `vote`: The vote (true = for, false = against)
        ///
        /// Emits `Voted` event on success.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            vote: bool,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if proposal exists
            let proposal = Self::proposals(&proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;

            // Check if proposal is closed
            ensure!(!proposal.is_closed, Error::<T>::ProposalClosed);

            // Check if voting period has ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block <= proposal.end_block, Error::<T>::VotingPeriodEnded);

            // Check if account has already voted
            ensure!(
                !Votes::<T>::contains_key(&proposal_id, &who),
                Error::<T>::AlreadyVoted
            );

            // Store the vote
            Votes::<T>::insert(&proposal_id, &who, vote);

            // Update vote tally
            VoteTallies::<T>::mutate(&proposal_id, |tally_opt| {
                if let Some(tally) = tally_opt {
                    if vote {
                        tally.for_votes = tally.for_votes.saturating_add(1);
                    } else {
                        tally.against_votes = tally.against_votes.saturating_add(1);
                    }
                }
            });

            // Emit event
            Self::deposit_event(Event::Voted {
                proposal_id,
                voter: who,
                vote,
            });

            Ok(())
        }

        /// Manually close a proposal whose voting period has ended.
        ///
        /// Parameters:
        /// - `origin`: The account closing the proposal
        /// - `proposal_id`: The ID of the proposal to close
        ///
        /// Emits `ProposalClosed` event on success.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::close_proposal())]
        pub fn close_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Check if proposal exists
            let mut proposal = Self::proposals(&proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;

            // Check if proposal is already closed
            ensure!(!proposal.is_closed, Error::<T>::ProposalClosed);

            // Check if voting period has ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block > proposal.end_block, Error::<T>::VotingPeriodNotEnded);

            // Close the proposal
            proposal.is_closed = true;
            Proposals::<T>::insert(&proposal_id, &proposal);

            // Get vote tally for event
            let tally = Self::vote_tallies(&proposal_id).unwrap_or_default();

            // Emit event
            Self::deposit_event(Event::ProposalClosed {
                proposal_id,
                for_votes: tally.for_votes,
                against_votes: tally.against_votes,
            });

            Ok(())
        }
    }

    /// Hook that runs at the beginning of each block.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();
            let mut closed_count = 0u32;
            let max_closures = T::MaxProposalsPerBlock::get();

            // Iterate through proposals and auto-close expired ones
            for (proposal_id, mut proposal) in Proposals::<T>::iter() {
                if closed_count >= max_closures {
                    break;
                }

                if !proposal.is_closed && n > proposal.end_block {
                    // Close the proposal
                    proposal.is_closed = true;
                    Proposals::<T>::insert(&proposal_id, &proposal);

                    // Get vote tally for event
                    let tally = Self::vote_tallies(&proposal_id).unwrap_or_default();

                    // Emit event
                    Self::deposit_event(Event::ProposalClosed {
                        proposal_id,
                        for_votes: tally.for_votes,
                        against_votes: tally.against_votes,
                    });

                    closed_count = closed_count.saturating_add(1);
                    weight = weight.saturating_add(T::WeightInfo::close_proposal());
                }
            }

            weight
        }
    }

    /// Type alias for proposal IDs.
    pub type ProposalId = u32;

    /// Information about a governance proposal.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ProposalInfo<AccountId, BlockNumber> 
    where
        AccountId: MaxEncodedLen,
        BlockNumber: MaxEncodedLen,
    {
        /// The account that created the proposal.
        pub proposer: AccountId,
        /// The proposal description.
        pub description: BoundedVec<u8, ConstU32<256>>,
        /// The block when the proposal was created.
        pub start_block: BlockNumber,
        /// The block when voting ends.
        pub end_block: BlockNumber,
        /// Whether the proposal has been closed.
        pub is_closed: bool,
    }

    /// Vote tally for a proposal.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct VoteTally {
        /// Number of votes in favor.
        pub for_votes: u32,
        /// Number of votes against.
        pub against_votes: u32,
    }

    /// Genesis configuration for the pallet.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial proposals to create at genesis.
        pub proposals: Vec<(T::AccountId, Vec<u8>)>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                proposals: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (proposer, description) in &self.proposals {
                // Validate description length against the configured maximum
                if description.len() > T::MaxDescriptionLength::get() as usize {
                    panic!("Description too long in genesis config");
                }

                let bounded_description: BoundedVec<u8, ConstU32<256>> = 
                    description.clone().try_into().expect("Description too long in genesis config");

                let proposal_id = NextProposalId::<T>::get();
                let current_block = BlockNumberFor::<T>::zero();
                let end_block = current_block.saturating_add(T::DefaultVotingPeriod::get());

                let proposal = ProposalInfo {
                    proposer: proposer.clone(),
                    description: bounded_description.clone(),
                    start_block: current_block,
                    end_block,
                    is_closed: false,
                };

                Proposals::<T>::insert(&proposal_id, &proposal);
                VoteTallies::<T>::insert(&proposal_id, VoteTally::default());
                NextProposalId::<T>::mutate(|id| *id = id.saturating_add(1));

                Pallet::<T>::deposit_event(Event::ProposalCreated {
                    proposal_id,
                    proposer: proposer.clone(),
                    description: bounded_description,
                    end_block,
                });
            }
        }
    }
}