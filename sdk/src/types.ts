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
    marketSequence: number;
    poolSequence: number;
    tradeTokenSequence: number;
    minimumOrderMarginUsd: BN;
    maximumMaintenanceMarginRate: number;
    fundingFeeBaseRate: BN;
    maximumFundingBaseRate: BN;
    minimumPrecisionMultiple: BN;
    poolRewardsIntervalLimit: BN;
    initFee: BN;
    tradingFeeUsdPoolRewardsRatio: number;
    stakingFeeRewardRatio: number;
    poolFeeRewardRatio: number;
}


export type MarketPosition = {
    openInterest: BN;
    entryPrice: BN;
}

export type MarketConfig = {
    tickSize: BN;
    openFeeRate: BN;
    closeFeeRate: BN;
    maximumLongOpenInterestCap: BN;
    maximumShortOpenInterestCap: BN;
    longShortRatioLimit: BN;
    longShortOiBottomLimit: BN;
    maximumLeverage: number;
}


export type MarketFundingFee = {
    longFundingFeeAmountPerSize: BN;
    shortFundingFeeAmountPerSize: BN;
    totalLongFundingFee: BN;
    totalShortFundingFee: BN;
    longFundingFeeRate: BN;
    shortFundingFeeRate: BN;
    updatedAt: BN;
}

export type Market = {
    longOpenInterest: MarketPosition;
    shortOpenInterest: MarketPosition;
    fundingFee: MarketFundingFee;
    config: MarketConfig;
    poolKey: PublicKey;
    poolMintKey: PublicKey;
    indexMintKey: PublicKey;
    stablePoolKey: PublicKey;
    stablePoolMintKey: PublicKey;
    index: number;
    symbol: string;
}


export type PoolBalance = {
    amount: BN;
    holdAmount: BN;
    unSettleAmount: BN;
    settleFundingFeeAmount: BN;
    lossAmount: BN;
}


export type BorrowingFee = {
    totalBorrowingFee: BN;
    totalRealizedBorrowingFee: BN;
    cumulativeBorrowingFeePerToken: BN;
    updatedAt: BN;
}

export type FeeReward = {
    feeAmount: BN;
    unSettleFeeAmount: BN;
    cumulativeRewardsPerStakeToken: BN;
    lastRewardsPerStakeTokenDeltas: BN[];
}

export type PoolConfig = {
    minimumStakeAmount: BN;
    minimumUnStakeAmount: BN;
    poolLiquidityLimit: BN;
    borrowingInterestRate: BN;
    stakeFeeRate: number;
    unStakeFeeRate: number;
    unSettleMintRatioLimit: number;
}

export enum PoolStatus {
    NORMAL = 0,
    StakePaused = 1,
    UnStakePaused = 2
}

export type Pool = {
    name: number[];
    pnl: BN;
    apr: BN;
    insuranceFundAmount: BN;
    totalSupply: BN;
    settleFundingFee: BN;
    balance: PoolBalance;
    stableBalance: PoolBalance;
    borrowingFee: BorrowingFee;
    feeReward: FeeReward;
    stableFeeReward: FeeReward;
    config: PoolConfig;
    mintVaultKey: PublicKey;
    key: PublicKey;
    stableKey: PublicKey;
    mintKey: PublicKey;
    index: number;
    status: PoolStatus;
    stable: boolean;
}


export type TradeToken = {
    totalLiability: BN;
    totalAmount: BN;
    mintKey: PublicKey;
    oracleKey: PublicKey;
    vaultKey: PublicKey;
    name: number[];
    discount: number;
    liquidationFactor: number;
    index: number;
    decimals: number;
}


export enum UserStakeStatus {
    INIT = 0,
    USING = 1
}


export type UserRewards = {
    realisedRewardsTokenAmount: BN;
    openRewardsPerStakeToken: BN;
    tokenKey: PublicKey;
}


export type UserStake = {
    stakedShare: BN;
    userRewards: UserRewards;
    poolKey: PublicKey;
    userStakeStatus: UserStakeStatus;
}

export enum UserTokenStatus {
    INIT = 0,
    USING = 1
}


export type UserToken = {
    amount: BN;
    usedAmount: BN;
    liabilityAmount: BN;
    tokenMintKey: PublicKey;
    userTokenAccountKey: PublicKey;
    userTokenStatus: UserTokenStatus;
}

export enum PositionStatus {
    INIT = 0,
    USING = 1
}

export type UserPosition = {
    positionSize: BN;
    entryPrice: BN;
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
    realizedPnl: BN;
    userKey: PublicKey;
    marginMintKey: PublicKey;
    indexMintKey: PublicKey;
    positionKey: PublicKey;
    symbol: string;
    updatedAt: BN;
    leverage: number;
    isLong: boolean;
    isPortfolioMargin: boolean;
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
    orderMargin: BN;
    orderSize: BN;
    triggerPrice: BN;
    acceptablePrice: BN;
    createdAt: BN;
    orderId: BN;
    marginMintKey: PublicKey;
    authority: PublicKey;
    symbol: string;
    leverage: number;
    orderSide: OrderSide;
    positionSide: PositionSide;
    orderType: OrderType;
    stopType: StopType;
    status: OrderStatus;
    isPortfolioMargin: boolean;
    padding: number[];
}


export type UserAccount = {
    nextOrderId: BN;
    nextLiquidationId: BN;
    hold: BN;
    tokens: UserToken[];
    stakes: UserStake[];
    positions: UserPosition[];
    orders: UserOrder[];
    key: PublicKey;
    authority: PublicKey;
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