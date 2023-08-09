#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod options_and_futures {
    use ink::{prelude::vec::Vec, storage::traits::StorageLayout};
    use ink::storage::Mapping;

    use scale::{Encode, Decode};

    type Reputation = i128;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Error {
        OnlyOwnerFunction,
        UnregisteredVoter,
        VoterAlreadyVoted,
        VoterEqualToCandidate,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Voter {
        reputation: Reputation,
        address: AccountId,
        available_votes: u128
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct OptionsAndFutures {
        /// Stores a single `bool` value on the storage.
        voters: ink::storage::Mapping<AccountId, Voter>,
        voters_addresses: Vec<AccountId>,
        owner: AccountId,
    }

    impl OptionsAndFutures {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let voters = Mapping::default();
            let voters_addresses: Vec<AccountId> = Vec::new();
            let owner = Self::env().caller();

            Self {
                voters,
                voters_addresses,
                owner
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId, available_votes: u128) -> Result<(), Error> {
            
            if self.env().caller() != self.owner {
                return Err(Error::OnlyOwnerFunction)
            }

            self.voters_addresses.push(voter);
            self.voters.insert(voter, &Voter{reputation: 0, address: voter, available_votes});
            
            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn vote(&mut self, candidate_address: AccountId, votes: i128) -> Result<(), Error> {
            let mut voter: Voter = self.voters.get(&self.env().caller()).ok_or(Error::UnregisteredVoter)?;
            
            if voter.available_votes < (votes.abs() as u128){
                return Err(Error::VoterAlreadyVoted);
            }

            if candidate_address == voter.address {
                return Err(Error::VoterEqualToCandidate);
            }
            
            let mut candidate: Voter = self.voters.get(candidate_address).ok_or(Error::UnregisteredVoter)?;

            candidate.reputation += votes;
            voter.available_votes -= votes.abs() as u128;

            self.voters.insert(candidate_address, &candidate);
            self.voters.insert(&self.env().caller(), &voter);

            Ok(())
        }

        #[ink(message)]
        pub fn remove_voter(&mut self, voter_address: AccountId) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::OnlyOwnerFunction);
            }
            let mut voter: Voter = self.voters.get(&voter_address).ok_or(Error::UnregisteredVoter)?;
            self.voters.remove(voter_address);
            Ok(())
        }

        #[ink(message)]
        pub fn get_voters(&self) -> Result<Vec<Voter>, Error> {
            let mut voters: Vec<Voter> = Vec::new();
            for voter in self.voters_addresses.clone(){
                voters.push(self.voters.get(voter).ok_or(Error::UnregisteredVoter)?);
            }
            Ok(voters)
        }        
    }
}

    // / Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // / module and test functions are marked with a `#[test]` attribute.
    // / The below code is technically just normal Rust code.
//     #[cfg(test)]
//     mod tests {
//         /// Imports all the definitions from the outer scope so we can use them here.
//         use super::*;

//         /// We test if the default constructor does its job.
//         #[ink::test]
//         fn default_works() {
//             let options_and_futures = OptionsAndFutures::default();
//             assert_eq!(options_and_futures.get(), false);
//         }

//         /// We test a simple use case of our contract.
//         #[ink::test]
//         fn it_works() {
//             let mut options_and_futures = OptionsAndFutures::new(false);
//             assert_eq!(options_and_futures.get(), false);
//             options_and_futures.flip();
//             assert_eq!(options_and_futures.get(), true);
//         }
//     }


//     /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
//     ///
//     /// When running these you need to make sure that you:
//     /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
//     /// - Are running a Substrate node which contains `pallet-contracts` in the background
//     #[cfg(all(test, feature = "e2e-tests"))]
//     mod e2e_tests {
//         /// Imports all the definitions from the outer scope so we can use them here.
//         use super::*;

//         /// A helper function used for calling contract messages.
//         use ink_e2e::build_message;

//         /// The End-to-End test `Result` type.
//         type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//         /// We test that we can upload and instantiate the contract using its default constructor.
//         #[ink_e2e::test]
//         async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
//             // Given
//             let constructor = OptionsAndFuturesRef::default();

//             // When
//             let contract_account_id = client
//                 .instantiate("options_and_futures", &ink_e2e::alice(), constructor, 0, None)
//                 .await
//                 .expect("instantiate failed")
//                 .account_id;

//             // Then
//             let get = build_message::<OptionsAndFuturesRef>(contract_account_id.clone())
//                 .call(|options_and_futures| options_and_futures.get());
//             let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
//             assert!(matches!(get_result.return_value(), false));

//             Ok(())
//         }

//         /// We test that we can read and write a value from the on-chain contract contract.
//         #[ink_e2e::test]
//         async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
//             // Given
//             let constructor = OptionsAndFuturesRef::new(false);
//             let contract_account_id = client
//                 .instantiate("options_and_futures", &ink_e2e::bob(), constructor, 0, None)
//                 .await
//                 .expect("instantiate failed")
//                 .account_id;

//             let get = build_message::<OptionsAndFuturesRef>(contract_account_id.clone())
//                 .call(|options_and_futures| options_and_futures.get());
//             let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//             assert!(matches!(get_result.return_value(), false));

//             // When
//             let flip = build_message::<OptionsAndFuturesRef>(contract_account_id.clone())
//                 .call(|options_and_futures| options_and_futures.flip());
//             let _flip_result = client
//                 .call(&ink_e2e::bob(), flip, 0, None)
//                 .await
//                 .expect("flip failed");

//             // Then
//             let get = build_message::<OptionsAndFuturesRef>(contract_account_id.clone())
//                 .call(|options_and_futures| options_and_futures.get());
//             let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//             assert!(matches!(get_result.return_value(), true));

//             Ok(())
//         }
//     }
// }
