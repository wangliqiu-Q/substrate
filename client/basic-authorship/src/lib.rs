
//! Basic implementation of block-authoring logic.
//!
//! # Example
//!
//! ```
//! # use sc_basic_authorship::ProposerFactory;
//! # use sp_consensus::{Environment, Proposer, RecordProof};
//! # use sp_runtime::generic::BlockId;
//! # use std::{sync::Arc, time::Duration};
//! # use substrate_test_runtime_client::{
//! #     runtime::{Extrinsic, Transfer}, AccountKeyring,
//! #     DefaultTestClientBuilderExt, TestClientBuilderExt,
//! # };
//! # use sc_transaction_pool::{BasicPool, FullChainApi};
//! # let client = Arc::new(substrate_test_runtime_client::new());
//! # let spawner = sp_core::testing::TaskExecutor::new();
//! # let txpool = BasicPool::new_full(
//! #     Default::default(),
//! #     None,
//! #     spawner,
//! #     client.clone(),
//! # );
//! // The first step is to create a `ProposerFactory`.
//! let mut proposer_factory = ProposerFactory::new(client.clone(), txpool.clone(), None);
//!
//! // From this factory, we create a `Proposer`.
//! let proposer = proposer_factory.init(
//! 	&client.header(&BlockId::number(0)).unwrap().unwrap(),
//! );
//!
//! // The proposer is created asynchronously.
//! let proposer = futures::executor::block_on(proposer).unwrap();
//!
//! // This `Proposer` allows us to create a block proposition.
//! // The proposer will grab transactions from the transaction pool, and put them into the block.
//! let future = proposer.propose(
//! 	Default::default(),
//! 	Default::default(),
//! 	Duration::from_secs(2),
//! 	RecordProof::Yes,
//! );
//!
//! // We wait until the proposition is performed.
//! let block = futures::executor::block_on(future).unwrap();
//! println!("Generated block: {:?}", block.block);
//! ```
//!

mod basic_authorship;

pub use crate::basic_authorship::{ProposerFactory, Proposer};
