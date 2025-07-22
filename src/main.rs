mod balances;
mod system;

use system::Pallet as sys_Pallet;
use balances::Pallet as bal_Pallet;

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
    println!("Hello, world!");
}
