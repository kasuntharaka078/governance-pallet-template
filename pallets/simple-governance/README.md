# Simple Governance Pallet

A basic on-chain governance pallet for Substrate FRAME that allows any account to create proposals and have other accounts vote on them.

## Features

- **Proposal Creation**: Any user can propose a new vote with a description
- **Voting Period**: Each proposal has a configurable voting period in blocks
- **Simple Voting**: Users can vote 'for' or 'against' proposals (one vote per account per proposal)
- **Automatic Closure**: Proposals automatically close when their voting period ends
- **Manual Closure**: Anyone can manually close expired proposals
- **On-chain Results**: Vote tallies are stored permanently on-chain
- **Events**: Comprehensive event emission for all actions

## Integration with Substrate Node Template

### 1. Add the Pallet to Your Workspace

Create a new directory `pallets/simple-governance/` and add all the pallet files:

```
pallets/
├── simple-governance/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── mock.rs
│       ├── tests.rs
│       ├── benchmarking.rs
│       └── weights.rs
└── template/
    └── ...
```

### 2. Update Root Cargo.toml

Add the pallet to your workspace dependencies in the root `Cargo.toml`:

```toml
[workspace.dependencies]
# ... existing dependencies ...
pallet-simple-governance = { path = "./pallets/simple-governance", default-features = false }
```

**Important**: Add this line to the `[workspace]` members array as well:
```toml
[workspace]
members = [
    "node",
    "pallets/template",
    "pallets/simple-governance",  # Add this line
    "runtime",
]
```

### 3. Update Runtime Cargo.toml

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
pallet-simple-governance.workspace = true

[features]
std = [
    # ... existing std features ...
    "pallet-simple-governance/std",
]
runtime-benchmarks = [
    # ... existing benchmark features ...
    "pallet-simple-governance/runtime-benchmarks",
]
try-runtime = [
    # ... existing try-runtime features ...
    "pallet-simple-governance/try-runtime",
]
```

### 4. Configure the Pallet in Runtime

Add the pallet configuration to `runtime/src/configs/mod.rs`:

```rust
parameter_types! {
    pub const MaxDescriptionLength: u32 = 256;
    pub const DefaultVotingPeriod: u64 = 7 * DAYS; // 7 days in blocks
    pub const MaxProposalsPerBlock: u32 = 10;
}

impl pallet_simple_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_simple_governance::weights::SubstrateWeight<Runtime>;
    type MaxDescriptionLength = MaxDescriptionLength;
    type DefaultVotingPeriod = DefaultVotingPeriod;
    type MaxProposalsPerBlock = MaxProposalsPerBlock;
}
```

### 5. Add Pallet to Runtime Construction

Update `runtime/src/lib.rs` to include the pallet:

```rust
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Runtime;

    // ... existing pallets ...

    #[runtime::pallet_index(8)] // Choose an available index
    pub type SimpleGovernance = pallet_simple_governance;
}
```

### 6. Add to Benchmarks (Optional)

If you want to include benchmarking, update `runtime/src/benchmarks.rs`:

```rust
frame_benchmarking::define_benchmarks!(
    // ... existing benchmarks ...
    [pallet_simple_governance, SimpleGovernance]
);
```

### 7. Build and Test

```bash
# Build the runtime
cargo build --release

# Run tests
cargo test -p pallet-simple-governance

# Run all tests
cargo test
```

## Usage Examples

### Creating a Proposal

```bash
# Using Polkadot JS Apps or similar interface
SimpleGovernance.propose("Increase validator rewards by 10%")
```

### Voting on a Proposal

```bash
# Vote 'for' proposal ID 0
SimpleGovernance.vote(0, true)

# Vote 'against' proposal ID 0  
SimpleGovernance.vote(0, false)
```

### Closing a Proposal

```bash
# Close proposal ID 0 (only works after voting period ends)
SimpleGovernance.closeProposal(0)
```

## Configuration Parameters

- **MaxDescriptionLength**: Maximum length of proposal descriptions (default: 256 characters)
- **DefaultVotingPeriod**: Duration of voting period in blocks (default: 7 days worth of blocks)
- **MaxProposalsPerBlock**: Maximum proposals that can be auto-closed per block (default: 10)

## Storage Items

- **Proposals**: Maps proposal IDs to proposal information
- **Votes**: Double map tracking individual votes (ProposalId -> AccountId -> Vote)
- **VoteTallies**: Maps proposal IDs to vote counts (for/against)
- **NextProposalId**: Counter for generating unique proposal IDs

## Events

- **ProposalCreated**: Emitted when a new proposal is created
- **Voted**: Emitted when someone votes on a proposal  
- **ProposalClosed**: Emitted when a proposal is closed (manually or automatically)

## Errors

- **DescriptionTooLong**: Proposal description exceeds maximum length
- **ProposalNotFound**: Specified proposal doesn't exist
- **VotingPeriodNotEnded**: Attempted to close proposal before voting period ends
- **AlreadyVoted**: Account has already voted on this proposal
- **ProposalClosed**: Attempted action on already closed proposal
- **VotingPeriodEnded**: Attempted to vote after voting period ended

## Future Extensions

This basic governance pallet can be extended with:

- **Quorum Requirements**: Minimum participation thresholds
- **Weighted Voting**: Stake-based or role-based voting power
- **Proposal Execution**: Automatic dispatch of approved proposals
- **Membership Restrictions**: Limiting who can propose or vote
- **Proposal Deposits**: Requiring stakes to create proposals
- **Vote Delegation**: Allowing accounts to delegate voting power

## Testing

The pallet includes comprehensive unit tests covering:
- Proposal creation and validation
- Voting mechanics and restrictions
- Automatic and manual proposal closure
- Error conditions and edge cases
- Genesis configuration

Run tests with: `cargo test -p pallet-simple-governance`

## License

This pallet is released into the public domain (Unlicense).