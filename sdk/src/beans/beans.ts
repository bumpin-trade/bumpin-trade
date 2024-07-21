import { PublicKey } from "@solana/web3.js";
import BigNumber from "bignumber.js";
import {
  BorrowingFeeAccount,
  FeeRewardAccount,
  MarketAccount,
  MarketConfigAccount,
  MarketFundingFeeAccount,
  MarketPositionAccount,
  PoolAccount,
  PoolBalanceAccount,
  PoolConfigAccount,
  PositionStatusAccount,
  RewardsAccount,
  StateAccount,
  TradeTokenAccount,
  UserAccount,
  UserOrderAccount,
  UserPositionAccount,
  UserRewardsAccount,
  UserStakeAccount,
  UserStakeStatusAccount,
  UserTokenAccount,
  UserTokenStatusAccount,
} from "../typedef";
import { BumpinConstants } from "../consts";
import { BumpinUtils } from "../utils/utils";
import { isEqual } from "lodash";
import { BumpinTokenUtils } from "../utils/token";
import { BumpinPoolUtils } from "../utils/pool";

export class Beans {
  public admin: PublicKey;
  public bumpSigner: PublicKey;
  public keeperSigner: PublicKey;
  public bumpSignerNonce: number;
  public marketSequence: number;
  public poolSequence: number;
  public tradeTokenSequence: number;
  public minimumOrderMarginUsd: BigNumber;
  public maximumMaintenanceMarginRate: number;
  public fundingFeeBaseRate: BigNumber;
  public maximumFundingBaseRate: BigNumber;
  public minimumPrecisionMultiple: BigNumber;
  public poolRewardsIntervalLimit: BigNumber;
  public initFee: number;
  public tradingFeeUsdPoolRewardsRatio: number;
  public poolFeeRewardRatio: number;

  constructor(state: StateAccount) {
    this.admin = state.admin;
    this.bumpSigner = state.bumpSigner;
    this.keeperSigner = state.keeperSigner;
    this.bumpSignerNonce = state.bumpSignerNonce;
    this.marketSequence = state.marketSequence;
    this.poolSequence = state.poolSequence;
    this.tradeTokenSequence = state.tradeTokenSequence;
    this.minimumOrderMarginUsd =
      state.minimumOrderMarginUsd.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.maximumMaintenanceMarginRate =
      state.maximumMaintenanceMarginRate /
      BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.fundingFeeBaseRate = state.fundingFeeBaseRate.toBigNumberWithDecimals(
      BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
    );
    this.maximumFundingBaseRate =
      state.maximumFundingBaseRate.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.minimumPrecisionMultiple =
      state.minimumPrecisionMultiple.toBigNumber();
    this.poolRewardsIntervalLimit =
      state.poolRewardsIntervalLimit.toBigNumber();
    this.initFee = state.initFee;
    this.tradingFeeUsdPoolRewardsRatio =
      state.tradingFeeUsdPoolRewardsRatio /
      BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.poolFeeRewardRatio =
      state.poolFeeRewardRatio / BumpinConstants.RATE_MULTIPLIER_NUMBER;
  }
}

export class MarketPosition {
  public openInterest: BigNumber;
  public entryPrice: BigNumber;

  constructor(marketPosition: MarketPositionAccount) {
    this.openInterest = marketPosition.openInterest.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.entryPrice = marketPosition.entryPrice.toBigNumberWithDecimals(
      BumpinConstants.PRICE_EXPONENT_NUMBER
    );
  }
}

export class MarketConfig {
  public tickSize: BigNumber;
  public openFeeRate: BigNumber;
  public closeFeeRate: BigNumber;
  public maximumLongOpenInterestCap: BigNumber;
  public maximumShortOpenInterestCap: BigNumber;
  public longShortRatioLimit: BigNumber;
  public longShortOiBottomLimit: BigNumber;
  public maximumLeverage: number;
  public minimumLeverage: number;

  constructor(marketConfig: MarketConfigAccount) {
    this.tickSize = marketConfig.tickSize.toBigNumberWithDecimals(
      BumpinConstants.PRICE_EXPONENT_NUMBER
    );
    this.openFeeRate = marketConfig.openFeeRate.toBigNumberWithDecimals(
      BumpinConstants.RATE_MULTIPLIER_NUMBER
    );
    this.closeFeeRate = marketConfig.closeFeeRate.toBigNumberWithDecimals(
      BumpinConstants.RATE_MULTIPLIER_NUMBER
    );
    this.maximumLongOpenInterestCap =
      marketConfig.maximumLongOpenInterestCap.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.maximumShortOpenInterestCap =
      marketConfig.maximumShortOpenInterestCap.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.longShortRatioLimit =
      marketConfig.longShortRatioLimit.toBigNumberWithDecimals(
        BumpinConstants.RATE_MULTIPLIER_NUMBER
      );
    this.longShortOiBottomLimit =
      marketConfig.longShortOiBottomLimit.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.maximumLeverage =
      marketConfig.maximumLeverage / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.minimumLeverage =
      marketConfig.minimumLeverage / BumpinConstants.RATE_MULTIPLIER_NUMBER;
  }
}

export class MarketFundingFee {
  public longFundingFeeAmountPerSize: BigNumber;
  public shortFundingFeeAmountPerSize: BigNumber;
  public totalLongFundingFee: BigNumber;
  public totalShortFundingFee: BigNumber;
  public longFundingFeeRate: BigNumber;
  public shortFundingFeeRate: BigNumber;
  public updatedAt: BigNumber;

  constructor(
    marketFundingFee: MarketFundingFeeAccount,
    baseCoinDecimals: number,
    stableCoinDecimals: number
  ) {
    this.longFundingFeeAmountPerSize =
      marketFundingFee.longFundingFeeAmountPerSize.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.shortFundingFeeAmountPerSize =
      marketFundingFee.shortFundingFeeAmountPerSize.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.totalLongFundingFee =
      marketFundingFee.totalLongFundingFee.toBigNumberWithDecimals(
        baseCoinDecimals
      );
    this.totalShortFundingFee =
      marketFundingFee.totalShortFundingFee.toBigNumberWithDecimals(
        stableCoinDecimals
      );
    this.longFundingFeeRate =
      marketFundingFee.longFundingFeeRate.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.shortFundingFeeRate =
      marketFundingFee.shortFundingFeeRate.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.updatedAt = marketFundingFee.updatedAt.toBigNumber();
  }
}

export class Market {
  public longOpenInterest: MarketPosition;
  public shortOpenInterest: MarketPosition;
  public fundingFee: MarketFundingFee;
  public config: MarketConfig;
  public poolKey: PublicKey;
  public poolMintKey: PublicKey;
  public indexMintOracle: PublicKey;
  public stablePoolKey: PublicKey;
  public stablePoolMintKey: PublicKey;
  public index: number;
  public symbol: string;

  constructor(
    market: MarketAccount,
    baseCoinDecimals: number,
    stableCoinDecimals: number
  ) {
    this.longOpenInterest = new MarketPosition(market.longOpenInterest);
    this.shortOpenInterest = new MarketPosition(market.shortOpenInterest);
    this.fundingFee = new MarketFundingFee(
      market.fundingFee,
      baseCoinDecimals,
      stableCoinDecimals
    );
    this.config = new MarketConfig(market.config);
    this.poolKey = market.poolKey;
    this.poolMintKey = market.poolMintKey;
    this.indexMintOracle = market.indexMintOracle;
    this.stablePoolKey = market.stablePoolKey;
    this.stablePoolMintKey = market.stablePoolMintKey;
    this.index = market.index;
    this.symbol = BumpinUtils.decodeString(market.symbol);
  }
}

export class PoolBalance {
  public settleFundingFee: BigNumber;
  public amount: BigNumber;
  public holdAmount: BigNumber;
  public unSettleAmount: BigNumber;
  public settleFundingFeeAmount: BigNumber;
  public lossAmount: BigNumber;

  constructor(
    poolBalance: PoolBalanceAccount,
    poolMintTradeTokenDecimals: number
  ) {
    this.settleFundingFee =
      poolBalance.settleFundingFee.toBigNumberWithDecimals(
        poolMintTradeTokenDecimals
      );
    this.amount = poolBalance.amount.toBigNumberWithDecimals(
      poolMintTradeTokenDecimals
    );
    this.holdAmount = poolBalance.holdAmount.toBigNumberWithDecimals(
      poolMintTradeTokenDecimals
    );
    this.unSettleAmount = poolBalance.unSettleAmount.toBigNumberWithDecimals(
      poolMintTradeTokenDecimals
    );
    this.settleFundingFeeAmount =
      poolBalance.settleFundingFeeAmount.toBigNumberWithDecimals(
        poolMintTradeTokenDecimals
      );
    this.lossAmount = poolBalance.lossAmount.toBigNumberWithDecimals(
      poolMintTradeTokenDecimals
    );
  }
}

export class BorrowingFee {
  public totalBorrowingFee: BigNumber;
  public totalRealizedBorrowingFee: BigNumber;
  public cumulativeBorrowingFeePerToken: BigNumber;
  public updatedAt: BigNumber;

  constructor(borrowingFee: BorrowingFeeAccount, coinDecimals: number) {
    this.totalBorrowingFee =
      borrowingFee.totalBorrowingFee.toBigNumberWithDecimals(coinDecimals);
    this.totalRealizedBorrowingFee =
      borrowingFee.totalRealizedBorrowingFee.toBigNumberWithDecimals(
        coinDecimals
      );
    this.cumulativeBorrowingFeePerToken =
      borrowingFee.cumulativeBorrowingFeePerToken.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.updatedAt = borrowingFee.updatedAt.toBigNumber();
  }
}

export class FeeReward {
  public feeAmount: BigNumber;
  public unSettleFeeAmount: BigNumber;
  public cumulativeRewardsPerStakeToken: BigNumber;
  public lastRewardsPerStakeTokenDeltas: BigNumber[];

  constructor(feeReward: FeeRewardAccount, coinDecimals: number) {
    this.feeAmount = feeReward.feeAmount.toBigNumberWithDecimals(coinDecimals);
    this.unSettleFeeAmount =
      feeReward.unSettleFeeAmount.toBigNumberWithDecimals(coinDecimals);
    this.cumulativeRewardsPerStakeToken =
      feeReward.cumulativeRewardsPerStakeToken.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.lastRewardsPerStakeTokenDeltas =
      feeReward.lastRewardsPerStakeTokenDeltas.map((delta) =>
        delta.toBigNumberWithDecimals(
          BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
        )
      );
  }
}

export class PoolConfig {
  public minimumStakeAmount: BigNumber;
  public minimumUnStakeAmount: BigNumber;
  public poolLiquidityLimit: BigNumber;
  public borrowingInterestRate: BigNumber;
  public stakeFeeRate: number;
  public unStakeFeeRate: number;
  public unSettleMintRatioLimit: number;

  constructor(poolConfig: PoolConfigAccount) {
    this.minimumStakeAmount =
      poolConfig.minimumStakeAmount.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.minimumUnStakeAmount =
      poolConfig.minimumUnStakeAmount.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.poolLiquidityLimit =
      poolConfig.poolLiquidityLimit.toBigNumberWithDecimals(
        BumpinConstants.RATE_MULTIPLIER_NUMBER
      );
    this.borrowingInterestRate =
      poolConfig.borrowingInterestRate.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.stakeFeeRate =
      poolConfig.stakeFeeRate / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.unStakeFeeRate =
      poolConfig.unStakeFeeRate / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.unSettleMintRatioLimit =
      poolConfig.unSettleMintRatioLimit /
      BumpinConstants.RATE_MULTIPLIER_NUMBER;
  }
}

export enum PoolStatus {
  NORMAL,
  StakePaused,
  UnStakePaused,
}

export class Pool {
  public name: string;
  public pnl: BigNumber;
  public apr: BigNumber;
  public insuranceFundAmount: BigNumber;
  public totalSupply: BigNumber;
  public balance: PoolBalance;
  public stableBalance: PoolBalance;
  public borrowingFee: BorrowingFee;
  public feeReward: FeeReward;
  public stableFeeReward: FeeReward;
  public config: PoolConfig;
  public poolVaultKey: PublicKey;
  public key: PublicKey;
  public stableMintKey: PublicKey;
  public mintKey: PublicKey;
  public index: number;
  public status: PoolStatus;
  public stable: boolean;

  constructor(
    pool: PoolAccount,
    baseCoinDecimals: number,
    stableCoinDecimals: number
  ) {
    this.name = BumpinUtils.decodeString(pool.name);
    this.pnl = pool.pnl.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.apr = pool.apr.toBigNumberWithDecimals(
      BumpinConstants.RATE_MULTIPLIER_NUMBER
    );
    this.insuranceFundAmount =
      pool.insuranceFundAmount.toBigNumberWithDecimals(baseCoinDecimals);
    this.totalSupply =
      pool.totalSupply.toBigNumberWithDecimals(baseCoinDecimals);
    this.balance = new PoolBalance(pool.balance, baseCoinDecimals);
    this.stableBalance = new PoolBalance(
      pool.stableBalance,
      stableCoinDecimals
    );
    this.borrowingFee = new BorrowingFee(pool.borrowingFee, baseCoinDecimals);
    this.feeReward = new FeeReward(pool.feeReward, baseCoinDecimals);
    this.stableFeeReward = new FeeReward(
      pool.stableFeeReward,
      stableCoinDecimals
    );
    this.config = new PoolConfig(pool.config);
    this.poolVaultKey = pool.poolVaultKey;
    this.key = pool.key;
    this.stableMintKey = pool.stableMintKey;
    this.mintKey = pool.mintKey;
    this.index = pool.index;
    this.stable = pool.stable;
    this.status = isEqual(pool.status, PoolStatus.NORMAL)
      ? PoolStatus.NORMAL
      : isEqual(pool.status, PoolStatus.StakePaused)
      ? PoolStatus.StakePaused
      : PoolStatus.UnStakePaused;
  }
}

export enum UserStakeStatus {
  INIT,
  USING,
}

export class Rewards {
  public poolUnClaimAmount: BigNumber;
  public poolTotalRewardsAmount: BigNumber;
  public poolRewardsVault: PublicKey;
  public daoRewardsVault: PublicKey;
  public daoTotalRewardsAmount: BigNumber;
  public poolIndex: number;

  constructor(rewards: RewardsAccount, coinDecimals: number) {
    this.poolUnClaimAmount =
      rewards.poolUnClaimAmount.toBigNumberWithDecimals(coinDecimals);
    this.poolTotalRewardsAmount =
      rewards.poolTotalRewardsAmount.toBigNumberWithDecimals(coinDecimals);
    this.daoTotalRewardsAmount =
      rewards.daoTotalRewardsAmount.toBigNumberWithDecimals(coinDecimals);
    this.poolRewardsVault = rewards.poolRewardsVault;
    this.daoRewardsVault = rewards.daoRewardsVault;
    this.poolIndex = rewards.poolIndex;
  }
}

export class TradeToken {
  public totalLiability: BigNumber;
  public totalAmount: BigNumber;
  public mintKey: PublicKey;
  public oracleKey: PublicKey;
  public vaultKey: PublicKey;
  public name: string;
  public discount: number;
  public liquidationFactor: number;
  public index: number;
  public decimals: number;

  constructor(tradeToken: TradeTokenAccount) {
    this.totalLiability = tradeToken.totalLiability.toBigNumberWithDecimals(
      tradeToken.decimals
    );
    this.totalAmount = tradeToken.totalAmount.toBigNumberWithDecimals(
      tradeToken.decimals
    );
    this.mintKey = tradeToken.mintKey;
    this.oracleKey = tradeToken.oracleKey;
    this.vaultKey = tradeToken.vaultKey;
    this.name = BumpinUtils.decodeString(tradeToken.name);
    this.discount = tradeToken.discount;
    this.liquidationFactor =
      tradeToken.liquidationFactor / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.index = tradeToken.index;
    this.decimals = tradeToken.decimals;
  }
}

export class UserRewards {
  public totalClaimRewardsAmount: BigNumber;
  public realisedRewardsTokenAmount: BigNumber;
  public openRewardsPerStakeToken: BigNumber;
  public tokenKey: PublicKey;

  constructor(userRewards: UserRewardsAccount, coinDecimals: number) {
    this.totalClaimRewardsAmount =
      userRewards.totalClaimRewardsAmount.toBigNumberWithDecimals(coinDecimals);
    this.realisedRewardsTokenAmount =
      userRewards.realisedRewardsTokenAmount.toBigNumberWithDecimals(
        coinDecimals
      );
    this.openRewardsPerStakeToken =
      userRewards.openRewardsPerStakeToken.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.tokenKey = userRewards.tokenKey;
  }
}

export class UserStake {
  public stakedShare: BigNumber;
  public userRewards: UserRewards;
  public poolKey: PublicKey;
  public userStakeStatus: UserStakeStatus;

  constructor(userStake: UserStakeAccount, coinDecimals: number) {
    this.stakedShare =
      userStake.stakedShare.toBigNumberWithDecimals(coinDecimals);
    this.userRewards = new UserRewards(userStake.userRewards, coinDecimals);
    this.poolKey = userStake.poolKey;
    this.userStakeStatus = isEqual(
      userStake.userStakeStatus,
      UserStakeStatusAccount.INIT
    )
      ? UserStakeStatus.INIT
      : UserStakeStatus.USING;
  }
}

export enum UserTokenStatus {
  INIT,
  USING,
}

export class UserToken {
  public amount: BigNumber;
  public usedAmount: BigNumber;
  public liabilityAmount: BigNumber;
  public tokenMintKey: PublicKey;
  public userTokenAccountKey: PublicKey;
  public userTokenStatus: UserTokenStatus;

  constructor(userToken: UserTokenAccount, coinDecimals: number) {
    this.amount = userToken.amount.toBigNumberWithDecimals(coinDecimals);
    this.usedAmount =
      userToken.usedAmount.toBigNumberWithDecimals(coinDecimals);
    this.liabilityAmount =
      userToken.liabilityAmount.toBigNumberWithDecimals(coinDecimals);
    this.tokenMintKey = userToken.tokenMintKey;
    this.userTokenAccountKey = userToken.userTokenAccountKey;
    this.userTokenStatus = isEqual(
      userToken.userTokenStatus,
      UserTokenStatusAccount.INIT
    )
      ? UserTokenStatus.INIT
      : UserTokenStatus.USING;
  }
}

export enum PositionStatus {
  INIT,
  USING,
}

export class UserPosition {
  public positionSize: BigNumber;
  public entryPrice: BigNumber;
  public initialMargin: BigNumber;
  public initialMarginUsd: BigNumber;
  public initialMarginUsdFromPortfolio: BigNumber;
  public mmUsd: BigNumber;
  public holdPoolAmount: BigNumber;
  public openFee: BigNumber;
  public openFeeInUsd: BigNumber;
  public realizedBorrowingFee: BigNumber;
  public realizedBorrowingFeeInUsd: BigNumber;
  public openBorrowingFeePerToken: BigNumber;
  public realizedFundingFee: BigNumber;
  public realizedFundingFeeInUsd: BigNumber;
  public openFundingFeeAmountPerSize: BigNumber;
  public closeFeeInUsd: BigNumber;
  public realizedPnl: BigNumber;
  public userKey: PublicKey;
  public marginMintKey: PublicKey;
  public indexMintOracle: PublicKey;
  public positionKey: PublicKey;
  public symbol: string;
  public updatedAt: BigNumber;
  public leverage: number;
  public isLong: boolean;
  public isPortfolioMargin: boolean;
  public status: PositionStatus;

  constructor(
    userPosition: UserPositionAccount,
    baseCoinDecimals: number,
    stableCoinDecimals: number
  ) {
    this.positionSize = userPosition.positionSize.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.entryPrice = userPosition.entryPrice.toBigNumberWithDecimals(
      BumpinConstants.PRICE_EXPONENT_NUMBER
    );
    this.initialMargin = userPosition.initialMargin.toBigNumberWithDecimals(
      userPosition.isLong ? baseCoinDecimals : stableCoinDecimals
    );
    this.initialMarginUsd =
      userPosition.initialMarginUsd.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.initialMarginUsdFromPortfolio =
      userPosition.initialMarginUsdFromPortfolio.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.mmUsd = userPosition.mmUsd.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.holdPoolAmount = userPosition.holdPoolAmount.toBigNumberWithDecimals(
      userPosition.isLong ? baseCoinDecimals : stableCoinDecimals
    );
    this.openFee = userPosition.openFee.toBigNumberWithDecimals(
      userPosition.isLong ? baseCoinDecimals : stableCoinDecimals
    );
    this.openFeeInUsd = userPosition.openFeeInUsd.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.realizedBorrowingFee =
      userPosition.realizedBorrowingFee.toBigNumberWithDecimals(
        userPosition.isLong ? baseCoinDecimals : stableCoinDecimals
      );
    this.realizedBorrowingFeeInUsd =
      userPosition.realizedBorrowingFeeInUsd.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.openBorrowingFeePerToken =
      userPosition.openBorrowingFeePerToken.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.realizedFundingFee =
      userPosition.realizedFundingFee.toBigNumberWithDecimals(
        userPosition.isLong ? baseCoinDecimals : stableCoinDecimals
      );
    this.realizedFundingFeeInUsd =
      userPosition.realizedFundingFeeInUsd.toBigNumberWithDecimals(
        BumpinConstants.USD_EXPONENT_NUMBER
      );
    this.openFundingFeeAmountPerSize =
      userPosition.openFundingFeeAmountPerSize.toBigNumberWithDecimals(
        BumpinConstants.SMALL_RATE_MULTIPLIER_NUMBER
      );
    this.closeFeeInUsd = userPosition.closeFeeInUsd.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.realizedPnl = userPosition.realizedPnl.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.userKey = userPosition.userKey;
    this.marginMintKey = userPosition.marginMintKey;
    this.indexMintOracle = userPosition.indexMintOracle;
    this.positionKey = userPosition.positionKey;
    this.symbol = BumpinUtils.decodeString(userPosition.symbol);
    this.updatedAt = userPosition.updatedAt.toBigNumber();
    this.leverage =
      userPosition.leverage / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.isLong = userPosition.isLong;
    this.isPortfolioMargin = userPosition.isPortfolioMargin;
    this.status = isEqual(userPosition.status, PositionStatusAccount.INIT)
      ? PositionStatus.INIT
      : PositionStatus.USING;
  }
}

export enum OrderSide {
  NONE,
  LONG,
  SHORT,
}

export enum OrderStatus {
  INIT,
  USING,
}

export enum PositionSide {
  NONE,
  INCREASE,
  DECREASE,
}

export enum OrderType {
  NONE,
  MARKET,
  LIMIT,
  STOP,
}

export enum StopType {
  NONE,
  StopLoss,
  TakeProfit,
}

/**
 * export type UserOrderAccount = {
 *     orderMargin: BN;
 *     orderSize: BN;
 *     triggerPrice: BN;
 *     acceptablePrice: BN;
 *     createdAt: BN;
 *     orderId: BN;
 *     marginMintKey: PublicKey;
 *     authority: PublicKey;
 *     symbol: number[];
 *     leverage: number;
 *     orderSide: OrderSideAccount;
 *     positionSide: PositionSideAccount;
 *     orderType: OrderTypeAccount;
 *     stopType: StopTypeAccount;
 *     status: OrderStatusAccount;
 *     isPortfolioMargin: boolean;
 * };
 *
 * export class UserStatusAccount {
 *     static readonly NORMAL = {normal: {}};
 *     static readonly LIQUIDATION = {liquidation: {}};
 *     static readonly DISABLE = {disable: {}};
 * }
 */

export enum UserStatus {
  NORMAL,
  LIQUIDATION,
  DISABLE,
}

export class UserOrder {
  public orderMargin: BigNumber;
  public orderSize: BigNumber;
  public triggerPrice: BigNumber;
  public acceptablePrice: BigNumber;
  public createdAt: BigNumber;
  public orderId: BigNumber;
  public marginMintKey: PublicKey;
  public authority: PublicKey;
  public symbol: string;
  public leverage: number;
  public orderSide: OrderSide;
  public positionSide: PositionSide;
  public orderType: OrderType;
  public stopType: StopType;
  public status: OrderStatus;
  public isPortfolioMargin: boolean;

  constructor(userOrder: UserOrderAccount, coinDecimals: number) {
    this.orderMargin =
      userOrder.orderMargin.toBigNumberWithDecimals(coinDecimals);
    this.orderSize = userOrder.orderSize.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.triggerPrice = userOrder.triggerPrice.toBigNumberWithDecimals(
      BumpinConstants.PRICE_EXPONENT_NUMBER
    );
    this.acceptablePrice = userOrder.acceptablePrice.toBigNumberWithDecimals(
      BumpinConstants.PRICE_EXPONENT_NUMBER
    );
    this.createdAt = userOrder.createdAt.toBigNumber();
    this.orderId = userOrder.orderId.toBigNumber();
    this.marginMintKey = userOrder.marginMintKey;
    this.authority = userOrder.authority;
    this.symbol = BumpinUtils.decodeString(userOrder.symbol);
    this.leverage = userOrder.leverage / BumpinConstants.RATE_MULTIPLIER_NUMBER;
    this.orderSide = isEqual(userOrder.orderSide, OrderSide.LONG)
      ? OrderSide.LONG
      : isEqual(userOrder.orderSide, OrderSide.SHORT)
      ? OrderSide.SHORT
      : OrderSide.NONE;
    this.positionSide = isEqual(userOrder.positionSide, PositionSide.INCREASE)
      ? PositionSide.INCREASE
      : isEqual(userOrder.positionSide, PositionSide.DECREASE)
      ? PositionSide.DECREASE
      : PositionSide.NONE;
    this.orderType = isEqual(userOrder.orderType, OrderType.MARKET)
      ? OrderType.MARKET
      : isEqual(userOrder.orderType, OrderType.LIMIT)
      ? OrderType.LIMIT
      : isEqual(userOrder.orderType, OrderType.STOP)
      ? OrderType.STOP
      : OrderType.NONE;
    this.stopType = isEqual(userOrder.stopType, StopType.StopLoss)
      ? StopType.StopLoss
      : isEqual(userOrder.stopType, StopType.TakeProfit)
      ? StopType.TakeProfit
      : StopType.NONE;
    this.status = isEqual(userOrder.status, OrderStatus.INIT)
      ? OrderStatus.INIT
      : OrderStatus.USING;
    this.isPortfolioMargin = userOrder.isPortfolioMargin;
  }
}

/**
 * export type UserAccount = {
 *     nextOrderId: BN;
 *     nextLiquidationId: BN;
 *     hold: BN;
 *     tokens: UserTokenAccount[];
 *     stakes: UserStakeAccount[];
 *     positions: UserPositionAccount[];
 *     orders: UserOrderAccount[];
 *     key: PublicKey;
 *     authority: PublicKey;
 *     createdAt: BN;
 *     status: UserStatusValue;
 * };
 */

export class User {
  public nextOrderId: BigNumber;
  public nextLiquidationId: BigNumber;
  public hold: BigNumber;
  public tokens: UserToken[];
  public stakes: UserStake[];
  public positions: UserPosition[];
  public orders: UserOrder[];
  public key: PublicKey;
  public authority: PublicKey;
  public createdAt: BigNumber;
  public status: UserStatus;

  constructor(
    user: UserAccount,
    pools: PoolAccount[],
    tradeTokens: TradeTokenAccount[]
  ) {
    this.nextOrderId = user.nextOrderId.toBigNumber();
    this.nextLiquidationId = user.nextLiquidationId.toBigNumber();
    this.hold = user.hold.toBigNumberWithDecimals(
      BumpinConstants.USD_EXPONENT_NUMBER
    );
    this.tokens = user.tokens.map((token) => {
      const target = BumpinTokenUtils.getTradeTokenByMintPublicKey(
        token.tokenMintKey,
        tradeTokens
      );
      return new UserToken(token, target.decimals);
    });
    this.stakes = user.stakes.map((stake) => {
      const targetPool = BumpinPoolUtils.getPoolByMintPublicKey(
        stake.poolKey,
        pools
      );
      const target = BumpinTokenUtils.getTradeTokenByMintPublicKey(
        targetPool.mintKey,
        tradeTokens
      );
      return new UserStake(stake, target.decimals);
    });
    this.positions = user.positions.map((position) => {
      const indexTarget = BumpinTokenUtils.getTradeTokenByMintPublicKey(
        position.indexMintOracle,
        tradeTokens
      );
      return new UserPosition(
        position,
        indexTarget.decimals,
        BumpinTokenUtils.getTradeTokenByMintPublicKey(
          position.marginMintKey,
          tradeTokens
        ).decimals
      );
    });
    this.orders = user.orders.map((order) => {
      const target = BumpinTokenUtils.getTradeTokenByMintPublicKey(
        order.marginMintKey,
        tradeTokens
      );
      return new UserOrder(order, target.decimals);
    });
    this.key = user.key;
    this.authority = user.authority;
    this.createdAt = user.createdAt.toBigNumber();
    this.status = isEqual(user.status, UserStatus.NORMAL)
      ? UserStatus.NORMAL
      : isEqual(user.status, UserStatus.LIQUIDATION)
      ? UserStatus.LIQUIDATION
      : UserStatus.DISABLE;
  }
}
