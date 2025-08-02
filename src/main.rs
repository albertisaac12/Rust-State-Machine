mod balances;
mod system;

// use std::thread::panicking;

use system::Pallet as sys_Pallet;
use balances::Pallet as bal_Pallet;

#[derive(Debug)]
pub struct Runtime {
    system : sys_Pallet,
    balances: bal_Pallet
}

impl Runtime {
    fn new() -> Self {
        let system = sys_Pallet::new();
        let balances = bal_Pallet::new();
        Self { system, balances}
    }
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
