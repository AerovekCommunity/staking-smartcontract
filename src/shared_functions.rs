multiversx_sc::imports!();
use crate::err_and_const::ERR_NOTHING_STAKED;
#[multiversx_sc::module]
pub trait SharedFunctions: crate::storage::StakingStorage {

    fn not_enough(&self, rewards: &BigUint) -> bool {
        let available_rewards = self.rewards().get();
        if rewards < &available_rewards {
            self.rewards().update(|value| *value -= rewards);
            false
        } else {
            true
        }
    }

    fn is_not_empty(&self, mapper: SingleValueMapper<BigUint>) -> BigUint {
        if mapper.is_empty() {
            sc_panic!(ERR_NOTHING_STAKED);
        }
        mapper.get()
    }

    fn lock_tokens(&self, user: &ManagedAddress, current_time: u64) {
        if self.lock_staking_state().get() {
            let lock_time = self.lock_time().get();
            let future_time = current_time + lock_time;
            self.unlock_future_time(&user).set(future_time);
        }
    }
}