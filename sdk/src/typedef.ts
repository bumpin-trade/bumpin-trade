import { PublicKey } from '@solana/web3.js';
// import BN from "bn.js";
import { BN } from '@coral-xyz/anchor';
import { PriceData } from '@pythnetwork/client';
import { BumpinUtils } from './utils/utils';
import { BumpinInvalidParameter, BumpinTokenNotFound } from './errors';
import BigNumber from 'bignumber.js';
import { isEqual } from 'lodash';
import {
    MarketConfig,
    MarketFundingFee,
    MarketPosition,
    OrderSide,
    OrderStatus,
    OrderType,
    Pool,
    PoolStatus,
    PositionSide,
    PositionStatus,
    StopType,
    TradeToken,
    UserStakeStatus,
    UserStatus,
    UserTokenStatus,
} from './beans/beans';

export class OracleSource {
    static readonly PYTH = { pyth: {} };
    static readonly PYTH_1K = { pyth1K: {} };
    static readonly PYTH_1M = { pyth1M: {} };
    static readonly SWITCHBOARD = { switchboard: {} };
    static readonly QUOTE_ASSET = { quoteAsset: {} };
    static readonly PYTH_STABLE_COIN = { pythStableCoin: {} };
    static readonly Prelaunch = { prelaunch: {} };
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
    maxPoolLiquidityShareRate: number;
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

export type ModifyStateParams = {
    minOrderMarginUsd: BN | null;
    maximumMaintenanceMarginRate: number | null;
    fundingFeeBaseRate: BN | null;
    maxFundingBaseRate: BN | null;
    tradingFeeStakingRewardsRatio: number | null;
    tradingFeePoolRewardsRatio: number | null;
    tradingFeeUsdPoolRewardsRatio: number | null;
    minPrecisionMultiple: BN | null;
    poolRewardsIntervalLimit: BN | null;
    initFee: BN | null;
    stakingFeeRewardRatio: number | null;
    poolFeeRewardRatio: number | null;
    essentialAccountAlt: number[] | null;
};

export type InitializePoolParams = {
    name: number[];
    stableMintKey: number[];
    poolConfig: PoolConfigAccount;
    stable: boolean;
};

export type StateAccount = {
    admin: PublicKey;
    bumpSigner: PublicKey;
    keeperSigner: PublicKey;
    essentialAccountAlt: PublicKey;
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

export type MarketPositionAccount = {
    openInterest: BN;
    entryPrice: BN;
};

export type MarketConfigAccount = {
    tickSize: BN;
    openFeeRate: BN;
    closeFeeRate: BN;
    maximumLongOpenInterestCap: BN;
    maximumShortOpenInterestCap: BN;
    longShortRatioLimit: BN;
    longShortOiBottomLimit: BN;
    maximumLeverage: number;
    minimumLeverage: number;
    maxPoolLiquidityShareRate: number;
};

export type MarketFundingFeeAccount = {
    longFundingFeeAmountPerSize: BN;
    shortFundingFeeAmountPerSize: BN;
    totalLongFundingFee: BN;
    totalShortFundingFee: BN;
    longFundingFeeRate: BN;
    shortFundingFeeRate: BN;
    updatedAt: BN;
};

export type MarketAccount = {
    longOpenInterest: MarketPositionAccount;
    shortOpenInterest: MarketPositionAccount;
    fundingFee: MarketFundingFeeAccount;
    config: MarketConfigAccount;
    poolKey: PublicKey;
    poolMintKey: PublicKey;
    indexMintOracle: PublicKey;
    stablePoolKey: PublicKey;
    stablePoolMintKey: PublicKey;
    index: number;
    symbol: number[];
};

export type PoolBalanceAccount = {
    settleFundingFee: BN;
    amount: BN;
    holdAmount: BN;
    unSettleAmount: BN;
    lossAmount: BN;
};

export type BorrowingFeeAccount = {
    totalBorrowingFee: BN;
    totalRealizedBorrowingFee: BN;
    cumulativeBorrowingFeePerToken: BN;
    updatedAt: BN;
};

export type FeeRewardAccount = {
    feeAmount: BN;
    unSettleFeeAmount: BN;
    cumulativeRewardsPerStakeToken: BN;
    lastRewardsPerStakeTokenDeltas: BN[];
};

export type PoolConfigAccount = {
    minimumStakeAmount: BN;
    minimumUnStakeAmount: BN;
    poolLiquidityLimit: BN;
    borrowingInterestRate: BN;
    stakeFeeRate: number;
    unStakeFeeRate: number;
    unSettleMintRatioLimit: number;
    padding: number[];
};

export class PoolStatusAccount {
    static readonly NORMAL = { init: {} };
    static readonly StakePaused = { stakePaused: {} };
    static readonly UnStakePaused = { unStakePaused: {} };

    toString() {
        return isEqual(this, PoolStatusAccount.NORMAL)
            ? 'Normal'
            : isEqual(this, PoolStatusAccount.StakePaused)
            ? 'StakePaused'
            : 'UnStakePaused';
    }

    public static from(o: PoolStatus) {
        if (o === PoolStatus.NORMAL) {
            return PoolStatusAccount.NORMAL;
        } else if (o === PoolStatus.StakePaused) {
            return PoolStatusAccount.StakePaused;
        } else {
            return PoolStatusAccount.UnStakePaused;
        }
    }
}

export type PoolAccount = {
    name: number[];
    pnl: BN;
    apr: BN;
    insuranceFundAmount: BN;
    totalSupply: BN;
    balance: PoolBalanceAccount;
    borrowingFee: BorrowingFeeAccount;
    feeReward: FeeRewardAccount;
    config: PoolConfigAccount;
    poolVaultKey: PublicKey;
    key: PublicKey;
    stableMintKey: PublicKey;
    mintKey: PublicKey;
    index: number;
    status: PoolStatusAccount;
    stable: boolean;
};

export type RewardsAccount = {
    poolUnClaimAmount: BN;
    poolTotalRewardsAmount: BN;
    poolRewardsVault: PublicKey;
    daoRewardsVault: PublicKey;
    daoTotalRewardsAmount: BN;
    poolIndex: number;
};

export type TradeTokenAccount = {
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

export class UserStakeStatusAccount {
    static readonly INIT = { init: {} };
    static readonly USING = { using: {} };

    toString() {
        return isEqual(this, UserStakeStatusAccount.INIT) ? 'Init' : 'Using';
    }

    public static from(o: UserStakeStatus) {
        if (o === UserStakeStatus.INIT) {
            return UserStakeStatusAccount.INIT;
        } else {
            return UserStakeStatusAccount.USING;
        }
    }
}

export type UserRewardsAccount = {
    totalClaimRewardsAmount: BN;
    realisedRewardsTokenAmount: BN;
    openRewardsPerStakeToken: BN;
    tokenKey: PublicKey;
};

export type UserStakeAccount = {
    stakedShare: BN;
    userRewards: UserRewardsAccount;
    poolKey: PublicKey;
    userStakeStatus: UserStakeStatusAccount;
};

export class UserTokenStatusAccount {
    static readonly INIT = { init: {} };
    static readonly USING = { using: {} };

    toString() {
        return isEqual(this, UserTokenStatusAccount.INIT) ? 'Init' : 'Using';
    }

    public static from(o: UserTokenStatus) {
        if (o === UserTokenStatus.INIT) {
            return UserTokenStatusAccount.INIT;
        } else {
            return UserTokenStatusAccount.USING;
        }
    }
}

export type UserTokenAccount = {
    amount: BN;
    usedAmount: BN;
    liabilityAmount: BN;
    tokenMintKey: PublicKey;
    userTokenStatus: UserTokenStatusAccount;
};

export class PositionStatusAccount {
    static readonly INIT = { init: {} };
    static readonly USING = { using: {} };

    toString() {
        return isEqual(this, PositionStatusAccount.INIT) ? 'Init' : 'Using';
    }

    public static from(o: PositionStatus) {
        if (o === PositionStatus.INIT) {
            return PositionStatus.INIT;
        } else {
            return PositionStatus.USING;
        }
    }
}

export type UserPositionAccount = {
    positionSize: BN;
    entryPrice: BN;
    marginTokenEntryPrice: BN;
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
    userTokenAccount: PublicKey;
    marginMintKey: PublicKey;
    indexMintOracle: PublicKey;
    positionKey: PublicKey;
    symbol: number[];
    updatedAt: BN;
    leverage: number;
    isLong: boolean;
    isPortfolioMargin: boolean;
    status: PositionStatusAccount;
};

export class OrderSideAccount {
    static readonly NONE = { none: {} };
    static readonly LONG = { long: {} };
    static readonly SHORT = { short: {} };

    toString() {
        return isEqual(this, OrderSideAccount.NONE)
            ? 'None'
            : isEqual(this, OrderSideAccount.LONG)
            ? 'Long'
            : 'Short';
    }

    public static from(o: OrderSide) {
        if (o === OrderSide.LONG) {
            return OrderSideAccount.LONG;
        } else if (o === OrderSide.SHORT) {
            return OrderSideAccount.SHORT;
        } else {
            return OrderSideAccount.NONE;
        }
    }
}

export class OrderStatusAccount {
    static readonly INIT = { init: {} };
    static readonly USING = { using: {} };

    toString() {
        return isEqual(this, OrderStatusAccount.INIT) ? 'Init' : 'Using';
    }

    public static from(o: OrderStatus) {
        if (o === OrderStatus.INIT) {
            return OrderStatus.INIT;
        } else {
            return OrderStatus.USING;
        }
    }
}

export class PositionSideAccount {
    static readonly NONE = { none: {} };
    static readonly INCREASE = { increase: {} };
    static readonly DECREASE = { decrease: {} };

    toString() {
        return isEqual(this, PositionSideAccount.NONE)
            ? 'None'
            : isEqual(this, PositionSideAccount.INCREASE)
            ? 'Increase'
            : 'Decrease';
    }

    public static from(o: PositionSide) {
        if (o === PositionSide.INCREASE) {
            return PositionSideAccount.INCREASE;
        } else if (o === PositionSide.DECREASE) {
            return PositionSideAccount.DECREASE;
        } else {
            return PositionSideAccount.NONE;
        }
    }
}

export class OrderTypeAccount {
    static readonly NONE = { none: {} };
    static readonly MARKET = { market: {} };
    static readonly LIMIT = { limit: {} };
    static readonly STOP = { stop: {} };

    toString() {
        return isEqual(this, OrderTypeAccount.NONE)
            ? 'None'
            : isEqual(this, OrderTypeAccount.MARKET)
            ? 'Market'
            : isEqual(this, OrderTypeAccount.LIMIT)
            ? 'Limit'
            : 'Stop';
    }

    public static from(o: OrderType) {
        if (o === OrderType.MARKET) {
            return OrderTypeAccount.MARKET;
        } else if (o === OrderType.LIMIT) {
            return OrderTypeAccount.LIMIT;
        } else if (o === OrderType.STOP) {
            return OrderTypeAccount.STOP;
        } else {
            return OrderTypeAccount.NONE;
        }
    }
}

export class StopTypeAccount {
    static readonly NONE = { none: {} };
    static readonly StopLoss = { stopLoss: {} };
    static readonly TakeProfit = { takeProfit: {} };

    toString() {
        return isEqual(this, StopTypeAccount.NONE)
            ? 'None'
            : isEqual(this, StopTypeAccount.StopLoss)
            ? 'StopLoss'
            : 'TakeProfit';
    }

    public static from(o: StopType) {
        if (o === StopType.StopLoss) {
            return StopTypeAccount.StopLoss;
        } else if (o === StopType.TakeProfit) {
            return StopTypeAccount.TakeProfit;
        } else {
            return StopTypeAccount.NONE;
        }
    }
}

export type OrderSideValue =
    | typeof OrderSideAccount.NONE
    | typeof OrderSideAccount.LONG
    | typeof OrderSideAccount.SHORT;

export type OrderStatusValue =
    | typeof OrderStatusAccount.INIT
    | typeof OrderStatusAccount.USING;

export type PositionSideValue =
    | typeof PositionSideAccount.NONE
    | typeof PositionSideAccount.INCREASE
    | typeof PositionSideAccount.DECREASE;

export type OrderTypeValue =
    | typeof OrderTypeAccount.NONE
    | typeof OrderTypeAccount.MARKET
    | typeof OrderTypeAccount.LIMIT
    | typeof OrderTypeAccount.STOP;

export type StopTypeValue =
    | typeof StopTypeAccount.NONE
    | typeof StopTypeAccount.StopLoss
    | typeof StopTypeAccount.TakeProfit;

export type UserOrderAccount = {
    orderMargin: BN;
    orderSize: BN;
    triggerPrice: BN;
    acceptablePrice: BN;
    createdAt: BN;
    orderId: BN;
    marginMintKey: PublicKey;
    authority: PublicKey;
    userTokenAccount: PublicKey;
    symbol: number[];
    leverage: number;
    orderSide: OrderSideAccount;
    positionSide: PositionSideAccount;
    orderType: OrderTypeAccount;
    stopType: StopTypeAccount;
    status: OrderStatusAccount;
    isPortfolioMargin: boolean;
};

export class UserStatusAccount {
    static readonly NORMAL = { normal: {} };
    static readonly LIQUIDATION = { liquidation: {} };
    static readonly DISABLE = { disable: {} };

    toString() {
        return isEqual(this, UserStatusAccount.NORMAL)
            ? 'Normal'
            : isEqual(this, UserStatusAccount.LIQUIDATION)
            ? 'Liquidation'
            : 'Disable';
    }

    public static from(o: UserStatus) {
        if (o === UserStatus.NORMAL) {
            return UserStatus.NORMAL;
        } else if (o === UserStatus.LIQUIDATION) {
            return UserStatus.LIQUIDATION;
        } else {
            return UserStatus.DISABLE;
        }
    }
}

export type UserStatusValue =
    | typeof UserStatusAccount.NORMAL
    | typeof UserStatusAccount.LIQUIDATION
    | typeof UserStatusAccount.DISABLE;

export type UserAccount = {
    nextOrderId: BN;
    nextLiquidationId: BN;
    hold: BN;
    tokens: UserTokenAccount[];
    stakes: UserStakeAccount[];
    positions: UserPositionAccount[];
    orders: UserOrderAccount[];
    key: PublicKey;
    authority: PublicKey;
    createdAt: BN;
    status: UserStatusValue;
};

export type TradeTokenBalance = {
    tokenNetValue: BigNumber;
    tokenUsedValue: BigNumber;
    tokenBorrowingValue: BigNumber;
};
export type AccountNetValue = {
    accountNetValue: BigNumber;
    totalMM: BigNumber;
};
export type PositionBalance = {
    // total_im_usd
    initialMarginUsd: BigNumber;
    // total_im_usd_from_portfolio
    initialMarginUsdFromPortfolio: BigNumber;
    // total_un_pnl_usd
    positionUnPnl: BigNumber;
    // total_position_mm
    mmUsd: BigNumber;
    positionFee: BigNumber;
};

export type PositionFee = {
    fundingFee: BigNumber;
    fundingFeeUsd: BigNumber;
    borrowingFee: BigNumber;
    borrowingFeeUsd: BigNumber;
    closeFeeUsd: BigNumber;
    totalUsd: BigNumber;
};

export type PlaceOrderParams = {
    isPortfolioMargin: boolean;
    isNativeToken: boolean;
    orderSide: OrderSide;
    positionSide: PositionSide;
    orderType: OrderType;
    stopType: StopType;
    size: number;
    orderMargin: number;
    leverage: number;
    triggerPrice: number;
    acceptablePrice: number;
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
    symbol: string;
    indexTradeTokenPrices: PriceData[];
};

export type PoolSummary = {
    pool: Pool;
    netPrice: BigNumber;
    availableLiquidate: BigNumber;
    categoryTags: string[];
    markets: MarketWithIndexTradeTokenPrices[];
};
export type UserSummary = {
    accountNetValue: BigNumber;
    pnl: BigNumber;
    earn: BigNumber;
    tokens: UserTokenSummary[];
    liabilityRatio: BigNumber;
    apr: BigNumber;
};
export type UserTokenSummary = {
    token: TradeToken;
    amount: BigNumber;
    used: BigNumber;
    borrow: BigNumber;
};

export type UserClaimResult = {
    totalStakingValue: BigNumber;
    totalApr: BigNumber;
    totalRewards: BigNumber;
    totalClaimed: BigNumber;
    totalUnClaim: BigNumber;
    rewards: Array<UserClaimRewardsResult>;
};
export type UserClaimRewardsResult = {
    pool: string;
    poolIndex: number;
    decimals: number;
    rewardsAmount: BigNumber;
};

export class TokenBalance {
    tradeToken: TradeToken;
    amount: BN;
    tokenAccountPublicKey: PublicKey;
    tradeTokenPriceData: PriceData;

    constructor(
        tradeToken: TradeToken,
        amount: bigint,
        tokenAccountPublicKey: PublicKey,
        tradeTokenPriceData: PriceData,
    ) {
        this.tradeToken = tradeToken;
        this.tokenAccountPublicKey = tokenAccountPublicKey;
        this.amount = new BN(amount.toString());
        this.tradeTokenPriceData = tradeTokenPriceData;
    }

    public getTokenName(): string {
        return this.tradeToken.name;
    }

    public getTokenBalanceUsd(): BigNumber {
        if (!this.tradeTokenPriceData.price) {
            throw new BumpinInvalidParameter('Price data not found');
        }
        return BumpinUtils.toUsd(
            this.amount,
            this.tradeTokenPriceData.price,
            this.tradeToken.decimals,
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

export class PositionSettle {
    settleMargin: BigNumber;
    positionFee: BigNumber;

    constructor(settleMargin: BigNumber, positionFee: BigNumber) {
        this.settleMargin = settleMargin;
        this.positionFee = positionFee;
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
        tokenBalances: TokenBalance[],
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
