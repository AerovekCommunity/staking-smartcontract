// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           70
// Async Callback (empty):               1
// Total number of exported functions:  72

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    staking
    (
        getAPR
        getTotalStaked
        getStakedPosition
        RPSAcumulated
        getStorageRewards
        getAPRLastTime
        getMinimumStake
        getStakeDeposit
        getProducedRewards
        getProducedRewardsUser
        getStakersList
        getNextNodeStaking
        getUserNode
        getLockStakingState
        getLockTime
        getUnlockFutureTime
        getWithdrawFee
        getBurnOrCirculate
        getEarlyWithdrawingState
        getTokenID
        getStakingModuleState
        getRewards
        getBurnTokens
        getBurnTokensUser
        getActionRewards
        getAviaToken
        getPercentageBonus
        getStakersListAvia
        getAviaNextNodeStaking
        getUserAviaNode
        getStorageAviaRewards
        getAPRAviaLastTime
        getStakedAviators
        getStakedAviatorsCount
        getAviaProducedRewards
        getAviaProducedRewardsUser
        getMinAviaDeposit
        getMaxAviaStaked
        getAviaSuppliedRewards
        getAviaPower
        supplyRewards
        AviaRewards
        stakingState
        changeAPR
        StakeMinimum
        aviaPower
        LockStakeState
        EarlyWithdrawState
        changeLockTime
        changeWithdrawFee
        circulateOrBurn
        stakingSettings
        AviaStakingSettings
        depositAERO
        withdrawAERO
        claimAERO
        reinvestAERO
        AEROStatistics
        userStats
        setRewardsActions
        rewardAERO
        getDaoVoteWeight
        getDaoMembers
        hasAviaStaked
        depositAVIA
        withdrawAVIA
        claimAVIA
        reinvestAVIA
        AVIAStatistics
        UserAVIAStatistics
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
