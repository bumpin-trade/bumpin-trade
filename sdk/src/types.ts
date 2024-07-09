import {PublicKey} from "@solana/web3.js";
// import BN from "bn.js";
import {BN} from "@coral-xyz/anchor";
import {OraclePriceData} from "./oracles/types";
import {PriceData} from "@pythnetwork/client";


export class OracleSource {
    static readonly PYTH = {pyth: {}};
    static readonly PYTH_1K = {pyth1K: {}};
    static readonly PYTH_1M = {pyth1M: {}};
    static readonly SWITCHBOARD = {switchboard: {}};
    static readonly QUOTE_ASSET = {quoteAsset: {}};
    static readonly PYTH_STABLE_COIN = {pythStableCoin: {}};
    static readonly Prelaunch = {prelaunch: {}};
}


export type InitializeMarketParams = {
    symbol: number[];
    tickSize: BN;
    openFeeRate: BN;
    closeFeeRate: BN;
    maximumLongOpenInterestCap: BN;
    maximumShortOpenInterestCap: BN;
    longShortRatioLimit: BN;
    longShortOiBottomLimit: BN;
    maximumLeverage: number;
    minimumLeverage: number;
    poolIndex: number;
    stablePoolIndex: number;
}

export type InitializeStateParams = {
    minOrderMarginUsd: BN;
    maximumMaintenanceMarginRate: number;
    fundingFeeBaseRate: BN;
    maxFundingBaseRate: BN;
    tradingFeeStakingRewardsRatio: BN;
    tradingFeePoolRewardsRatio: BN;
    tradingFeeUsdPoolRewardsRatio: BN;
    borrowingFeeStakingRewardsRatio: BN;
    borrowingFeePoolRewardsRatio: BN;
    minPrecisionMultiple: BN;
    mintFeeStakingRewardsRatio: BN;
    mintFeePoolRewardsRatio: BN;
    redeemFeeStakingRewardsRatio: BN;
    redeemFeePoolRewardsRatio: BN;
    poolRewardsIntervalLimit: BN;
    initFee: BN;
    stakingFeeRewardRatio: number;
    poolFeeRewardRatio: number;
}


export type InitializePoolParams = {
    name: number[];
    stableMintKey: number[];
    poolConfig: PoolConfig;
    stable: boolean;
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
    initFee: number;
    tradingFeeUsdPoolRewardsRatio: number;
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
    minimumLeverage: number;
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
    symbol: number[];
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
    padding: number[];
}

export class PoolStatus {
    static readonly NORMAL = {init: {}};
    static readonly StakePaused = {stakePaused: {}};
    static readonly UnStakePaused = {unStakePaused: {}};
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
    stableMintKey: PublicKey;
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


export class UserStakeStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}


export type UserRewards = {
    total_claim_rewards_amount: BN;
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

export class UserTokenStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}


export type UserToken = {
    amount: BN;
    usedAmount: BN;
    liabilityAmount: BN;
    tokenMintKey: PublicKey;
    userTokenAccountKey: PublicKey;
    userTokenStatus: UserTokenStatus;
}

export class PositionStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
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
    symbol: number[];
    updatedAt: BN;
    leverage: number;
    isLong: boolean;
    isPortfolioMargin: boolean;
    status: PositionStatus;

}

export class OrderSide {
    static readonly NONE = {none: {}};
    static readonly LONG = {long: {}};
    static readonly SHORT = {short: {}};
}

export class OrderStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}

export class PositionSide {
    static readonly NONE = {none: {}};
    static readonly INCREASE = {increase: {}};
    static readonly DECREASE = {decrease: {}};
}

export class OrderType {
    static readonly NONE = {none: {}};
    static readonly MARKET = {market: {}};
    static readonly LIMIT = {limit: {}};
    static readonly STOP = {stop: {}};
}

export class StopType {
    static readonly NONE = {none: {}};
    static readonly StopLoss = {stopLoss: {}};
    static readonly TakeProfit = {takeProfit: {}};
}

export type OrderSideValue =
    | typeof OrderSide.NONE
    | typeof OrderSide.LONG
    | typeof OrderSide.SHORT;

export type OrderStatusValue =
    | typeof OrderStatus.INIT
    | typeof OrderStatus.USING;

export type PositionSideValue =
    | typeof PositionSide.NONE
    | typeof PositionSide.INCREASE
    | typeof PositionSide.DECREASE;

export type OrderTypeValue =
    | typeof OrderType.NONE
    | typeof OrderType.MARKET
    | typeof OrderType.LIMIT
    | typeof OrderType.STOP;

export type StopTypeValue =
    | typeof StopType.NONE
    | typeof StopType.StopLoss
    | typeof StopType.TakeProfit;


export type UserOrder = {
    orderMargin: BN;
    orderSize: BN;
    triggerPrice: BN;
    acceptablePrice: BN;
    createdAt: BN;
    orderId: BN;
    marginMintKey: PublicKey;
    authority: PublicKey;
    symbol: number[];
    leverage: number;
    orderSide: OrderSide;
    positionSide: PositionSide;
    orderType: OrderType;
    stopType: StopType;
    status: OrderStatus;
    isPortfolioMargin: boolean;
}

export class UserStatus {
    static readonly NORMAL = {normal: {}};
    static readonly LIQUIDATION = {liquidation: {}};
    static readonly DISABLE = {disable: {}};
}

export type UserStatusValue =
    | typeof UserStatus.NORMAL
    | typeof UserStatus.LIQUIDATION
    | typeof UserStatus.DISABLE;

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
    status: UserStatusValue;
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

export type PlaceOrderParams = {
    isPortfolioMargin: boolean;
    isNativeToken: boolean;
    orderSide: OrderSideValue;
    positionSide: PositionSideValue;
    orderType: OrderTypeValue;
    stopType: StopTypeValue;
    size: BN;
    orderMargin: BN;
    leverage: number;
    triggerPrice: BN;
    acceptablePrice: BN;
}



export type InnerPlaceOrderParams = {
    symbol: number[];
    isPortfolioMargin: boolean;
    isNativeToken: boolean;
    orderSide: OrderSideValue;
    positionSide: PositionSideValue;
    orderType: OrderTypeValue;
    stopType: StopTypeValue;
    size: BN;
    orderMargin: BN;
    leverage: number;
    triggerPrice: BN;
    acceptablePrice: BN;
    placeTime: BN;
    poolIndex: number;
    stablePoolIndex: number;
    marketIndex: number;
    tradeTokenIndex: number;
    indexTradeTokenIndex: number;
    orderId: BN,
}

export type MarketWithIndexTradeTokenPrices = {
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
    symbol: number[];
    indexTradeTokenPrices: PriceData[];
}


export type PoolSummary = {
    pool: Pool;
    categoryTags: string[];
    markets: MarketWithIndexTradeTokenPrices[];
}


export  type UserClaimResult = {
    total: BN;
    claimed: BN;
    unClaim: BN;
    rewards: Array<UserClaimRewardsResult>;
}
export type UserClaimRewardsResult = {
    pool: number[];
    rewardsAmount: BN;
}