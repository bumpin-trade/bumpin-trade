import {PublicKey} from "@solana/web3.js";
// import BN from "bn.js";
import {BN} from "@coral-xyz/anchor";
import {PriceData} from "@pythnetwork/client";
import {BumpinUtils} from "./utils/utils";
import {BumpinInvalidParameter, BumpinTokenNotFound} from "./errors";
import BigNumber from "bignumber.js";
import {isEqual} from "lodash";

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
};

export type InitializeStateParams = {
    keeperKey: number[];
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
};

export type InitializePoolParams = {
    name: number[];
    stableMintKey: number[];
    poolConfig: PoolConfig;
    stable: boolean;
};

export type StateAccount = {
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
};

export type MarketPosition = {
    openInterest: BN;
    entryPrice: BN;
};

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
};

export type MarketFundingFee = {
    longFundingFeeAmountPerSize: BN;
    shortFundingFeeAmountPerSize: BN;
    totalLongFundingFee: BN;
    totalShortFundingFee: BN;
    longFundingFeeRate: BN;
    shortFundingFeeRate: BN;
    updatedAt: BN;
};

export type Market = {
    longOpenInterest: MarketPosition;
    shortOpenInterest: MarketPosition;
    fundingFee: MarketFundingFee;
    config: MarketConfig;
    poolKey: PublicKey;
    poolMintKey: PublicKey;
    indexMintOracle: PublicKey;
    stablePoolKey: PublicKey;
    stablePoolMintKey: PublicKey;
    index: number;
    symbol: number[];
};

export type PoolBalance = {
    settleFundingFee: BN;
    amount: BN;
    holdAmount: BN;
    unSettleAmount: BN;
    settleFundingFeeAmount: BN;
    lossAmount: BN;
};

export type BorrowingFee = {
    totalBorrowingFee: BN;
    totalRealizedBorrowingFee: BN;
    cumulativeBorrowingFeePerToken: BN;
    updatedAt: BN;
};

export type FeeReward = {
    feeAmount: BN;
    unSettleFeeAmount: BN;
    cumulativeRewardsPerStakeToken: BN;
    lastRewardsPerStakeTokenDeltas: BN[];
};

export type PoolConfig = {
    minimumStakeAmount: BN;
    minimumUnStakeAmount: BN;
    poolLiquidityLimit: BN;
    borrowingInterestRate: BN;
    stakeFeeRate: number;
    unStakeFeeRate: number;
    unSettleMintRatioLimit: number;
    padding: number[];
};

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
    balance: PoolBalance;
    stableBalance: PoolBalance;
    borrowingFee: BorrowingFee;
    feeReward: FeeReward;
    stableFeeReward: FeeReward;
    config: PoolConfig;
    poolVaultKey: PublicKey;
    key: PublicKey;
    stableMintKey: PublicKey;
    mintKey: PublicKey;
    index: number;
    status: PoolStatus;
    stable: boolean;
};

export type Rewards = {
    poolUnClaimAmount: BN;
    poolTotalRewardsAmount: BN;
    poolRewardsVault: PublicKey;
    daoRewardsVault: PublicKey;
    daoTotalRewardsAmount: BN;
    poolIndex: number;
};

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
};

export class UserStakeStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}

export type UserRewards = {
    totalClaimRewardsAmount: BN;
    realisedRewardsTokenAmount: BN;
    openRewardsPerStakeToken: BN;
    tokenKey: PublicKey;
};

export type UserStake = {
    stakedShare: BN;
    userRewards: UserRewards;
    poolKey: PublicKey;
    userStakeStatus: UserStakeStatus;
};

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
};

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
    indexMintOracle: PublicKey;
    positionKey: PublicKey;
    symbol: number[];
    updatedAt: BN;
    leverage: number;
    isLong: boolean;
    isPortfolioMargin: boolean;
    status: PositionStatus;
};

export class OrderSide {
    static readonly NONE = {none: {}};
    static readonly LONG = {long: {}};
    static readonly SHORT = {short: {}};

    toString() {
        return isEqual(this, OrderSide.NONE) ? "None" : isEqual(this, OrderSide.LONG) ? "Long" : "Short";
    }
}

export class OrderStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}

export class PositionSide {
    static readonly NONE = {none: {}};
    static readonly INCREASE = {increase: {}};
    static readonly DECREASE = {decrease: {}};

    toString() {
        return isEqual(this, PositionSide.NONE) ? "None" : isEqual(this, PositionSide.INCREASE) ? "Increase" : "Decrease";
    }
}

export class OrderType {
    static readonly NONE = {none: {}};
    static readonly MARKET = {market: {}};
    static readonly LIMIT = {limit: {}};
    static readonly STOP = {stop: {}};

    toString() {
        return isEqual(this, OrderType.NONE) ? "None" : isEqual(this, OrderType.MARKET) ? "Market" : isEqual(this, OrderType.LIMIT) ? "Limit" : "Stop";
    }
}

export class StopType {
    static readonly NONE = {none: {}};
    static readonly StopLoss = {stopLoss: {}};
    static readonly TakeProfit = {takeProfit: {}};

    toString() {
        return isEqual(this, StopType.NONE) ? "None" : isEqual(this, StopType.StopLoss) ? "StopLoss" : "TakeProfit";
    }
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
};

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
    createdAt: BN;
    status: UserStatusValue;
};

export type TradeTokenBalance = {
    tokenNetValue: BN;
    tokenUsedValue: BN;
    tokenBorrowingValue: BN;
};
export type AccountNetValue = {
    accountNetValue: BN;
    totalMM: BN;
}
export type PositionBalance = {
    // total_im_usd
    initialMarginUsd: BN;
    // total_im_usd_from_portfolio
    initialMarginUsdFromPortfolio: BN;
    // total_un_pnl_usd
    positionUnPnl: BN;
    // total_position_mm
    mmUsd: BN;
    positionFee: BN;
};

export type PositionFee = {
    fundingFee: BN;
    fundingFeeUsd: BN;
    borrowingFee: BN;
    borrowingFeeUsd: BN;
    closeFeeUsd: BN;
    totalUsd: BN;
};

export type PlaceOrderParams = {
    isPortfolioMargin: boolean;
    isNativeToken: boolean;
    orderSide: OrderSideValue;
    positionSide: PositionSideValue;
    orderType: OrderTypeValue;
    stopType: StopTypeValue;
    size: number;
    orderMargin: number;
    leverage: number;
    triggerPrice: number;
};

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
    stableTradeTokenIndex: number;
    orderId: BN;
};

export type MarketWithIndexTradeTokenPrices = {
    longOpenInterest: MarketPosition;
    shortOpenInterest: MarketPosition;
    fundingFee: MarketFundingFee;
    config: MarketConfig;
    poolKey: PublicKey;
    poolMintKey: PublicKey;
    indexMintOracle: PublicKey;
    stablePoolKey: PublicKey;
    stablePoolMintKey: PublicKey;
    index: number;
    symbol: number[];
    indexTradeTokenPrices: PriceData[];
};

export type PoolSummary = {
    pool: Pool;
    netPrice: BigNumber;
    categoryTags: string[];
    markets: MarketWithIndexTradeTokenPrices[];
};

export type UserClaimResult = {
    total: BN;
    claimed: BN;
    unClaim: BN;
    rewards: Array<UserClaimRewardsResult>;
};
export type UserClaimRewardsResult = {
    pool: number[];
    rewardsAmount: BN;
};

export class TokenBalance {
    tradeToken: TradeToken;
    amount: BN;
    tradeTokenPriceData: PriceData;

    constructor(
        tradeToken: TradeToken,
        amount: bigint,
        tradeTokenPriceData: PriceData
    ) {
        this.tradeToken = tradeToken;
        this.amount = new BN(amount.toString());
        this.tradeTokenPriceData = tradeTokenPriceData;
    }

    public getTokenName(): string {
        return BumpinUtils.decodeString(this.tradeToken.name);
    }

    public getTokenBalanceUsd(): BigNumber {
        if (!this.tradeTokenPriceData.price) {
            throw new BumpinInvalidParameter("Price data not found");
        }
        return BumpinUtils.toUsd(
            this.amount,
            this.tradeTokenPriceData.price,
            this.tradeToken.decimals
        );
    }
}

export class AccountValue {
    netValue: BigNumber;
    totalMM: BigNumber;

    constructor(netValue: BigNumber, totalMM: BigNumber) {
        this.netValue = netValue;
        this.totalMM = totalMM;
    }
}

export class MarketUnPnlUsd {
    longUnPnl: BigNumber;
    shortUnPnl: BigNumber;

    constructor(longUnPnl: BigNumber, shortUnPnl: BigNumber) {
        this.longUnPnl = longUnPnl;
        this.shortUnPnl = shortUnPnl;
    }
}

export class WalletBalance {
    recognized: boolean;
    solLamports: number;
    solDecimals: number;
    tokenBalances: TokenBalance[];

    constructor(
        recognized: boolean,
        solLamports: number,
        solDecimals: number,
        tokenBalances: TokenBalance[]
    ) {
        this.recognized = recognized;
        this.solLamports = solLamports;
        this.solDecimals = solDecimals;
        this.tokenBalances = tokenBalances;
    }

    public getTokenBalance(mintKey: PublicKey): TokenBalance {
        for (let tokenBalance of this.tokenBalances) {
            if (tokenBalance.tradeToken.mintKey.equals(mintKey)) {
                return tokenBalance;
            }
        }
        throw new BumpinTokenNotFound(mintKey);
    }
}