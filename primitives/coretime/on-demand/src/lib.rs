// Copyright (C) Magnet.
// This file is part of Magnet.

// Magnet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Magnet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Magnet.  If not, see <http://www.gnu.org/licenses/>.

//! # On demand Order Inherent Primitives
//!
//! This crate defines those primitives that should be taken into account when building
//! the on demand order pallet inherent
//!
#![cfg_attr(not(feature = "std"), no_std)]
use cumulus_primitives_core::{
	relay_chain::BlockNumber as RelayBlockNumber, relay_chain::Hash as PHash, ParaId,
	PersistedValidationData,
};
use sp_core::H256;
use sp_runtime::sp_std::vec::Vec;
use sp_runtime::traits::MaybeDisplay;
#[cfg(feature = "std")]
pub mod inherent_client;
pub mod metadata;
pub mod well_known_keys;
use codec::{Codec, Decode, Encode};
use {scale_info::TypeInfo, sp_inherents::InherentIdentifier};

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OrderInherentData<AuthorityId> {
	/// Proof of relaychain storage.
	pub relay_storage_proof: sp_trie::StorageProof,
	/// Validation data.
	pub validation_data: Option<PersistedValidationData>,
	/// Parachain ID.
	pub para_id: ParaId,
	/// Sequence number of order.
	pub sequence_number: u64,
	/// Author of order.
	pub author_pub: Option<AuthorityId>,
}

// Identifier of the order inherent
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"orderiht";

#[derive(Clone, PartialEq)]
pub enum OrderStatus {
	Init,
	Order,
	Execute,
	Complete,
}

#[derive(Clone)]
pub struct OrderRecord<AuthorityId> {
	/// Hash of relaychain block.
	pub relay_parent: Option<PHash>,
	/// Relaychain block height.
	pub relay_height: RelayBlockNumber,
	/// Order status
	pub order_status: OrderStatus,
	/// Validation data.
	pub validation_data: Option<PersistedValidationData>,
	/// Sequence number of order.
	pub sequence_number: u64,
	/// Author of order.
	pub author_pub: Option<AuthorityId>,
	/// Backup transactions hash.
	pub txs: Vec<H256>,
}

impl<AuthorityId> OrderRecord<AuthorityId> {
	pub fn new() -> OrderRecord<AuthorityId> {
		OrderRecord {
			relay_parent: None,
			relay_height: 0,
			order_status: OrderStatus::Init,
			validation_data: None,
			sequence_number: 0,
			author_pub: None,
			txs: Vec::new(),
		}
	}
	pub fn reset(&mut self) {
		self.relay_parent = None;
		self.relay_height = 0;
		self.order_status = OrderStatus::Init;
		self.validation_data = None;
		self.sequence_number = 0;
		self.author_pub = None;
		self.txs = Vec::new();
	}
}

sp_api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait OrderRuntimeApi<Balance, AuthorityId> where
		Balance: Codec + MaybeDisplay,
		AuthorityId:Codec
	{

		fn slot_width()-> u32;

		fn sequence_number()-> u64;

		fn current_relay_height()->u32;

		fn order_max_amount() -> Balance;

		fn order_placed(
			relay_storage_proof: sp_trie::StorageProof,
			validation_data: PersistedValidationData,
			para_id:ParaId,
		)-> Option<AuthorityId>;

		fn reach_txpool_threshold(gas_balance:Balance, core_price:Balance) -> bool;

		fn order_executed(sequence_number:u64) -> bool ;
	}
}

use crate::metadata::MyPhase as Phase;
use crate::metadata::OnDemandEvent;
use crate::metadata::RelaychainRuntimeEvent;
use codec::Error;
use codec::Input;
use frame_support::dispatch::Parameter;
use sp_runtime::traits::Member;
#[derive(Encode, TypeInfo, Debug)]
pub struct MyEventRecord<E: Parameter + Member + Default, T> {
	/// The phase of the block it happened in.
	pub phase: Phase,
	/// The event itself.
	pub event: E,
	/// The list of the topics this event has.
	pub topics: Vec<T>,
}

impl<E: Parameter + Member + Default, T: Decode> Decode for MyEventRecord<E, T> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		log::info!("==============begin===============");
		let p_phase = Phase::decode(input);
		let phase = if let Ok(phase) = p_phase {
			log::info!("=============================phase:{:?}", phase);
			phase
		} else {
			Default::default()
		};
		let p_event = E::decode(input);
		let t_event = if let Ok(event) = p_event {
			log::info!("=============================type_info:{:?}", event);
			event
		} else {
			E::default()
		};

		let p_topics = Vec::<T>::decode(input);
		let topics = if let Ok(topics) = p_topics { topics } else { Vec::new() };
		log::info!("==============end===============");
		// Add your custom logic here if needed

		Ok(MyEventRecord { phase, event: t_event, topics })
	}
}
