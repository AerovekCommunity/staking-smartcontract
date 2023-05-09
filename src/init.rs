#![no_std]
multiversx_sc::imports!();
use crate::err_and_const::{
    ERR_ESDT_INVALID,
    ERR_LOW_FEE,
    ERR_LOW_MINSTAKE,
    ERR_LOW_LOCKTIME,
    ERR_LOW_APR
};
mod storage;
mod staking_settings;
mod staking;
mod err_and_const;
mod avia_staking;
mod shared_functions;
#[multiversx_sc::contract]
pub trait StakingContract: 
crate::storage::StakingStorage 
+ crate::staking_settings::StakingSettings 
+ crate::staking::Staking 
+ crate::avia_staking::AVIAStaking
+ crate::shared_functions::SharedFunctions
{
    #[init]
    fn init(&self,
        token_id: TokenIdentifier, 
        apr: u64, 
        locktime: u64, 
        min_stake: BigUint, 
        fee: u64, 
        avia_token: TokenIdentifier, 
        per_bonus: u64, 
        min_avia_deposit: BigUint, 
        max_avia_staked: u64
    ){
        require!(apr > 1u64, ERR_LOW_APR);
        require!(locktime > 1u64, ERR_LOW_LOCKTIME);
        require!(min_stake > BigUint::zero(), ERR_LOW_MINSTAKE);
        require!(fee > 0u64, ERR_LOW_FEE);
        require!(token_id.is_valid_esdt_identifier() && avia_token.is_valid_esdt_identifier() ,ERR_ESDT_INVALID);
        self.initialize_staking(apr,locktime,min_stake,fee,per_bonus, min_avia_deposit,max_avia_staked);
        self.token_id().set_if_empty(token_id);
        self.avia_token().set_if_empty(avia_token);
    }

    fn initialize_staking(&self,apr:u64,locktime:u64,min_stake: BigUint,fee: u64, per_bonus: u64, min_avia_deposit: BigUint, max_avia_staked: u64){
        self.apr().set_if_empty(apr);
        self.lock_time().set_if_empty(locktime);
        self.minimum_stake().set_if_empty(min_stake);
        self.withdraw_fee().set_if_empty(fee);
        self.percentage_bonus().set_if_empty(per_bonus);
        
        self.staking_module_state().set_if_empty(true);
        self.lock_staking_state().set_if_empty(true);
        self.early_withdrawing_state().set_if_empty(true);
        self.burn_or_circulate().set_if_empty(false);

        self.min_avia_deposit().set_if_empty(min_avia_deposit);
        self.max_avia_staked().set_if_empty(max_avia_staked);
        self.rps();

    }
}