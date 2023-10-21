use std::collections::HashMap;

pub struct BalancesModule {
    balance: HashMap<u32, u32>,
}

impl BalancesModule {
    pub fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, account_id: u32, amount: u32) {
        self.balance.insert(account_id, amount);
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u32) -> Result<(), &str> {
        let old_balance_from = self.balance.get(&from).ok_or("account id doesn't exist")?;
        // if account id doesn't exist, then just assume 0 balance & then still transfer the amount
        let old_balance_to = self.balance.get(&to).unwrap_or(&0);

        let new_balance_from = old_balance_from
            .checked_sub(amount)
            .ok_or("invalid subtraction")?;
        let new_balance_to = old_balance_to
            .checked_add(amount)
            .ok_or("invalid addition")?;

        self.set_balance(from, new_balance_from);
        self.set_balance(to, new_balance_to);

        Ok(())
    }

    pub fn get_balance(&self, account_id: u32) -> Option<&u32> {
        self.balance.get(&account_id)
    }
}
