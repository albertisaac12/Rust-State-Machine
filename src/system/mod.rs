use std::collections::BTreeMap;
use num::{Zero,CheckedAdd, One};

pub trait Config {
    type BlockNumber: Zero + CheckedAdd + Copy + One;
    type Nonce: Zero + One + CheckedAdd + Copy;
    type AccountId: Ord + Clone;
}
#[derive(Debug)]
pub struct Pallet<T:Config> { 
    block_number : T::BlockNumber,
    nonce: BTreeMap<T::AccountId,T::Nonce>
}

impl<T> Pallet<T> where T: Config{

	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero() , nonce: BTreeMap::new() }
	}

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero()); // by default return 0
        let new_nonce = current_nonce.checked_add(&T::Nonce::one()).unwrap();
        self.nonce.insert(who.clone(), new_nonce);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    struct testConfig;

    impl super::Config for testConfig{
        type BlockNumber = u32;
        type AccountId = String;
        type Nonce = u32;
    }

    #[test]
    fn test_block_number() {
        let pallet = Pallet::<testConfig>::new();
        assert_eq!(pallet.block_number(),0);
    }

    #[test]
    fn test_inc_block_number() {
        let mut pallet = Pallet::<testConfig>::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number,1);
    }

    #[test]
    fn test_inc_nonce() {
        let mut pallet = Pallet::<testConfig>::new();
        println!("{:?}",pallet.inc_nonce(&"meow".to_owned()));

        assert_eq!(*pallet.nonce.get(&"meow".to_string()).unwrap(),1);
    }


}
