use std::collections::BTreeMap;
use num::{Zero,CheckedAdd, One};


// type BlockNumber = u32;
// type Nonce = u32;
// type AccountId = String;
#[derive(Debug)]
pub struct Pallet<BlockNumber,AccountId,Nonce> { // u32 , String , u32
    block_number : BlockNumber,
    nonce: BTreeMap<AccountId,Nonce>
}

impl<BlockNumber,AccountId,Nonce> Pallet<BlockNumber,AccountId,Nonce> where BlockNumber: Zero + CheckedAdd + Copy + One, AccountId: Ord + Clone, Nonce: Zero + One + CheckedAdd + Copy{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero() , nonce: BTreeMap::new() }
	}

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero()); // by default return 0
        let new_nonce = current_nonce.checked_add(&Nonce::one()).unwrap();
        self.nonce.insert(who.clone(), new_nonce);
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_block_number() {
        let pallet = Pallet::<u32,String,u32>::new();
        assert_eq!(pallet.block_number(),0);
    }

    #[test]
    fn test_inc_block_number() {
        let mut pallet = Pallet::<u32,String,u32>::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number,1);
    }

    #[test]
    fn test_inc_nonce() {
        let mut pallet = Pallet::<u32,String,u32>::new();
        println!("{:?}",pallet.inc_nonce(&"meow".to_owned()));

        assert_eq!(*pallet.nonce.get(&"meow".to_string()).unwrap(),1);
    }


}
