#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use serde::{Deserialize, Serialize};
use frame_support::traits::Time;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::traits::Randomness;
// use frame_support::sp_runtime::traits::Hash;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;


	pub type BusinessId = u32;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// type Timestamp:
		type Timestamp: Time;
		type Random: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub type Businesses<T: Config> = StorageMap<_, Blake2_128Concat, BusinessId, Business<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		BusinessCreated { business_id: T::Hash, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		DuplicateBusiness,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_business(origin: OriginFor<T>, business_id: BusinessId) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;
			let now:u64 = T::Timestamp::now().saturated_into();
			let new_business = Business::<T>{id:business_id,owner:owner.clone(),created_date:now};

			// Check if the business does not already exist in our storage map
			ensure!(!Businesses::<T>::contains_key(&business_id), Error::<T>::DuplicateBusiness);

			Businesses::<T>::insert(business_id, new_business);

			Ok(())
		}
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct Business<T: Config>{
		pub id: BusinessId,
		pub owner: T::AccountId,
		pub created_date: u64,
	}
}
