import {PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";

export class OracleSource {
    static readonly PYTH = { pyth: {} };
    static readonly PYTH_1K = { pyth1K: {} };
    static readonly PYTH_1M = { pyth1M: {} };
    static readonly SWITCHBOARD = { switchboard: {} };
    static readonly QUOTE_ASSET = { quoteAsset: {} };
    static readonly PYTH_STABLE_COIN = { pythStableCoin: {} };
    static readonly Prelaunch = { prelaunch: {} };
}

export type State = {
    admin: PublicKey;
    bump_signer: PublicKey;
    keeper_signer: PublicKey;
    bump_signer_nonce: number;
    number_of_markets: number;
    number_of_pools: number;
    number_of_trade_tokens: number;
    min_order_margin_usd: BN;
    max_maintenance_margin_rate: BN;
    funding_fee_base_rate: BN;
    max_funding_base_rate: BN;
    min_precision_multiple: BN;
    pool_rewards_interval_limit: BN;
    init_fee: BN;
    trading_fee_usd_pool_rewards_ratio: BN;
    staking_fee_reward_ratio: BN;
    pool_fee_reward_ratio: BN;
}


export type MarketPosition = {
    open_interest: BN;
    entry_price: BN;
}

export type MarketConfig = {
    max_leverage: BN;
    tick_size: BN;
    open_fee_rate: BN;
    close_fee_rate: BN;
    max_long_open_interest_cap: BN;
    max_short_open_interest_cap: BN;
    long_short_ratio_limit: BN;
    long_short_oi_bottom_limit: BN;
}

export type MarketFundingFee = {
    last_update_time: BN;
    funding_fee_rate: BN;
    funding_fee: BN;
}

export type Market = {
    symbol: string;
    market_index: number;
    pool_key: PublicKey;
    pool_mint: PublicKey;
    index_mint: PublicKey;
    stable_pool_key: PublicKey;
    stable_pool_mint: PublicKey;
    long_open_interest: MarketPosition;
    short_open_interest: MarketPosition;
    funding_fee: MarketFundingFee;
    market_trade_config: MarketConfig;
}


export type PoolBalance = {
    pool_mint: PublicKey;
    amount: BN;
    hold_amount: BN;
    un_settle_amount: BN;
    loss_amount: BN;
}

export type BorrowingFee = {
    pool_mint: PublicKey;
    fee_rate: BN;
    last_update_time: BN;
}

export type FeeReward = {
    pool_mint: PublicKey;
    reward: BN;
    last_update_time: BN;
}

export type PoolConfig = {
    mini_stake_amount: BN;
    mini_un_stake_amount: BN;
    pool_liquidity_limit: BN;
    stake_fee_rate: BN;
    un_stake_fee_rate: BN;
    un_settle_mint_ratio_limit: BN;
    borrowing_interest_rate: BN;
}

export enum PoolStatus {
    NORMAL = 0,
    StakePaused = 1,
    UnStakePaused = 2
}

export type Pool = {
    pool_key: PublicKey;
    pool_mint: PublicKey;
    pool_index: number;
    pool_mint_vault: PublicKey;
    pool_name: string;
    pool_balance: PoolBalance;
    stable_balance: PoolBalance;
    borrowing_fee: BorrowingFee;
    fee_reward: FeeReward;
    stable_fee_reward: FeeReward;
    pool_config: PoolConfig;
    total_supply: BN;
    pool_status: PoolStatus;
    stable: boolean;
    pnl: BN;
    apr: BN;
    insurance_fund_amount: BN;
}


export type TradeToken = {
    mint: PublicKey;
    mintName: string;
    oracle: PublicKey;
    token_index: number;
    discount: BN;
    liquidation_factor: BN;
    decimals: number;
    total_liability: BN;
    total_amount: BN;
    trade_token_vault: PublicKey;

}

export enum UserStakeStatus {
    INIT = 0,
    USING = 1

}

export type UserRewards = {
    token: PublicKey;
    realised_rewards_token_amount: BN;
    open_rewards_per_stake_token: BN;
}

export type UserStake = {
    user_stake_status: UserStakeStatus;
    pool_key: PublicKey;
    amount: BN;
    user_rewards: UserRewards;
}


export enum UserTokenStatus {
    INIT = 0,
    USING = 1
}


export type UserToken = {
    user_token_status: UserTokenStatus;
    token_mint: PublicKey;
    user_token_account_key: PublicKey;
    amount: BN;
    used_amount: BN;
    liability: BN;
}

export enum PositionStatus {
    INIT = 0,
    USING = 1
}

export type UserPosition = {
    position_key: PublicKey;
    symbol: string;
    is_long: boolean;
    cross_margin: boolean;
    authority: PublicKey;
    margin_mint: PublicKey;
    index_mint: PublicKey;
    position_size: BN;
    entry_price: BN;
    leverage: BN;
    initial_margin: BN;
    initial_margin_usd: BN;
    initial_margin_usd_from_portfolio: BN;
    mm_usd: BN;
    hold_pool_amount: BN;
    open_fee: BN;
    open_fee_in_usd: BN;
    realized_borrowing_fee: BN;
    realized_borrowing_fee_in_usd: BN;
    open_borrowing_fee_per_token: BN;
    realized_funding_fee: BN;
    realized_funding_fee_in_usd: BN;
    open_funding_fee_amount_per_size: BN;
    close_fee_in_usd: BN;
    last_update_time: BN;
    realized_pnl: BN;
    status: PositionStatus;

}

export enum OrderSide {
    NONE = 0,
    LONG = 1,
    SHORT = 2,
}

export enum OrderStatus {
    INIT = 0,
    USING = 1,
}

export enum PositionSide {
    NONE = 0,
    INCREASE = 1,
    DECREASE = 2,
}

export enum OrderType {
    NONE = 0,
    MARKET = 1,
    LIMIT = 2,
    STOP = 3,
}

export enum StopType {
    NONE = 0,
    StopLoss = 1,
    TakeProfit = 2,
}

export type UserOrder = {
    authority: PublicKey;
    order_id: BN;
    symbol: string;
    order_side: OrderSide;
    position_side: PositionSide;
    order_type: OrderType;
    stop_type: StopType;
    cross_margin: boolean;
    margin_mint: PublicKey;
    order_margin: BN;
    leverage: BN;
    order_size: BN;
    trigger_price: BN;
    acceptable_price: BN;
    time: BN;
    status: OrderStatus;
}

export type UserAccount = {
    user_key: PublicKey;
    authority: PublicKey;
    next_order_id: BN;
    next_liquidation_id: BN;
    hold: BN;
    user_tokens: Array<UserToken>;
    user_stakes: Array<UserStake>;
    user_positions: Array<UserPosition>;
    user_orders: Array<UserOrder>;
}