use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String,u128>
}

impl Pallet{
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,user: &String,amount:u128) {

        self.balances.insert(user.clone(), amount);
    }

    pub fn get_balances(&self, user: &String) -> u128 {
        *self.balances.get(user).unwrap_or(&0)
    }

}