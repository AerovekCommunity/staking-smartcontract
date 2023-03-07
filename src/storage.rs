multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct StakingStatistics<M: ManagedTypeApi> {
    pub total_staked: BigUint<M>,
    pub apr: u64,
    pub available_rewards: BigUint<M>,
    pub produced_rewards: BigUint<M>,
    pub burned_total: BigUint<M>,
}
#[derive(TypeAbi, TopEncode, TopDecode, NestedDecode, NestedEncode, Clone)]
pub struct StakersList<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub staked: BigUint<M>,
}
#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct UserStatistics<M: ManagedTypeApi> {
    pub staked: BigUint<M>,
    pub rewards: BigUint<M>,
    pub produced_rewards: BigUint<M>,
    pub burned: BigUint<M>,
}
#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Settings<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub apr: u64,
    pub early_withdrawing_state: bool,
    pub lock_time: u64,
    pub burn_or_circulate: bool,
    pub minimum: BigUint<M>,
    pub withdraw_fee: u64,
}
#[multiversx_sc::module]
pub trait StakingStorage {
    //CURRENT APR OF STAKING
    #[view(getAPR)]
    #[storage_mapper("APR")]
    fn apr(&self) -> SingleValueMapper<u64>;

    //TOTAL STAKED TOKENS FOR STAKING
    #[view(getTotalStaked)]
    #[storage_mapper("totalStaked")]
    fn total_staked(&self) -> SingleValueMapper<BigUint>;

    //SAVED POSITION FOR STAKER
    #[view(getStakedPosition)]
    #[storage_mapper("stakedPosition")]
    fn new_position(&self, staker: &ManagedAddress) -> SingleValueMapper<BigUint>;

    //ACUMULATED REWARD PER SECOND, COUNTS ONLY UP
    #[view(RPSAcumulated)]
    #[storage_mapper("RPSAcumulated")]
    fn rps_acumulated(&self) -> SingleValueMapper<BigUint>;

    //STORAGE REWARDS FOR STAKER
    #[view(getStorageRewards)]
    #[storage_mapper("storageRewards")]
    fn storage_rewards(&self, staker: &ManagedAddress) -> SingleValueMapper<BigUint>;

    //TIME WHEN APR WAS LAST CHANGED
    #[view(getAPRLastTime)]
    #[storage_mapper("APRLastTime")]
    fn apr_last_time(&self) -> SingleValueMapper<u64>;

    //MINIMUM STAKED TOKENS
    #[view(getMinimumStake)]
    #[storage_mapper("minimumStake")]
    fn minimum_stake(&self) -> SingleValueMapper<BigUint>;

    //DEPOSITED AMOUNTS OF TOKENS FOR SPECIFIC ADDRESS
    #[view(getStakeDeposit)]
    #[storage_mapper("stakeDeposit")]
    fn stake_deposit(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    //ALL PRODUCED REWARDS
    #[view(getProducedRewards)]
    #[storage_mapper("producedRewards")]
    fn produced_rewards(&self) -> SingleValueMapper<BigUint>;
    //USER PRODUCED REWARDS
    #[view(getProducedRewardsUser)]
    #[storage_mapper("producedRewardsUser")]
    fn produced_rewards_user(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    //LIST OF STAKERS< UPDATING WITH EVERY CHANGE
    #[view(getStakersList)]
    #[storage_mapper("StakersList")]
    fn stakers_list(&self) -> LinkedListMapper<StakersList<Self::Api>>;

    //COUNTING UP NODES FOR EACH STAKER
    #[view(getNextNodeStaking)]
    #[storage_mapper("nextNodeStaking")]
    fn next_node_staking(&self) -> SingleValueMapper<u32>;

    //SPECIFIC NODE OF USER
    #[view(getUserNode)]
    #[storage_mapper("userNode")]
    fn user_node(&self, user: &ManagedAddress) -> SingleValueMapper<u32>;

    //LOCK STATE OF STAKING
    #[view(getLockStakingState)]
    #[storage_mapper("LockStakingState")]
    fn lock_staking_state(&self) -> SingleValueMapper<bool>;

    //LOCK TIME OF STAKING WITHDRAW
    #[view(getLockTime)]
    #[storage_mapper("LockTime")]
    fn lock_time(&self) -> SingleValueMapper<u64>;

    //FUTURE UNLOCK TIME FOR SPECIFIC ADDRESS
    #[view(getUnlockFutureTime)]
    #[storage_mapper("UnlockFutureTime")]
    fn unlock_future_time(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    //FEE FOR EARLY WITHDRAW WHILE IN LOCK STATE
    #[view(getWithdrawFee)]
    #[storage_mapper("withdrawFee")]
    fn withdraw_fee(&self) -> SingleValueMapper<u64>;

    //DECISION IF EARLY WITHDRAW TOKENS GET BURN OR GET BACK TO POOL
    #[view(getBurnOrCirculate)]
    #[storage_mapper("burnOrCirculate")]
    fn burn_or_circulate(&self) -> SingleValueMapper<bool>;

    //STATE OF EARLY WITHDRAW
    #[view(getEarlyWithdrawingState)]
    #[storage_mapper("EarlyWithdrawingState")]
    fn early_withdrawing_state(&self) -> SingleValueMapper<bool>;

    #[view(getTokenID)]
    #[storage_mapper("tokenID")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    //STAKING MODULE STATE
    #[view(getStakingModuleState)]
    #[storage_mapper("stakingModuleState")]
    fn staking_module_state(&self) -> SingleValueMapper<bool>;

    #[view(getRewards)]
    #[storage_mapper("rewards")]
    fn rewards(&self) -> SingleValueMapper<BigUint>;

    //OVERALL BURNED TOKENS
    #[view(getBurnTokens)]
    #[storage_mapper("BurnedTokens")]
    fn burned_tokens(&self) -> SingleValueMapper<BigUint>;

    //USER BURNED TOKENS
    #[view(getBurnTokensUser)]
    #[storage_mapper("BurnedTokensUser")]
    fn burned_tokens_user(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
