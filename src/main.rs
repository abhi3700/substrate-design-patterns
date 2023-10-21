#![allow(dead_code)]

mod step_1;

fn main() {
    println!("Substrate code design pattern exploration");
}

mod tests {

    #[cfg(test)]
    mod step_1 {

        use crate::step_1;

        #[test]
        fn test_get_balance() {
            let balances = step_1::BalancesModule::new();

            // get the balances
            assert_eq!(balances.get_balance(1), None);
            assert_eq!(balances.get_balance(2), None);
        }

        #[test]
        fn test_set_balance() {
            let mut balances = step_1::BalancesModule::new();

            // get the balances
            assert_eq!(balances.get_balance(1), None);
            assert_eq!(balances.get_balance(2), None);

            // set the balances
            balances.set_balance(1, 100);
            assert_eq!(balances.get_balance(1), Some(&100));
        }

        #[test]
        #[should_panic(expected = "balance not matching with the set value")]
        fn test_view_balance_fails() {
            let mut balances = step_1::BalancesModule::new();

            // set balance of 1 as 50
            balances.set_balance(1, 50);
            assert_eq!(
                balances.get_balance(1),
                Some(&25),
                "balance not matching with the set value"
            );
        }

        #[test]
        /// balance transfer to existential account
        fn test_transfer_to_existential_ac() {
            let mut balances = step_1::BalancesModule::new();

            // set balance
            balances.set_balance(1, 50);
            balances.set_balance(2, 1); // set bare min. amount

            // check the balances pre transfer
            assert_eq!(balances.get_balance(1), Some(&50));
            assert_eq!(balances.get_balance(2), Some(&1));

            // transfer some amount to an existential account
            // NOTE: existential ac means to have a non-zero balance
            assert!(balances.transfer(1, 2, 40).is_ok());

            // check the balances post transfer
            assert_eq!(balances.get_balance(1), Some(&10));
            assert_eq!(balances.get_balance(2), Some(&41));
        }

        #[test]
        /// balance transfer to non-existential account
        fn test_transfer_to_nonexistential_ac() {
            let mut balances = step_1::BalancesModule::new();

            // set balance
            balances.set_balance(1, 50);

            // check the balances pre transfer
            assert_eq!(balances.get_balance(1), Some(&50));
            assert_eq!(balances.get_balance(2), None);

            // transfer some amount to an non-existential account
            // NOTE: existential ac means to have a non-zero balance
            assert!(balances.transfer(1, 2, 40).is_ok());

            // check the balances post transfer
            assert_eq!(balances.get_balance(1), Some(&10));
            assert_eq!(balances.get_balance(2), Some(&40));
        }

        #[test]
        #[should_panic(expected = "transferring more than sender owns")]
        /// balance transfer fails due to insufficient sender balance
        fn test_transfer_fails_insufficient_balance() {
            let mut balances = step_1::BalancesModule::new();

            // set balance
            balances.set_balance(1, 50);

            // check the balances pre transfer
            assert_eq!(balances.get_balance(1), Some(&50));
            assert_eq!(balances.get_balance(2), None);

            // M-1 (RECOMMENDED)
            // assert!(balances.transfer(1, 2, 51).is_err());
            // M-2
            assert!(
                balances.transfer(1, 2, 51).is_ok(),
                "transferring more than sender owns"
            );
        }

        #[test]
        #[should_panic(expected = "transferring more than a receiver can own")]
        /// balance transfer fails due to invalid balance amount value i.e. when exceeds the `u32::MAX`
        fn test_transfer_fails_when_transferred_amt_exceeds_max() {
            let mut balances = step_1::BalancesModule::new();

            // set balance
            balances.set_balance(1, u32::MAX);
            balances.set_balance(2, 1); // set bare min. balance

            // check the balances pre transfer
            assert_eq!(balances.get_balance(1), Some(&u32::MAX));
            assert_eq!(balances.get_balance(2), Some(&1));

            // M-1 (RECOMMENDED)
            // assert!(balances.transfer(1, 2, u32::MAX).is_err());
            // // M-2
            assert!(
                balances.transfer(1, 2, u32::MAX).is_ok(),
                "transferring more than a receiver can own"
            );
        }
    }
}
