#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;
	use frame_support::Blake2_128Concat;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	use scale_info::TypeInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Student {
		pub id: Vec<u8>,
		pub name: Vec<u8>,
		pub age: u8,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn students)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Students<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber), ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Create Student
		CreateStudent(T::AccountId, T::Hash),

		// Delete Student
		DeleteStudent(T::AccountId, T::Hash),

		// Transfer Student
		TransferStudent(T::AccountId, T::AccountId, T::Hash),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StudentExist,
		StudentNotExist,
		NotStudentOwner,
		TransferToSelf,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn create_student(
			origin: OriginFor<T>,
			id: Vec<u8>,
			name: Vec<u8>,
			age: u8,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let student = Student { id: id.clone(), name, age };

			let student_id = T::Hashing::hash_of(&student);

			let current_block = <frame_system::Pallet<T>>::block_number();

			ensure!(!<Students<T>>::contains_key(&student_id), <Error<T>>::StudentExist);

			<Students<T>>::insert(&student_id, (&sender, current_block));

			Self::deposit_event(Event::CreateStudent(sender, student_id));

			//log::info!("A student is created {:?}", student);

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn delete_student(
			origin: OriginFor<T>,
			id: Vec<u8>,
			name: Vec<u8>,
			age: u8,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let student = Student { id: id.clone(), name, age };

			let student_id = T::Hashing::hash_of(&student);

			// Check student exist
			ensure!(<Students<T>>::contains_key(&student_id), <Error<T>>::StudentNotExist);

			// Get student owner
			let (owner, _) = Students::<T>::get(&student_id);

			// Ensure the student exists and is called by the student owner
			ensure!(sender == owner, <Error<T>>::NotStudentOwner);

			<Students<T>>::remove(student_id);

			Self::deposit_event(Event::DeleteStudent(sender, student_id));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn transfer_student(
			origin: OriginFor<T>,
			to: T::AccountId,
			id: Vec<u8>,
			name: Vec<u8>,
			age: u8,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let student = Student { id: id.clone(), name, age };

			let student_id = T::Hashing::hash_of(&student);

			Self::transfer_student_to(&student_id, &from, &to)?;

			Self::deposit_event(Event::TransferStudent(from, to, student_id));

			Ok(())
		}
	}

	//** Our helper functions.**//
	impl<T: Config> Pallet<T> {
		pub fn transfer_student_to(
			student_id: &T::Hash,
			from: &T::AccountId,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			// Check student exist
			ensure!(<Students<T>>::contains_key(&student_id), <Error<T>>::StudentNotExist);

			// Get student owner
			let (owner, _) = Students::<T>::get(&student_id);

			// Ensure the student exists and is called by the student owner
			ensure!(from == &owner, <Error<T>>::NotStudentOwner);

			// Verify the student is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			<Students<T>>::remove(student_id);

			let current_block = <frame_system::Pallet<T>>::block_number();

			<Students<T>>::insert(&student_id, (to, current_block));

			Ok(())
		}
	}
}
