use std::collections::BTreeMap;
use num::{CheckedAdd,CheckedSub,Zero};

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd+ CheckedSub + Zero + Copy;
}
#[derive(Debug)]
pub struct Pallet<T:Config> {
    balances: BTreeMap<T::AccountId,T::Balance>
}

impl<T> Pallet<T> where T: Config {
    
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,user: &T::AccountId,amount:T::Balance) {

        self.balances.insert(user.clone(), amount);
    }

    pub fn get_balances(&self, user: &T::AccountId) -> T::Balance {
        *self.balances.get(user).unwrap_or(&T::Balance::zero())
    }


    pub fn transfer(&mut self, caller: &T::AccountId,to: &T::AccountId,amount:T::Balance) -> crate::support::DispatchResult {

 
        let balance_caller = self.get_balances(&caller);
        let new_from_balance = balance_caller.checked_sub(&amount).ok_or("Underflow")?;
        self.balances.insert(caller.clone(), new_from_balance); 

        let balance_to = self.get_balances(&to);
        let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow")?;
        self.balances.insert(to.clone(), new_balance_to);

        Ok(())
    }
}


pub enum Call<T:Config> {
    Transfer {
        to :T::AccountId,
        amount :T::Balance
    }
}


impl<T: Config> crate::support::Dispatch for Pallet<T> {
    
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }

        Ok(())
    }
}





#[cfg(test)]
mod tests {
    
    use super::*;
    use rstest::{rstest,fixture};

    #[derive(Debug)]
    struct testConfig;

    impl super::Config for testConfig {

        type Balance = u128;
    }

    impl crate::system::Config for testConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn test_new_creation_of_pallet() {
        let p1:Pallet<testConfig> = Pallet::new();
        let empty = *p1.balances.get(&"meow".to_string()).unwrap_or(&0);
        assert_eq!(empty,0);
    }


    #[test]
    fn test_set_balance() {
        let mut p1 = Pallet::<testConfig>::new();
        p1.set_balance(&"Kitty".to_string(), 1000);

        let balance_kitty = p1.get_balances(&"Kitty".to_string());
        assert_eq!(balance_kitty,1000);
    }

    
    #[fixture]
    fn create_two_users_with_1000_balance() -> Pallet::<testConfig>{
        let user1 = String::from("Alice");
        let user2 = String::from("Bob");

        let mut pallet = Pallet::new();

        pallet.set_balance(&user1, 1000);  
        pallet.set_balance(&user2, 1000);  

        pallet
    }

    #[rstest]
    fn test_balance_transfer(create_two_users_with_1000_balance:Pallet::<testConfig>) {
        let mut pallet = create_two_users_with_1000_balance;
        println!("{:?}",pallet);
        
   

        match pallet.transfer(&"Alice".to_string(), &"Bob".to_owned(), 1000) {
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