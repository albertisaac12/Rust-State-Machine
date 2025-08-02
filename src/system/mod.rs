use std::collections::BTreeMap;

type BlockNumber = u32;
type Nonce = u32;
type AccountId = String;
#[derive(Debug)]
pub struct Pallet {
    block_number : BlockNumber,
    nonce: BTreeMap<AccountId,Nonce>
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

    pub fn block_number(&self) -> Nonce {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number+=1;
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&0); // by default return 0
        self.nonce.insert(who.to_string(), current_nonce+1 );
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_block_number() {
        let pallet = Pallet::new();
        assert_eq!(pallet.block_number(),0);
    }

    #[test]
    fn test_inc_block_number() {
        let mut pallet = Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number,1);
    }

    #[test]
    fn test_inc_nonce() {
        let mut pallet = Pallet::new();
        pallet.inc_nonce(&"meow".to_owned());

        assert_eq!(*pallet.nonce.get(&"meow".to_string()).unwrap(),1);
    }


}
