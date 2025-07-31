use std::collections::BTreeMap;

#[derive(Debug)]
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


    pub fn transfer(&mut self, caller: &str,to: &str,amount:u128) -> Result<(),&'static str> {

        let caller = caller.to_string();
        let to = to.to_string();
        
        let balance_caller = self.get_balances(&caller);
        let new_from_balance = balance_caller.checked_sub(amount).ok_or("Underflow")?;
        self.balances.insert(caller, new_from_balance); 

        let balance_to = self.get_balances(&to);
        let new_balance_to = balance_to.checked_add(amount).ok_or("Overflow")?;
        self.balances.insert(to, new_balance_to);

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    
    use super::*;
    use rstest::{rstest,fixture};

    #[test]
    fn test_new_creation_of_pallet() {
        let p1 = Pallet::new();
        let empty = *p1.balances.get(&"meow".to_string()).unwrap_or(&0);
        assert_eq!(empty,0);
    }


    #[test]
    fn test_set_balance() {
        let mut p1 = Pallet::new();
        p1.set_balance(&"Kitty".to_string(), 1000);

        let balance_kitty = p1.get_balances(&"Kitty".to_string());
        assert_eq!(balance_kitty,1000);
    }

    
    #[fixture]
    fn create_two_users_with_1000_balance() -> Pallet{
        let user1 = String::from("Alice");
        let user2 = String::from("Bob");

        let mut pallet = Pallet::new();

        pallet.set_balance(&user1, 1000);  
        pallet.set_balance(&user2, 1000);  

        pallet
    }

    #[rstest]
    fn test_balance_transfer(create_two_users_with_1000_balance:Pallet) {
        let mut pallet = create_two_users_with_1000_balance;
        println!("{:?}",pallet);
        
        // transferring from Alice to Bob

        match pallet.transfer("Alice", "Bob", 1000) {
            Ok(()) => {
                let balance_alice = pallet.get_balances(&"Alice".to_string());
                let balance_bob = pallet.get_balances(&"Bob".to_string());

                assert_eq!(balance_alice,0);
                assert_eq!(balance_bob,2000);
            },
            Err(_) => {
                assert!(false)
            }
        }



    }
}