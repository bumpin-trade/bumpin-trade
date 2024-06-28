import {PublicKey} from "@solana/web3.js";
// import BN from "bn.js";
import {BN} from "@coral-xyz/anchor";


export class OracleSource {
    static readonly PYTH = {pyth: {}};
    static readonly PYTH_1K = {pyth1K: {}};
    static readonly PYTH_1M = {pyth1M: {}};
    static readonly SWITCHBOARD = {switchboard: {}};
    static readonly QUOTE_ASSET = {quoteAsset: {}};
    static readonly PYTH_STABLE_COIN = {pythStableCoin: {}};
    static readonly Prelaunch = {prelaunch: {}};
}

export type State = {
    admin: PublicKey;
    bumpSigner: PublicKey;
    keeperSigner: PublicKey;
    bumpSignerNonce: number;
    numberOfMarkets: number;
    numberOfPools: number;
    numberOfTradeTokens: number;
    minOrderMarginUsd: BN;
    maxMaintenanceMarginRate: BN;
    fundingFeeBaseRate: BN;
    maxFundingBaseRate: BN;
    minPrecisionMultiple: BN;
    poolRewardsIntervalLimit: BN;
    initFee: BN;
    tradingFeeUsdPoolRewardsRatio: BN;
    stakingFeeRewardRatio: BN;
    poolFeeRewardRatio: BN;
}


export type MarketPosition = {
    openInterest: BN;
    entryPrice: BN;
}

export type MarketConfig = {
    maxLeverage: BN;
    tickSize: BN;
    openFeeRate: BN;
    closeFeeRate: BN;
    maxLongOpenInterestCap: BN;
    maxShortOpenInterestCap: BN;
    longShortRatioLimit: BN;
    longShortOiBottomLimit: BN;
}

export type MarketFundingFee = {
    lastUpdateTime: BN;
    fundingFeeRate: BN;
    fundingFee: BN;
}

export type Market = {
    symbol: string;
    marketIndex: number;
    poolKey: PublicKey;
    poolMint: PublicKey;
    indexMint: PublicKey;
    stablePoolKey: PublicKey;
    stablePoolMint: PublicKey;
    longOpenInterest: MarketPosition;
    shortOpenInterest: MarketPosition;
    fundingFee: MarketFundingFee;
    marketTradeConfig: MarketConfig;
}

export type PoolBalance = {
    poolMint: PublicKey;
    amount: BN;
    holdAmount: BN;
    unSettleAmount: BN;
    lossAmount: BN;
}

export type BorrowingFee = {
    poolMint: PublicKey;
    feeRate: BN;
    lastUpdateTime: BN;
}

export type FeeReward = {
    poolMint: PublicKey;
    reward: BN;
    lastUpdateTime: BN;
}

export type PoolConfig = {
    miniStakeAmount: BN;
    miniUnStakeAmount: BN;
    poolLiquidityLimit: BN;
    stakeFeeRate: BN;
    unStakeFeeRate: BN;
    unSettleMintRatioLimit: BN;
    borrowingInterestRate: BN;
}

export enum PoolStatus {
    NORMAL = 0,
    StakePaused = 1,
    UnStakePaused = 2
}

export type Pool = {
    poolKey: PublicKey;
    poolMint: PublicKey;
    poolIndex: number;
    poolMintVault: PublicKey;
    poolName: string;
    poolBalance: PoolBalance;
    stableBalance: PoolBalance;
    borrowingFee: BorrowingFee;
    feeReward: FeeReward;
    stableFeeReward: FeeReward;
    poolConfig: PoolConfig;
    totalSupply: BN;
    poolStatus: PoolStatus;
    stable: boolean;
    pnl: BN;
    apr: BN;
    insuranceFundAmount: BN;
}

export type TradeToken = {
    mint: PublicKey;
    mintName: number[];
    oracle: PublicKey;
    tokenIndex: number;
    discount: BN;
    liquidationFactor: BN;
    decimals: number;
    totalLiability: BN;
    totalAmount: BN;
    tradeTokenVault: PublicKey;
}


export enum UserStakeStatus {
    INIT = 0,
    USING = 1
}

export type UserRewards = {
    token: PublicKey;
    realisedRewardsTokenAmount: BN;
    openRewardsPerStakeToken: BN;
}

export type UserStake = {
    stakedShare: BN;
    userStakeStatus: UserStakeStatus;
    poolKey: PublicKey;
    userRewards: UserRewards;
}

export enum UserTokenStatus {
    INIT = 0,
    USING = 1
}

export type UserToken = {
    userTokenStatus: UserTokenStatus;
    tokenMint: PublicKey;
    userTokenAccountKey: PublicKey;
    amount: BN;
    usedAmount: BN;
    liability: BN;
}

export enum PositionStatus {
    INIT = 0,
    USING = 1
}

export type UserPosition = {
    positionKey: PublicKey;
    symbol: string;
    isLong: boolean;
    crossMargin: boolean;
    authority: PublicKey;
    marginMint: PublicKey;
    indexMint: PublicKey;
    positionSize: BN;
    entryPrice: BN;
    leverage: BN;
    initialMargin: BN;
    initialMarginUsd: BN;
    initialMarginUsdFromPortfolio: BN;
    mmUsd: BN;
    holdPoolAmount: BN;
    openFee: BN;
    openFeeInUsd: BN;
    realizedBorrowingFee: BN;
    realizedBorrowingFeeInUsd: BN;
    openBorrowingFeePerToken: BN;
    realizedFundingFee: BN;
    realizedFundingFeeInUsd: BN;
    openFundingFeeAmountPerSize: BN;
    closeFeeInUsd: BN;
    lastUpdateTime: BN;
    realizedPnl: BN;
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
    userKey: PublicKey;
    authority: PublicKey;
    nextOrderId: BN;
    nextLiquidationId: BN;
    hold: BN;
    userTokens: UserToken[];
    userStakes: UserStake[];
    userPositions: UserPosition[];
    userOrders: UserOrder[];
}

export type TradeTokenBalance = {
    tokenNetValue: BN;
    tokenUsedValue: BN;
    tokenBorrowingValue: BN;
}

export type PositionBalance = {
    initialMarginUsdFromPortfolio: BN;
    positionUnPnl: BN;
    mmUsd: BN;
}