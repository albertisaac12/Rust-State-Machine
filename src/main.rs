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
    type Balance = u128;
}

impl Runtime {
    fn new() -> Self {
        let system = sys_Pallet::new();
        let balances = bal_Pallet::new();
        Self { system, balances}
    }


    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult{

        self.system.inc_block_number();
        
        if block.header.block_number != self.system.block_number() {
            return Err("Block Number Mismatch");
        }

        for (i,support::Extrinsic {caller,call}) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| eprintln!("Error Message: {e} Block Number: {0} Externsic Number{1}",block.header.block_number,i));
        }
        
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
	
        match  runtime_call {
            RuntimeCall::BalancesTransfer(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }

		Ok(())
	}
}


pub enum RuntimeCall { 

    BalancesTransfer(balances::Call<Runtime>)
}

fn main() {
    let mut runtime = Runtime::new();

    let block1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![support::Extrinsic {
			caller: "alice".to_string(),
			call: RuntimeCall::BalancesTransfer(balances::Call::Transfer { to: "bob".to_string(), amount: 69 }),
		},] 
    };

    runtime.execute_block(block1).expect("meow");
  
    println!("{:#?}",runtime);
}
