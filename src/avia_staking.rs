multiversx_sc::imports!();
use crate::err_and_const::{
    ERR_MODULE_DEACTIVATED, 
    ERR_NOTHING_STAKED,
    ERR_WRONG_AVIA,
    ERR_AVIA_NOT_FOUND,
    ERR_MIN_DEPOSIT_AVIA,
    ERR_MAX_AVIA,
    NFT_AMOUNT, 
    PERCENTAGE, 
    YEAR_IN_SECONDS, 
};
use crate::storage::{
    AviatorStatistics, 
    UserAviatorStatistics, 
    StakersListAvia
};
#[multiversx_sc::module]
pub trait AVIAStaking: crate::storage::StakingStorage + crate::shared_functions::SharedFunctions {

    #[payable("*")]
    #[endpoint(depositAVIA)]
    fn deposit_avia(&self){
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );
        let user = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        let deposit_mapper= self.stake_deposit(&user);
        require!(!deposit_mapper.is_empty(), ERR_NOTHING_STAKED);
        let (token_id, nonce, _amount) = self.call_value().single_esdt().into_tuple();
        let avia_token = self.avia_token().get();
        require!(token_id == avia_token, ERR_WRONG_AVIA);

        let deposit = deposit_mapper.get();
        let min_stake_deposit = self.min_avia_deposit().get();
        let max_avia_stake = self.max_avia_staked().get();
        let staked_aviators_len = self.staked_aviators(&user).len() as u64 + 1;

        require!(staked_aviators_len <= max_avia_stake , ERR_MAX_AVIA);
        require!(deposit >= min_stake_deposit * staked_aviators_len, ERR_MIN_DEPOSIT_AVIA);

        if deposit > BigUint::zero() && !self.staked_aviators(&user).is_empty() {
            let state = self.action_rewards(&user).get();
            match state {
                //CLAIM
                true => {
                    self.safe_avia_rewards(&user);
                }
                //REINVEST
                false => {
                    self.safe_avia_reinvest_rewards(&user);
                }
            }
        } 
        else { 
            self.apr_avia_last_time(&user).set(current_time);
        }
        self.staked_aviators(&user).insert(nonce);
        self.update_list_avia(&user);
        self.staked_aviators_count().update(|count| * count += 1u64);
        self.lock_tokens(&user, current_time);
    }

    #[endpoint(withdrawAVIA)]
    fn withdraw_avia(&self, nonce: u64) {
        let user = self.blockchain().get_caller();
        let avia_token = self.avia_token().get();
        require!(self.staked_aviators(&user).contains(&nonce), ERR_AVIA_NOT_FOUND);
        self.safe_avia_rewards(&user);
        self.staked_aviators(&user).swap_remove(&nonce);
        self.send().direct_esdt(&user, &avia_token, nonce, &BigUint::from(NFT_AMOUNT));
        self.staked_aviators_count().update(|count| * count -= 1u64);
        self.update_list_avia(&user);
    }

 //CLAIM REWARD
    #[endpoint(claimAVIA)]
    fn claim_avia(&self) {
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );
        let current_time = self.blockchain().get_block_timestamp();
        let user = &self.blockchain().get_caller();
        self.safe_avia_rewards(&user);
        self.lock_tokens(&user, current_time);
    }
    //REINVEST REWARD
    #[endpoint(reinvestAVIA)]
    fn reinvest_avia(&self) {
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );
        let user = &self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        self.safe_avia_reinvest_rewards(&user);
        self.update_list_avia(&user);
        self.lock_tokens(&user, current_time);
    }


     //CALCULATION OF REWARD
     fn calculate_avia_reward(&self, user: &ManagedAddress) -> BigUint {
        let current_time = self.blockchain().get_block_timestamp();
        let my_stake = self.is_not_empty(self.stake_deposit(user));
        let avia_len = self.staked_aviators(user).len() as u64;
        let apr_avia_last_time = self.apr_avia_last_time(&user).get();
        let current_apr = avia_len  * self.percentage_bonus().get();
        let diff_time = current_time - apr_avia_last_time;
        let result = my_stake * current_apr * diff_time / PERCENTAGE / YEAR_IN_SECONDS;
        let storage_avia_rewards = self.storage_avia_rewards(user).take();
        let rewards = result + storage_avia_rewards;
        self.apr_avia_last_time(user).set(current_time);
        rewards
     }
 
        fn safe_avia_reinvest_rewards(&self, user: &ManagedAddress) -> BigUint {
            let rewards = self.calculate_avia_reward(&user);
            let not_enough = self.not_enough(&rewards);
            if !not_enough {
                self.total_staked().update(|value| *value += &rewards);
                self.stake_deposit(&user).update(|value| *value += &rewards);
                self.avia_produced_rewards().update(|value| * value += &rewards);
                self.avia_produced_rewards_user(user).update(|value| * value += &rewards);
                self.produced_rewards().update(|value| *value += &rewards);
                self.produced_rewards_user(user)
                    .update(|value| *value += &rewards);
            } else {
                self.storage_avia_rewards(&user)
                    .update(|amount| *amount += &rewards);
            }
            rewards
        }
    
     //ENSURES SAFETY OF REWARDS. IF THERE ARE NO REWARDS PROVIDED FOR CONTRACT, IT WILL SAVE THEM IN REWARD MAPPER
     fn safe_avia_rewards(&self, user: &ManagedAddress) -> BigUint {
        let token_id = self.token_id().get();
        let rewards = self.calculate_avia_reward(&user);
        let not_enough = self.not_enough(&rewards);
        if !not_enough {
            self.send().direct_esdt(user, &token_id, 0u64, &rewards);
            self.avia_produced_rewards().update(|value| * value += &rewards);
            self.avia_produced_rewards_user(user).update(|value| * value += &rewards);
            self.produced_rewards().update(|value| *value += &rewards);
            self.produced_rewards_user(user)
                .update(|value| *value += &rewards);
        } else {
            self.storage_avia_rewards(user)
                .update(|amount| *amount += &rewards);
        }
        rewards
    }

    fn update_list_avia(&self, user: &ManagedAddress) {
        let staked = self.staked_aviators(&user).len();
        let user_node = self.user_avia_node(user);
        if user_node.is_empty() {
            self.next_node_avia_staker(user);
            let staker = StakersListAvia {
                user: user.clone(),
                staked,
            };
            self.stakers_list_avia().push_back(staker);
        } else {
            let node = user_node.get();
            let staker = StakersListAvia {
                user: user.clone(),
                staked,
            };
            self.stakers_list_avia().set_node_value_by_id(node, staker);
        }
    }

    fn get_staked_avia(&self, user: ManagedAddress) -> ManagedVec<u64> {
        let mut staked_aviators: ManagedVec<u64> = ManagedVec::new();
        for aviator in self.staked_aviators(&user).iter(){
            staked_aviators.push(aviator);
        }
        staked_aviators
    }

    #[view(AVIAStatistics)]
    fn avia_statistics(&self) -> AviatorStatistics<Self::Api> {
        let aviator_count = self.staked_aviators_count().get();
        let percentage_bonus = self.percentage_bonus().get();
        let total_percentage_bonus = aviator_count * percentage_bonus;
        let avia_token = self.avia_token().get();
        let produced_rewards = self.avia_produced_rewards().get();
        let min_stake_deposit = self.min_avia_deposit().get();
        let max_avia_staked = self.max_avia_staked().get();
        let avia_rewards = self.avia_supplied_rewards().get();
        let stats = AviatorStatistics {
            avia_token,
            percentage_bonus,
            total_percentage_bonus,
            aviator_count,
            produced_rewards,
            min_stake_deposit,
            max_avia_staked,
            avia_rewards
        };
        stats
    }
    #[view(UserAVIAStatistics)]
    fn user_avia_statistics(&self, user: ManagedAddress) -> UserAviatorStatistics<Self::Api> {
        let staked_aviators_count = self.staked_aviators(&user).len() as u64;
        let rewards = self.calculate_avia_reward(&user);
        let current_apr = staked_aviators_count  * self.percentage_bonus().get();
        let produced_rewards = self.avia_produced_rewards_user(&user).get();
        let staked_aviators = self.get_staked_avia(user);
        let stats = UserAviatorStatistics {
            staked_aviators_count,
            current_apr,
            produced_rewards,
            rewards,
            staked_aviators
        };
        stats
    }
    fn next_node_avia_staker(&self, user: &ManagedAddress) {
        let nodes_mapp = self.next_avia_node_staking();
        nodes_mapp.update(|next_node| *next_node += 1u32);
        let node = nodes_mapp.get();
        self.user_avia_node(user).set_if_empty(node);
    }

    }
    
