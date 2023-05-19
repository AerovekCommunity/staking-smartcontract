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
#[derive(TypeAbi, TopEncode, TopDecode, NestedDecode, NestedEncode, Clone)]
pub struct StakersListAvia<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub staked: usize,
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

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct AVIASettings<M: ManagedTypeApi> {
    pub avia_id: TokenIdentifier<M>,
    pub bonus_apr: u64,
    pub min_stake_deposit: BigUint<M>,
    pub max_avia_staked: u64
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct AviatorStatistics<M: ManagedTypeApi> {
    pub avia_token: TokenIdentifier<M>,
    pub percentage_bonus: u64,
    pub total_percentage_bonus: u64,
    pub aviator_count: u64,
    pub produced_rewards: BigUint<M>,
    pub min_stake_deposit: BigUint<M>,
    pub max_avia_staked: u64,
    pub avia_rewards: BigUint<M>
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct UserAviatorStatistics<M: ManagedTypeApi> {
    pub staked_aviators_count: u64,
    pub current_apr: u64,
    pub produced_rewards: BigUint<M>,
    pub rewards: BigUint<M>,
    pub staked_aviators:ManagedVec<M,u64>
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

    //AVAILABLE REWARDS FOR STAKERS
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

    //IT WILL SET ACTION WHAT REWARDS SHOULD DO UPON STAKING TOKENS (CLAIM OR REINVEST THEM - REINVEST AS DEFAULT)
    #[view(getActionRewards)]
    #[storage_mapper("ActionRewards")]
    fn action_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<bool>;




//AVIATORS STAKING
    //AVIATORS TOKEN ID
    #[view(getAviaToken)]
    #[storage_mapper("aviaToken")]
    fn avia_token(&self) -> SingleValueMapper<TokenIdentifier>;

    //BONUS FOR AVIATOR STAKING
    #[view(getPercentageBonus)]
    #[storage_mapper("PercentageBonus")]
    fn percentage_bonus(&self) -> SingleValueMapper<u64>;

    //LIST OF AVIATOR STAKERS UPDATING WITH EVERY CHANGE
    #[view(getStakersListAvia)]
    #[storage_mapper("StakersListAvia")]
    fn stakers_list_avia(&self) -> LinkedListMapper<StakersListAvia<Self::Api>>;

    //COUNTING UP NODES FOR EACH AVIATOR STAKER
    #[view(getAviaNextNodeStaking)]
    #[storage_mapper("nextAviaNodeStaking")]
    fn next_avia_node_staking(&self) -> SingleValueMapper<u32>;

    //SPECIFIC NODE OF AVIATOR USER
    #[view(getUserAviaNode)]
    #[storage_mapper("userAviaNode")]
    fn user_avia_node(&self, user: &ManagedAddress) -> SingleValueMapper<u32>;
    
    //STORAGE REWARDS FOR AVIATOR STAKER
    #[view(getStorageAviaRewards)]
    #[storage_mapper("storageAviaRewards")]
    fn storage_avia_rewards(&self, staker: &ManagedAddress) -> SingleValueMapper<BigUint>;
    
    //TIME WHEN BONUS APR WAS LAST CHANGED
    #[view(getAPRAviaLastTime)]
    #[storage_mapper("APRAviaLastTime")]
    fn apr_avia_last_time(&self, staker: &ManagedAddress) -> SingleValueMapper<u64>;
    
    //STAKED AVIATORS FOR USER
    #[view(getStakedAviators)]
    #[storage_mapper("StakedAviators")]
    fn staked_aviators(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    //STAKED AVIATORS ALL COUNT
    #[view(getStakedAviatorsCount)]
    #[storage_mapper("stakedAviatorsCount")]
    fn staked_aviators_count(&self) -> SingleValueMapper<u64>;

    //AVIATOR ALL PRODUCED REWARDS
    #[view(getAviaProducedRewards)]
    #[storage_mapper("aviaProducedRewards")]
    fn avia_produced_rewards(&self) -> SingleValueMapper<BigUint>;

    //USER AVIATOR ALL PRODUCED REWARDS
    #[view(getAviaProducedRewardsUser)]
    #[storage_mapper("aviaProducedRewardsUser")]
    fn avia_produced_rewards_user(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getMinAviaDeposit)]
    #[storage_mapper("minAviaDeposit")]
    fn min_avia_deposit(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxAviaStaked)]
    #[storage_mapper("maxAviaStaked")]
    fn max_avia_staked(&self) -> SingleValueMapper<u64>;

    #[view(getAviaSuppliedRewards)]
    #[storage_mapper("aviaSuppliedRewards")]
    fn avia_supplied_rewards(&self) -> SingleValueMapper<BigUint>;

    #[view(getAviaPower)]
    #[storage_mapper("aviaPower")]
    fn avia_power(&self) -> SingleValueMapper<BigUint>;
}
