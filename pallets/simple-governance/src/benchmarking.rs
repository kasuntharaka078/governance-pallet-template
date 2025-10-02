//! Benchmarking setup for pallet-simple-governance

use super::*;
use crate::Pallet as SimpleGovernance;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use alloc::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn propose() {
        let caller: T::AccountId = whitelisted_caller();
        let description = vec![0u8; T::MaxDescriptionLength::get() as usize];
        
        #[extrinsic_call]
        propose(RawOrigin::Signed(caller.clone()), description.clone());

        // Verify the proposal was created
        assert_eq!(SimpleGovernance::<T>::next_proposal_id(), 1);
        assert!(SimpleGovernance::<T>::proposals(0).is_some());
        
        let proposal = SimpleGovernance::<T>::proposals(0).unwrap();
        assert_eq!(proposal.proposer, caller);
        assert_eq!(proposal.description.into_inner(), description);
    }

    #[benchmark]
    fn vote() {
        let proposer: T::AccountId = whitelisted_caller();
        let voter: T::AccountId = account("voter", 0, 0);
        let description = vec![0u8; 100];
        
        // Create a proposal first
        assert_ok!(SimpleGovernance::<T>::propose(
            RawOrigin::Signed(proposer).into(),
            description
        ));
        
        #[extrinsic_call]
        vote(RawOrigin::Signed(voter.clone()), 0, true);

        // Verify the vote was recorded
        assert_eq!(SimpleGovernance::<T>::votes(0, &voter), Some(true));
        
        let tally = SimpleGovernance::<T>::vote_tallies(0).unwrap();
        assert_eq!(tally.for_votes, 1);
        assert_eq!(tally.against_votes, 0);
    }

    #[benchmark]
    fn close_proposal() {
        let proposer: T::AccountId = whitelisted_caller();
        let closer: T::AccountId = account("closer", 0, 0);
        let description = vec![0u8; 100];
        
        // Create a proposal
        assert_ok!(SimpleGovernance::<T>::propose(
            RawOrigin::Signed(proposer).into(),
            description
        ));
        
        // Add some votes
        let voter1: T::AccountId = account("voter1", 0, 0);
        let voter2: T::AccountId = account("voter2", 0, 0);
        
        assert_ok!(SimpleGovernance::<T>::vote(
            RawOrigin::Signed(voter1).into(),
            0,
            true
        ));
        
        assert_ok!(SimpleGovernance::<T>::vote(
            RawOrigin::Signed(voter2).into(),
            0,
            false
        ));
        
        // Move past voting period by setting the proposal as ended
        // We'll modify the proposal directly for benchmarking purposes
        let current_block = frame_system::Pallet::<T>::block_number();
        Proposals::<T>::mutate(0, |proposal_opt| {
            if let Some(proposal) = proposal_opt {
                proposal.end_block = current_block.saturating_sub(1u32.into());
            }
        });
        
        #[extrinsic_call]
        close_proposal(RawOrigin::Signed(closer), 0);

        // Verify the proposal was closed
        let proposal = SimpleGovernance::<T>::proposals(0).unwrap();
        assert!(proposal.is_closed);
    }

    impl_benchmark_test_suite!(SimpleGovernance, crate::mock::new_test_ext(), crate::mock::Test);
}