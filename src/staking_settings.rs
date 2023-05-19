multiversx_sc::imports!();

use crate::err_and_const::{
    ERR_LOCK_EMPTY, ERR_LOCK_STATE, ERR_LOW, ERR_WITHDRAW_FEE, ERR_WRONG_TOKEN,
};
use crate::storage::{Settings,AVIASettings};
#[multiversx_sc::module]
pub trait StakingSettings: crate::storage::StakingStorage + crate::staking::Staking + crate::avia_staking::AVIAStaking + crate::shared_functions::SharedFunctions {
    //SUPPLY REWARDS FOR STAKING
    #[only_owner]
    #[payable("*")]
    #[endpoint(supplyRewards)]
    fn supply_rewards(&self) {
        let saved_token = self.token_id().get();
        let payment = self.call_value().single_esdt();
        let token = payment.token_identifier;
        require!(token == saved_token, ERR_WRONG_TOKEN);
        let amount = payment.amount;
        self.rewards().update(|rewards| *rewards += amount);
    }
    #[only_owner]
    #[payable("*")]
    #[endpoint(AviaRewards)]
    fn avia_rewards(&self) {
        let saved_token = self.token_id().get();
        let payment = self.call_value().single_esdt();
        let token = payment.token_identifier;
        require!(token == saved_token, ERR_WRONG_TOKEN);
        let amount = payment.amount;
        self.rewards().update(|rewards| *rewards += &amount);
        self.avia_supplied_rewards().update(|rewards| *rewards += &amount);
    }

    //ACTIVATE OR DEACTIVATE STAKING MODULE
    #[only_owner]
    #[endpoint(stakingState)]
    fn staking_state(&self, new_state: bool) {
        match new_state {
            //DEACTIVATE
            false => self.staking_module_state().set(false),
            //ACTIVATE
            true => self.staking_module_state().set(true),
        }
    }

    //CHANGE APR
    #[only_owner]
    #[endpoint(changeAPR)]
    fn change_apr(&self, apr: u64) {
        require!(apr > 0u64, ERR_LOW);
        self.rps();
        self.apr().set(apr);
    }
    //CHANGE MINIMUM STAKING
    #[only_owner]
    #[endpoint(StakeMinimum)]
    fn stake_minimum(&self, minimum: BigUint) {
        require!(minimum > 0u64, ERR_LOW);
        self.minimum_stake().set(minimum);
    }

     //CHANGE AVIA POWER
     #[only_owner]
     #[endpoint(aviaPower)]
     fn change_avia_power(&self, power: BigUint) {
         require!(power > 0u64, ERR_LOW);
         self.avia_power().set(power);
     }
    //LOCK STATE OF STAKING
    #[only_owner]
    #[endpoint(LockStakeState)]
    fn lock_stake_state(&self, new_state: bool) {
        match new_state {
            //DEACTIVATE
            false => {
                self.lock_staking_state().set(false);
            }
            //ACTIVATE
            true => {
                require!(!self.lock_time().is_empty(), ERR_LOCK_EMPTY);
                self.lock_staking_state().set(true);
            }
        }
    }
    //EARLY WITHDRAW STAKE STATE
    #[only_owner]
    #[endpoint(EarlyWithdrawState)]
    fn early_withdraw_state(&self, new_state: bool) {
        match new_state {
            //DEACTIVATE
            false => {
                self.early_withdrawing_state().set(false);
            }
            //ACTIVATE
            true => {
                require!(self.lock_staking_state().get(), ERR_LOCK_STATE);
                require!(!self.withdraw_fee().is_empty(), ERR_WITHDRAW_FEE);
                self.early_withdrawing_state().set(true);
            }
        }
    }
    //CHANGE LOCK TIME FOR LOCKING TOKENS IN STAKING
    #[only_owner]
    #[endpoint(changeLockTime)]
    fn change_lock_time(&self, time: u64) {
        require!(time > 1u64, ERR_LOW);
        self.lock_time().set(time);
    }
    //CHANGE EARLY FEE FOR WITHDRAWING IN LOCK TIME
    #[only_owner]
    #[endpoint(changeWithdrawFee)]
    fn change_withdraw_fee(&self, fee: u64) {
        require!(fee > 0u64, ERR_LOW);
        self.withdraw_fee().set(fee);
    }
    //DECIDE IF THE FEE FROM EARLY WITHDRAWS GET BURNED OR GET BACK TO STAKING VAULT
    #[only_owner]
    #[endpoint(circulateOrBurn)]
    fn circulate_or_burn(&self, state: bool) {
        match state {
            //BURN
            true => {
                self.burn_or_circulate().set(true);
            }
            //CIRCULATE
            false => {
                self.burn_or_circulate().set(false);
            }
        }
    }

    #[view(stakingSettings)]
    fn staking_settings(&self) -> Settings<Self::Api> {
        let apr = self.apr().get();
        let minimum = self.minimum_stake().get();
        let lock_time = self.lock_time().get();
        let early_withdrawing_state = self.early_withdrawing_state().get();
        let burn_or_circulate = self.burn_or_circulate().get();
        let withdraw_fee = self.withdraw_fee().get();
        let token_id = self.token_id().get();

        let staking_settings = Settings {
            token_id,
            apr,
            early_withdrawing_state,
            lock_time,
            burn_or_circulate,
            minimum,
            withdraw_fee,
        };
        staking_settings
    }

    #[view(AviaStakingSettings)]
    fn avia_staking_settings(&self) -> AVIASettings<Self::Api> {
        let avia_id = self.avia_token().get();
        let bonus_apr = self.percentage_bonus().get();
        let min_stake_deposit = self.min_avia_deposit().get();
        let max_avia_staked = self.max_avia_staked().get();

        let staking_settings = AVIASettings {
            avia_id,
            bonus_apr,
            min_stake_deposit,
            max_avia_staked,
        };
        staking_settings
    }
}
