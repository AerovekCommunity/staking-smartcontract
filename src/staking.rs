multiversx_sc::imports!();
use crate::err_and_const::{
    ERR_LOCK_TIME, 
    ERR_LOW_AMOUNT_DEPOSIT, 
    ERR_MODULE_DEACTIVATED, 
    ERR_WITHDRAW, 
    ERR_WRONG_TOKEN, 
    PERCENTAGE, 
    YEAR_IN_SECONDS,
    ERR_AVIA_STAKED
};
use crate::storage::{
    StakersList, 
    StakingStatistics, 
    UserStatistics
};

#[multiversx_sc::module]
pub trait Staking: crate::storage::StakingStorage + crate::shared_functions::SharedFunctions + crate::avia_staking::AVIAStaking{
    //DEPOSIT TOKENS
    #[payable("*")]
    #[endpoint(depositAERO)]
    fn deposit(&self) {
        let user = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        let (token_id, _nonce, amount) = self.call_value().single_esdt().into_tuple();
        require!(token_id == self.token_id().get(), ERR_WRONG_TOKEN);
        require!(amount >= self.minimum_stake().get(), ERR_LOW_AMOUNT_DEPOSIT);
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );

        if self.stake_deposit(&user).get() > BigUint::zero() {
            let state = self.action_rewards(&user).get();
            match state {
                //CLAIM
                true => {
                    self.avia_safe_reinvest(&user);
                    self.safe_rewards(&user);  
                }
                //REINVEST
                false => {
                    self.safe_reinvest_rewards(&user);
                }
            }
           
        }
        self.save_position(&user);
        self.total_staked().update(|value| *value += &amount);

        self.stake_deposit(&user).update(|value| *value += &amount);
        self.lock_tokens(&user, current_time);
        self.update_list(&user);
        
    }
    //WITHDRAW TOKENS
    #[endpoint(withdrawAERO)]
    fn withdraw(&self, amount: BigUint) {
        let current_time = self.blockchain().get_block_timestamp();
        let user = self.blockchain().get_caller();
        let deposit_amount = self.stake_deposit(&user).get();
        let rest_amount = &deposit_amount - &amount;
        let min_amount = self.min_avia_deposit().get() * self.staked_aviators(&user).len() as u64;
        require!(rest_amount >= min_amount , ERR_AVIA_STAKED);
        self.avia_safe(&user); 
        let token_id = self.token_id().get();
        require!(amount <= deposit_amount, ERR_WITHDRAW);
        let withdraw = self.user_withdraw_fee(&amount, &user);
        self.unlock_tokens(&user, current_time);
        self.safe_rewards(&user);
        self.send().direct_esdt(&user, &token_id, 0u64, &withdraw);
        self.stake_deposit(&user).update(|value| *value -= &amount);
        self.total_staked().update(|value| *value -= &amount);
        self.update_list(&user);
    }
    //CLAIM REWARD
    #[endpoint(claimAERO)]
    fn claim(&self) {
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );
        let current_time = self.blockchain().get_block_timestamp();
        let user = &self.blockchain().get_caller();
        self.lock_tokens(&user, current_time);
        self.avia_safe(&user);
        self.safe_rewards(&user);   
    }
    //REINVEST REWARD
    #[endpoint(reinvestAERO)]
    fn reinvest(&self) {
        let current_time = self.blockchain().get_block_timestamp();
        require!(
            self.staking_module_state().get() == true,
            ERR_MODULE_DEACTIVATED
        );
        let user = &self.blockchain().get_caller();
        self.avia_safe_reinvest(&user);
        self.safe_reinvest_rewards(&user);
        self.lock_tokens(&user, current_time);
        self.update_list(&user);   
    }

    //CALCULATION OF REWARD
    fn calculate_reward(&self, user: &ManagedAddress) -> BigUint {
        let my_stake = self.is_not_empty(self.stake_deposit(&user));
        let rps_position = self.is_not_empty(self.new_position(user));
        let rps_acumulated = self.rps_acumulated().get();
        let rps = rps_acumulated + self.rps_calculated();
        let current_rps = rps - rps_position;
        let result = current_rps * my_stake / YEAR_IN_SECONDS;
        let storage_rewards = self.storage_rewards(user).take();
        let rewards = result + storage_rewards;
        self.save_position(user);
        rewards
    }
   

    #[view(AEROStatistics)]
    fn staking_statistics(&self) -> StakingStatistics<Self::Api> {
        let total_staked = self.total_staked().get();
        let apr = self.apr().get();
        let burned_total = self.burned_tokens().get();
        let produced_rewards = self.produced_rewards().get();
        let available_rewards = self.rewards().get();
        let staking_stats = StakingStatistics {
            total_staked,
            apr,
            available_rewards,
            produced_rewards,
            burned_total,
        };
        staking_stats
    }

    #[view(userStats)]
    fn user_stats(&self, user: &ManagedAddress) -> UserStatistics<Self::Api> {
        let staked = self.stake_deposit(user).get();
        let rewards = self.calculate_reward(user);
        let burned = self.burned_tokens_user(user).get();
        let produced_rewards = self.produced_rewards_user(user).get();
        let user_stats = UserStatistics {
            staked,
            rewards,
            produced_rewards,
            burned,
        };
        user_stats
    }
    fn save_position(&self, user: &ManagedAddress) {
        let rps_acumulated = self.rps_acumulated().get();
        let rps = rps_acumulated + self.rps_calculated();
        self.new_position(user).set(rps);
    }
    //SAVING REWARD PER SECOND INTO ACUMULATOR
    fn rps(&self) {
        let current_time = self.blockchain().get_block_timestamp();
        let rps = self.rps_calculated();
        self.rps_acumulated().update(|amount| *amount += &rps);
        self.apr_last_time().set(current_time);
    }
    //CALCULATED REWARD PER SECOND WITH CURRENT APR
    fn rps_calculated(&self) -> BigUint {
        let current_time = self.blockchain().get_block_timestamp();
        self.apr_last_time().set_if_empty(current_time);
        let apr_last_time = self.apr_last_time().get();
        let current_apr = self.apr().get();
        let diff_time = (current_time + 1u64) - apr_last_time;
        let rps_calculated = BigUint::from(current_apr * diff_time / PERCENTAGE);
        rps_calculated
    }

    //ENSURES SAFETY OF REWARDS. IF THERE ARE NO REWARDS PROVIDED FOR CONTRACT, IT WILL SAVE THEM IN REWARD MAPPER
    fn safe_rewards(&self, user: &ManagedAddress) -> BigUint {
        let token_id = self.token_id().get();
        let rewards = self.calculate_reward(&user);
        let not_enough = self.not_enough(&rewards);
        if !not_enough {
            self.send().direct_esdt(user, &token_id, 0u64, &rewards);
            self.produced_rewards().update(|value| * value += &rewards);
            self.produced_rewards_user(user)
                .update(|value| *value += &rewards);
        } else {
            self.storage_rewards(user)
                .update(|amount| *amount += &rewards);
        }
        rewards
    }

    fn safe_reinvest_rewards(&self, user: &ManagedAddress) -> BigUint {
        let rewards = self.calculate_reward(&user);
        let not_enough = self.not_enough(&rewards);
        if !not_enough {
            self.total_staked().update(|value| *value += &rewards);
            self.stake_deposit(&user).update(|value| *value += &rewards);
            self.produced_rewards().update(|value| *value += &rewards);
            self.produced_rewards_user(user)
                .update(|value| *value += &rewards);
        } else {
            self.storage_rewards(&user)
                .update(|amount| *amount += &rewards);
        }
        rewards
    }

    fn unlock_tokens(&self, user: &ManagedAddress, current_time: u64) {
        if self.lock_staking_state().get() && !self.early_withdrawing_state().get() {
            require!(
                self.unlock_future_time(&user).get() < current_time,
                ERR_LOCK_TIME
            );
        }
    }

    fn update_list(&self, user: &ManagedAddress) {
        let staked = self.stake_deposit(&user).get();
        let user_node = self.user_node(user);
        if user_node.is_empty() {
            self.next_node_staker(user);
            let staker = StakersList {
                user: user.clone(),
                staked,
            };
            self.stakers_list().push_back(staker);
        } else {
            let node = user_node.get();
            let staker = StakersList {
                user: user.clone(),
                staked,
            };
            self.stakers_list().set_node_value_by_id(node, staker);
        }
    }

    fn next_node_staker(&self, user: &ManagedAddress) {
        let nodes_mapp = self.next_node_staking();
        nodes_mapp.update(|next_node| *next_node += 1u32);
        let node = nodes_mapp.get();
        self.user_node(user).set_if_empty(node);
    }

    fn user_withdraw_fee(&self, withdraw: &BigUint, user: &ManagedAddress) -> BigUint {
        if self.early_withdrawing_state().get() && self.lock_staking_state().get() {
            let fee = self.withdraw_fee().get();
            let token_id = self.token_id().get();
            let withdraw_left = withdraw * fee / PERCENTAGE;
            let withdraw_after_fee = withdraw - &withdraw_left;
            match self.burn_or_circulate().get() {
                true => {
                    self.send().esdt_local_burn(&token_id, 0u64, &withdraw_left);
                    self.burned_tokens()
                        .update(|value| *value += &withdraw_left);
                    self.burned_tokens_user(user)
                        .update(|value| *value += &withdraw_left);
                }
                false => {
                    self.rewards().update(|rewards| *rewards += &withdraw_left);
                }
            }
            withdraw_after_fee
        } else {
            withdraw.clone()
        }
    }

    

    #[endpoint(setRewardsActions)]
    fn rewards_action(&self, state: bool){
        let user = self.blockchain().get_caller();
        match state {
            //CLAIM
            true => {
                self.action_rewards(&user).set(true);
            }
            //REINVEST
            false => {
                self.action_rewards(&user).set(false);
            }
        }
    }

    #[view(rewardAERO)]
    fn get_all_rewards(&self, user: &ManagedAddress) -> BigUint {
        let avia_rewards = self.calculate_avia_reward(user);
        let aero_rewards = self.calculate_reward(user);
        let rewards = avia_rewards + aero_rewards;
        rewards
    }

    #[view(getDaoVoteWeight)]
    fn get_dao_vote_weight_view(
        &self,
        address: ManagedAddress,
        _token: OptionalValue<TokenIdentifier>,
    ) -> BigUint {
        let staked = self.stake_deposit(&address).get();
        let real_staked = staked;
        let avia_size = self.staked_aviators(&address).len() as u64;
        let avia_power =  BigUint::from(avia_size) * self.avia_power().get();
        let result= real_staked + avia_power;
        result
    }

    #[view(getDaoMembers)]
    fn get_dao_members_view(
        &self,
        _token: OptionalValue<TokenIdentifier>,
    ) -> MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>> {
        let mut members = MultiValueEncoded::new();
        for user in self.stakers_list().iter(){
            let user_address = user.clone().into_value().user;
            let staked = user.into_value().staked;
            if staked != BigUint::zero(){
            let real_staked = staked;
            let avia_size = self.staked_aviators(&user_address).len() as u64;
            let avia_power =  BigUint::from(avia_size) * self.avia_power().get();
            let result = real_staked + avia_power;
            members.push((user_address,result).into());
        }
        }
        members
    }

    #[view(hasAviaStaked)]
    fn has_avia_staked(&self, user: &ManagedAddress) -> bool{
        if self.staked_aviators(&user).is_empty() {
            return false
        }
        return true
    }

    fn avia_safe(&self, user: &ManagedAddress){
        if self.stake_deposit(&user).get() > BigUint::zero() && !self.staked_aviators(&user).is_empty() {
                    self.safe_avia_rewards(&user);
        }
    }

    fn avia_safe_reinvest(&self, user: &ManagedAddress){
        if self.stake_deposit(user).get() > BigUint::zero() && !self.staked_aviators(&user).is_empty() {
        self.safe_avia_reinvest_rewards(&user);  
        }
    }

}