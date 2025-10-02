//! Unit tests for the simple governance pallet.

use crate::{mock::*, Error, Event};
use frame_support::{
    assert_noop, assert_ok,
    BoundedVec,
};
use sp_runtime::BuildStorage;

#[test]
fn propose_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let description = b"Test proposal".to_vec();
        let proposer = 1u64;
        
        // Propose should work
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(proposer),
            description.clone()
        ));
        
        // Check that the proposal was created
        let proposal = SimpleGovernance::proposals(0).unwrap();
        assert_eq!(proposal.proposer, proposer);
        assert_eq!(proposal.description.into_inner(), description);
        assert_eq!(proposal.start_block, 1);
        assert_eq!(proposal.end_block, 101); // 1 + DefaultVotingPeriod (100)
        assert!(!proposal.is_closed);
        
        // Check that vote tally was initialized
        let tally = SimpleGovernance::vote_tallies(0).unwrap();
        assert_eq!(tally.for_votes, 0);
        assert_eq!(tally.against_votes, 0);
        
        // Check that next proposal ID was incremented
        assert_eq!(SimpleGovernance::next_proposal_id(), 1);
        
        // Check event was emitted
        System::assert_has_event(
            Event::ProposalCreated {
                proposal_id: 0,
                proposer,
                description: BoundedVec::try_from(description).unwrap(),
                end_block: 101,
            }.into()
        );
    });
}

#[test]
fn propose_fails_with_long_description() {
    new_test_ext().execute_with(|| {
        let long_description = vec![0u8; 300]; // Exceeds MaxDescriptionLength (256)
        
        assert_noop!(
            SimpleGovernance::propose(RuntimeOrigin::signed(1), long_description),
            Error::<Test>::DescriptionTooLong
        );
    });
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal first
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Vote for the proposal
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, true));
        
        // Check that the vote was recorded
        assert_eq!(SimpleGovernance::votes(0, 2), Some(true));
        
        // Check that vote tally was updated
        let tally = SimpleGovernance::vote_tallies(0).unwrap();
        assert_eq!(tally.for_votes, 1);
        assert_eq!(tally.against_votes, 0);
        
        // Vote against the proposal with different account
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(3), 0, false));
        
        // Check updated tally
        let tally = SimpleGovernance::vote_tallies(0).unwrap();
        assert_eq!(tally.for_votes, 1);
        assert_eq!(tally.against_votes, 1);
        
        // Check events were emitted
        System::assert_has_event(
            Event::Voted {
                proposal_id: 0,
                voter: 2,
                vote: true,
            }.into()
        );
        
        System::assert_has_event(
            Event::Voted {
                proposal_id: 0,
                voter: 3,
                vote: false,
            }.into()
        );
    });
}

#[test]
fn vote_fails_nonexistent_proposal() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SimpleGovernance::vote(RuntimeOrigin::signed(1), 999, true),
            Error::<Test>::ProposalNotFound
        );
    });
}

#[test]
fn vote_fails_already_voted() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Vote once
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, true));
        
        // Try to vote again
        assert_noop!(
            SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, false),
            Error::<Test>::AlreadyVoted
        );
    });
}

#[test]
fn vote_fails_after_voting_period() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Move past the voting period manually without calling on_initialize
        System::set_block_number(102);
        
        // Try to vote after period ended
        assert_noop!(
            SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, true),
            Error::<Test>::VotingPeriodEnded
        );
    });
}

#[test]
fn vote_fails_on_closed_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Move past voting period manually and close proposal
        System::set_block_number(102);
        assert_ok!(SimpleGovernance::close_proposal(RuntimeOrigin::signed(1), 0));
        
        // Try to vote on closed proposal
        assert_noop!(
            SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, true),
            Error::<Test>::ProposalClosed
        );
    });
}

#[test]
fn close_proposal_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal and add some votes
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(2), 0, true));
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(3), 0, false));
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(4), 0, true));
        
        // Move past voting period but don't trigger on_initialize
        System::set_block_number(102);
        
        // Close the proposal manually
        assert_ok!(SimpleGovernance::close_proposal(RuntimeOrigin::signed(5), 0));
        
        // Check that proposal is marked as closed
        let proposal = SimpleGovernance::proposals(0).unwrap();
        assert!(proposal.is_closed);
        
        // Check event was emitted with correct vote counts
        System::assert_has_event(
            Event::ProposalClosed {
                proposal_id: 0,
                for_votes: 2,
                against_votes: 1,
            }.into()
        );
    });
}

#[test]
fn close_proposal_fails_nonexistent() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SimpleGovernance::close_proposal(RuntimeOrigin::signed(1), 999),
            Error::<Test>::ProposalNotFound
        );
    });
}

#[test]
fn close_proposal_fails_voting_period_not_ended() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Try to close before voting period ends (ends at block 101)
        System::set_block_number(50);
        
        assert_noop!(
            SimpleGovernance::close_proposal(RuntimeOrigin::signed(1), 0),
            Error::<Test>::VotingPeriodNotEnded
        );
    });
}

#[test]
fn close_proposal_fails_already_closed() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create a proposal
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Test proposal".to_vec()
        ));
        
        // Move past voting period manually and close proposal
        System::set_block_number(102);
        assert_ok!(SimpleGovernance::close_proposal(RuntimeOrigin::signed(1), 0));
        
        // Try to close again
        assert_noop!(
            SimpleGovernance::close_proposal(RuntimeOrigin::signed(1), 0),
            Error::<Test>::ProposalClosed
        );
    });
}

#[test]
fn auto_close_on_initialize_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create multiple proposals
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(1),
            b"Proposal 1".to_vec()
        ));
        assert_ok!(SimpleGovernance::propose(
            RuntimeOrigin::signed(2),
            b"Proposal 2".to_vec()
        ));
        
        // Add some votes
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(3), 0, true));
        assert_ok!(SimpleGovernance::vote(RuntimeOrigin::signed(4), 1, false));
        
        // Move past voting period - this should auto-close proposals
        run_to_block(102);
        
        // Check both proposals are closed
        assert!(SimpleGovernance::proposals(0).unwrap().is_closed);
        assert!(SimpleGovernance::proposals(1).unwrap().is_closed);
        
        // Check events were emitted
        System::assert_has_event(
            Event::ProposalClosed {
                proposal_id: 0,
                for_votes: 1,
                against_votes: 0,
            }.into()
        );
        
        System::assert_has_event(
            Event::ProposalClosed {
                proposal_id: 1,
                for_votes: 0,
                against_votes: 1,
            }.into()
        );
    });
}

#[test]
fn multiple_proposals_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create multiple proposals
        for i in 0..5 {
            assert_ok!(SimpleGovernance::propose(
                RuntimeOrigin::signed(1),
                format!("Proposal {}", i).as_bytes().to_vec()
            ));
        }
        
        // Check all proposals were created
        for i in 0..5 {
            assert!(SimpleGovernance::proposals(i).is_some());
        }
        
        assert_eq!(SimpleGovernance::next_proposal_id(), 5);
    });
}

#[test]
fn genesis_config_works() {
    // Test that genesis config can initialize proposals
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    
    crate::GenesisConfig::<Test> {
        proposals: vec![
            (1u64, b"Genesis proposal 1".to_vec()),
            (2u64, b"Genesis proposal 2".to_vec()),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    
    let mut ext = sp_io::TestExternalities::from(t);
    ext.execute_with(|| {
        // Check proposals were created
        assert_eq!(SimpleGovernance::next_proposal_id(), 2);
        
        let proposal1 = SimpleGovernance::proposals(0).unwrap();
        assert_eq!(proposal1.proposer, 1u64);
        assert_eq!(proposal1.description.into_inner(), b"Genesis proposal 1".to_vec());
        
        let proposal2 = SimpleGovernance::proposals(1).unwrap();
        assert_eq!(proposal2.proposer, 2u64);
        assert_eq!(proposal2.description.into_inner(), b"Genesis proposal 2".to_vec());
    });
}