mod balances;
mod system;
mod support;

use system::Pallet as sys_Pallet;
use balances::Pallet as bal_Pallet;

use support::Dispatch;


mod types {
    // use crate::support;

    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type AccountId = String;
    pub type Balance = u128;

    pub type Extrinsic = crate::support::Extrinsic<AccountId,crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header,Extrinsic>;

}

// use types::*;



#[derive(Debug)]
pub struct Runtime {
    system : sys_Pallet::<Self>,
    balances: bal_Pallet::<Self>
}

impl system::Config for Runtime{
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    // type AccountId = String;
    type Balance = u128;
}

impl Runtime {
    fn new() -> Self {
        let system = sys_Pallet::new();
        let balances = bal_Pallet::new();
        Self { system, balances}
    }


    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult{

        /* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/

        self.system.inc_block_number();
        
        if block.header.block_number != self.system.block_number() {
            return Err("Block Number Mismatch");
        }

        for support::Extrinsic {caller,call} in block.extrinsics.into_iter() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| eprintln!("{e} {}",block.header.block_number));
        }
        

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		unimplemented!();
	}
}


pub enum RuntimeCall { // Will represent all the various calls exposed by different Pallets or rather our blockchain to the users

}

fn main() {
    let mut runtime = Runtime::new();

    // Genisis State
    runtime.balances.set_balance(&"Alice".to_string(), 100);

    // First Block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(),1);

    // Introducing a Transaction item
    // if let Ok(x) = runtime.balances.transfer("Alice", "Bob", 30) {
    //     println!("The transfer was successfull");
    // } else {
    //     panic!("Transfer Failed");
    // }
    runtime.system.inc_nonce(&"Alice".to_string());
    let _res = runtime.balances.transfer(&"Alice".to_owned(), &"Bob".to_owned(), 30).map_err(|e| eprintln!("{e}"));
    
    runtime.system.inc_nonce(&"Alice".to_owned());
    let _res = runtime.balances.transfer(&"Alice".to_owned(), &"Charlie".to_string(), 30).map_err(|e| eprintln!("{e}"));

    println!("{:#?}",runtime);
}
