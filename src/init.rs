#![no_std]
multiversx_sc::imports!();
mod storage;
mod staking_settings;
mod staking;
mod err_and_const;
#[multiversx_sc::contract]
pub trait StakingContract: crate::storage::StakingStorage + crate::staking_settings::StakingSettings + crate::staking::Staking
{
    #[init]
    fn init(&self,token_id: TokenIdentifier, apr: u64, locktime: u64, min_stake: BigUint, fee: u64){
        require!(apr > 1u64, "Low APR");
        require!(locktime > 1u64, "Low LockTime");
        require!(min_stake > BigUint::zero(), "Low MinimalStake");
        require!(fee > 0u64, "Low Fee");
        require!(token_id.is_valid_esdt_identifier(),"ESDT not valid form");
        self.initialize_staking(apr,locktime,min_stake,fee);
        self.token_id().set_if_empty(token_id);
    }

    fn initialize_staking(&self,apr:u64,locktime:u64,min_stake: BigUint,fee: u64){
        self.apr().set_if_empty(apr);
        self.lock_time().set_if_empty(locktime);
        self.minimum_stake().set_if_empty(min_stake);
        self.withdraw_fee().set_if_empty(fee);
        
        self.staking_module_state().set_if_empty(true);
        self.lock_staking_state().set_if_empty(true);
        self.early_withdrawing_state().set_if_empty(true);
        self.burn_or_circulate().set_if_empty(true);
        self.rps();

    }
}