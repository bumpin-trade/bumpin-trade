import { ConfirmOptions, Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json";
import idlPyth from "./idl/pyth.json";
import { BumpinClientConfig, NetType } from "./bumpinClientConfig";
import { BumpinUtils } from "./utils/utils";
import { BumpinTrade } from "./types/bumpin_trade";
import {

  AccountValue,
  Market,
  MarketUnPnlUsd,
  MarketWithIndexTradeTokenPrices,
  PlaceOrderParams,
  Pool,
  PoolSummary,
  Rewards,
  StateAccount,
  TokenBalance,
  TradeToken,
  UserAccount,
  UserClaimResult,
  UserClaimRewardsResult,
  UserStakeStatus,
  WalletBalance,
} from "./typedef";
import {
  BumpinAccountNotFound,
  BumpinClientNotInitialized,
  BumpinInvalidParameter,
  BumpinSubscriptionFailed,
  BumpinUserNotLogin,
} from "./errors";
import { PollingUserAccountSubscriber } from "./account/pollingUserAccountSubscriber";
import { BulkAccountLoader } from "./account/bulkAccountLoader";
import { DataAndSlot } from "./account/types";
import { PollingStateAccountSubscriber } from "./account/pollingStateAccountSubscriber";
import { PoolComponent } from "./componets/pool";
import { Pyth } from "./types/pyth";
import { PythClient } from "./oracles/pythClient";
import { UserComponent } from "./componets/user";
import { TradeTokenComponent } from "./componets/tradeToken";
import { MarketComponent } from "./componets/market";
import { BumpinTokenUtils } from "./utils/token";
import { BumpinPoolUtils } from "./utils/pool";
import { BumpinMarketUtils } from "./utils/market";
import { TEN, ZERO } from "./constants/numericConstants";
import { PriceData } from "@pythnetwork/client";
import BigNumber from "bignumber.js";
import { AccountLayout, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { RewardsComponent } from "./componets/rewards";
import "./types/bnExt";

export class BumpinClient {
  netType: NetType;
  connection: Connection;
  wallet: Wallet;
  provider: AnchorProvider;
  public program: Program<BumpinTrade>;

  isInitialized: boolean = false;
  bulkAccountLoader: BulkAccountLoader;

  programPyth: Program<Pyth> | undefined;
  pythClient: PythClient;

  // Systems subscriptions
  stateSubscriber: PollingStateAccountSubscriber;
  userAccountSubscriber: PollingUserAccountSubscriber | undefined;

  // Components
  poolComponent: PoolComponent | undefined;
  rewardComponent: RewardsComponent | undefined;
  tradeTokenComponent: TradeTokenComponent | undefined;
  marketComponent: MarketComponent | undefined;
  userComponent: UserComponent | undefined;

  constructor(config: BumpinClientConfig) {
    this.netType = config.netType;
    this.connection = new Connection(config.endpoint, config.connectionConfig);
    this.wallet = config.wallet;
    let opt: ConfirmOptions = {
      skipPreflight: false,
      commitment: "confirmed", //default commitment: confirmed
      preflightCommitment: "confirmed",
      maxRetries: 0,
      minContextSlot: undefined,
    };
    this.provider = new anchor.AnchorProvider(
      this.connection,
      this.wallet,
      opt
    );
    this.program = new anchor.Program(
      JSON.parse(JSON.stringify(idlBumpinTrade)),
      this.provider
    );
    this.bulkAccountLoader = new BulkAccountLoader(
      this.connection,
      "confirmed",
      config.pollingFrequency
    );

    const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
    this.stateSubscriber = new PollingStateAccountSubscriber(
      this.program,
      statePda,
      this.bulkAccountLoader
    );

    if (this.netType === NetType.LOCALNET || this.netType === NetType.CUSTOM) {
      this.programPyth = new anchor.Program(
        JSON.parse(JSON.stringify(idlPyth)),
        this.provider
      );
      this.pythClient = new PythClient(this.programPyth.provider.connection);
    } else {
      this.pythClient = new PythClient(this.connection);
    }
  }

  public hasWallet(): boolean {
    return this.wallet !== null;
  }

  public async initialize() {
    if (this.isInitialized) {
      return;
    }

    await this.stateSubscriber.subscribe();

    this.poolComponent = new PoolComponent(
      this.bulkAccountLoader,
      this.stateSubscriber,
      this.program
    );
    const p1 = this.poolComponent.subscribe();

    this.tradeTokenComponent = new TradeTokenComponent(
      this.bulkAccountLoader,
      this.stateSubscriber,
      this.program
    );
    const p2 = this.tradeTokenComponent.subscribe();

    this.marketComponent = new MarketComponent(
      this.bulkAccountLoader,
      this.stateSubscriber,
      this.program
    );
    const p3 = this.marketComponent.subscribe();

    this.rewardComponent = new RewardsComponent(
      this.bulkAccountLoader,
      this.stateSubscriber,
      this.program
    );
    const p4 = this.rewardComponent.subscribe();

    await Promise.all([p1, p2, p3, p4]);
    this.isInitialized = true;
    console.log("BumpinClient initialized");
  }

  public async login(): Promise<UserAccount> {
    this.checkInitialization();
    const [pda, _] = BumpinUtils.getPdaSync(this.program, [
      Buffer.from("user"),
      this.wallet.publicKey.toBuffer(),
    ]);
    try {
      let me = (await this.program.account.user.fetch(
        pda
      )) as any as UserAccount;
      if (me) {
        this.userComponent = new UserComponent(
          this.wallet.publicKey,
          this.pythClient,
          this.bulkAccountLoader,
          this.stateSubscriber,
          this.program
        );
        await this.userComponent.subscribe();
        console.log("User logged in");
      }
      return me;
    } catch (e) {
      throw new BumpinAccountNotFound(
        "User Account, pda: " +
          pda.toString() +
          " wallet: " +
          this.wallet.publicKey.toString()
      );
    }
    //TODO: Maybe has another error type
  }

  public async initializeUser() {
    this.checkInitialization();

    await this.program.methods
      .initializeUser()
      .accounts({
        authority: this.wallet.publicKey,
        payer: this.wallet.publicKey,
      })
      .signers([])
      .rpc();
  }

  public async getWalletBalance(
    recognized: boolean = true,
    sync: boolean = false
  ): Promise<WalletBalance> {
    this.checkInitialization(true);

    const tradeTokens = await this.getTradeTokens(sync);

    const balance = await this.connection.getBalance(this.wallet.publicKey);
    const userTokenAccounts = await this.connection.getTokenAccountsByOwner(
      this.wallet.publicKey,
      {
        programId: TOKEN_PROGRAM_ID,
      }
    );
    const accounts = userTokenAccounts.value.map((accountInfo: any) => {
      const key: PublicKey = accountInfo.publicKey;
      const accountData = AccountLayout.decode(accountInfo.account.data);
      const mint = accountData.mint;
      const amount = accountData.amount;
      return {
        key,
        mint,
        amount,
      };
    });

    let available = accounts;
    if (recognized) {
      available = accounts.filter((account) => {
        return tradeTokens.some((tradeToken) => {
          return tradeToken.mintKey.equals(account.mint);
        });
      });
    }

    let tokenBalances: TokenBalance[] = available.map((account) => {
      const tradeToken = tradeTokens.find((tradeToken) => {
        return tradeToken.mintKey.equals(account.mint);
      });
      if (!tradeToken) {
        throw new BumpinAccountNotFound(
          "TradeToken, mint: " + account.mint.toString()
        );
      }
      const tradeTokenPriceData = this.getTradeTokenPrice(
        BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0]
      );
      return new TokenBalance(tradeToken, account.amount, tradeTokenPriceData);
    });

    return new WalletBalance(recognized, balance, 9, tokenBalances);
  }

  public getTradeTokenPrice(tradeTokenKey: PublicKey): PriceData {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeTokenPrices(tradeTokenKey, 1)[0];
  }

  public async getTradeTokenPriceByMintKey(
    mintKey: PublicKey
  ): Promise<PriceData> {
    this.checkInitialization();
    let res = await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
      mintKey,
      1
    );
    return res[0];
  }

  public async getTradeTokenPriceByOracleKey(
    oracleKey: PublicKey
  ): Promise<PriceData> {
    this.checkInitialization();
    let res = this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
      oracleKey,
      1
    );
    return res[0];
  }

  public async getPoolSummary(
    stashedPrice: number = 2,
    sync: boolean = false
  ): Promise<PoolSummary[]> {
    this.checkInitialization();

    let poolSummaries: PoolSummary[] = [];

    let pools = await this.getPools(sync);
    let markets = await this.getMarkets(sync);

    for (let pool of pools) {
      let poolSummary: PoolSummary = {
        pool: pool,
        netPrice: await this.getPoolNetPrice(pool.key, sync),
        categoryTags: [],
        markets: [],
      };
      let isMixed = false;
      for (let market of markets) {
        if (
          market.poolKey.equals(pool.key) ||
          market.stablePoolKey.equals(pool.key)
        ) {
          let prices = this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
            market.indexMintOracle,
            stashedPrice
          );

          let marketWithPrices: MarketWithIndexTradeTokenPrices = {
            ...market,
            indexTradeTokenPrices: prices,
          };

          poolSummary.markets.push(marketWithPrices);
          //TODO: fix
          // if (!market.indexMintKey.equals(market.poolMintKey)) {
          //     isMixed = true;
          // }
        }
      }
      if (pool.stable) {
        poolSummary.categoryTags.push("stable_pool");
      } else if (isMixed) {
        poolSummary.categoryTags.push("mix_pool");
      } else {
        poolSummary.categoryTags.push("standard_pool");
      }
      poolSummaries.push(poolSummary);
    }

    return poolSummaries;
  }

    public async stake(
        fromPortfolio: boolean,
        size: number,
        mint: PublicKey,
        sync: boolean = false
    ) {
        this.checkInitialization(true);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mint,
            await this.getTradeTokens()
        );
        let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(
            mint,
            await this.getPools()
        );
        if (fromPortfolio) {
            await this.userComponent!.portfolioStake(
                size,
                targetTradeToken,
                await this.getTradeTokens(),
                targetPool,
                await this.getMarkets(sync), await this.getPools(sync),
                sync
            );
        } else {
            await this.userComponent!.walletStake(
                size,
                targetTradeToken,
                await this.getTradeTokens(),
                this.wallet.publicKey,
                targetPool,
                await this.getMarkets(sync),
                sync
            );
        }
    }

  public async unStake(
    toPortfolio: boolean,
    share: BN,
    mint: PublicKey,
    sync: boolean = false
  ) {
    this.checkInitialization(true);

    let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
      mint,
      await this.getTradeTokens()
    );
    let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(
      mint,
      await this.getPools()
    );
    let markets = await this.getMarkets(sync);
    await this.userComponent!.unStake(
      toPortfolio,
      share,
      targetTradeToken,
      this.wallet.publicKey,
      targetPool,
      markets
    );
  }

  public async deposit(
    userTokenAccount: PublicKey,
    mintPublicKey: PublicKey,
    size: number
  ) {
    this.checkInitialization(true);

    const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
    let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
      mintPublicKey,
      await this.getTradeTokens()
    );
    let amount = BumpinUtils.size2Amount(
      new BigNumber(size),
      targetTradeToken.decimals
    );
    await this.program.methods
      .deposit(targetTradeToken.index, amount)
      .accounts({
        userTokenAccount,
      })
      .signers([])
      .rpc();
  }

  public async placePerpOrder(
    marketIndex: number,
    param: PlaceOrderParams,
    sync: boolean = false
  ) {
    this.checkInitialization(true);

    let market = BumpinMarketUtils.getMarketByIndex(
      marketIndex,
      await this.getMarkets(sync)
    );
    await this.userComponent!.placePerpOrder(
      market.symbol,
      marketIndex,
      param,
      this.wallet.publicKey,
      await this.poolComponent!.getPools(sync),
      await this.marketComponent!.getMarkets(),
      await this.tradeTokenComponent!.getTradeTokens(sync)
    );
  }

  public async getUser(sync: boolean = false): Promise<UserAccount> {
    this.checkInitialization(true);
    return this.userComponent!.getUser(sync);
  }

  public async getState(sync: boolean = false): Promise<StateAccount> {
    if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
      throw new BumpinSubscriptionFailed("State", 0);
    }

    if (sync) {
      await this.stateSubscriber.fetch();
    }

    let state = this.stateSubscriber.state;
    if (!state) {
      throw new BumpinAccountNotFound("State");
    }
    return state.data;
  }

  public async getPools(sync: boolean = false): Promise<Pool[]> {
    this.checkInitialization();
    return this.poolComponent!.getPools(sync);
  }

  public async getRewards(sync: boolean = false): Promise<Rewards[]> {
    this.checkInitialization();
    return this.rewardComponent!.getRewards(sync);
  }

  public async getPoolsWithSlot(
    sync: boolean = false
  ): Promise<DataAndSlot<Pool>[]> {
    this.checkInitialization();
    return this.poolComponent!.getPoolsWithSlot(sync);
  }

  public async getPool(
    poolKey: PublicKey,
    sync: boolean = false
  ): Promise<Pool> {
    this.checkInitialization();
    return this.poolComponent!.getPool(poolKey, sync);
  }

  public async getPoolByIndex(
    poolIndex: number,
    sync: boolean = false
  ): Promise<Pool> {
    this.checkInitialization();
    return this.poolComponent!.getPool(
      BumpinUtils.getPoolPda(this.program, poolIndex)[0],
      sync
    );
  }

  public async getPoolWithSlot(
    poolKey: PublicKey,
    sync: boolean = false
  ): Promise<DataAndSlot<Pool>> {
    this.checkInitialization();
    return this.poolComponent!.getPoolWithSlot(poolKey, sync);
  }

  public async getTradeTokens(sync: boolean = false): Promise<TradeToken[]> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeTokens(sync);
  }

  public async getTradeTokensWithSlot(
    sync: boolean = false
  ): Promise<DataAndSlot<TradeToken>[]> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeTokensWithSlot(sync);
  }

  public async getTradeToken(
    tradeTokenKey: PublicKey,
    sync: boolean = false
  ): Promise<TradeToken> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeToken(tradeTokenKey, sync);
  }

  public async getTradeTokenByIndex(
    tradeTokenIndex: number,
    sync: boolean = false
  ): Promise<TradeToken> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeToken(
      BumpinUtils.getTradeTokenPda(this.program, tradeTokenIndex)[0],
      sync
    );
  }

  public async getTradeTokenByMintKey(
    mintKey: PublicKey,
    sync: boolean = false
  ): Promise<TradeToken> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeTokenByMintKey(mintKey, sync);
  }

  public async getTradeTokenByOracleKey(
        oracleKey: PublicKey,
        sync: boolean = false
    ): Promise<TradeToken> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokenByOracleKey(oracleKey, sync);
    }public async getTradeTokenWithSlot(
    tradeTokenKey: PublicKey,
    sync: boolean = false
  ): Promise<DataAndSlot<TradeToken>> {
    this.checkInitialization();
    return this.tradeTokenComponent!.getTradeTokenWithSlot(tradeTokenKey, sync);
  }

  public async getMarkets(sync: boolean = false): Promise<Market[]> {
    this.checkInitialization();
    return this.marketComponent!.getMarkets(sync);
  }

  public async getMarketsWithSlot(
    sync: boolean = false
  ): Promise<DataAndSlot<Market>[]> {
    this.checkInitialization();
    return this.marketComponent!.getMarketsWithSlot(sync);
  }

  public async getMarket(
    marketKey: PublicKey,
    sync: boolean = false
  ): Promise<Market> {
    this.checkInitialization();
    return this.marketComponent!.getMarket(marketKey, sync);
  }

  public async getMarketByIndex(
    marketIndex: number,
    sync: boolean = false
  ): Promise<Market> {
    this.checkInitialization();
    return this.marketComponent!.getMarket(
      BumpinUtils.getMarketPda(this.program, marketIndex)[0],
      sync
    );
  }

  public async getMarketWithSlot(
    marketKey: PublicKey,
    sync: boolean = false
  ): Promise<DataAndSlot<Market>> {
    this.checkInitialization();
    return this.marketComponent!.getMarketWithSlot(marketKey, sync);
  }

  public async getFundingFeeRate(
    marketIndex: number,
    sync: boolean = false
  ): Promise<{
    long: number;
    short: number;
  }> {
    this.checkInitialization();
    const marketKey = BumpinUtils.getMarketPda(this.program, marketIndex)[0];
    const market = await this.getMarket(marketKey, sync);
    let long = market.fundingFee.longFundingFeeRate.toBigNumber().div(10 ** 10);
    let short = market.fundingFee.shortFundingFeeRate
      .toBigNumber()
      .div(10 ** 10);
    if (long.isNaN()) {
      long = new BigNumber(0);
    }
    if (short.isNaN()) {
      short = new BigNumber(0);
    }
    return {
      long: long.toNumber(),
      short: short.toNumber(),
    };
  }

  public async getBorrowingFeeRate(
    marketIndex: number,
    sync: boolean = false
  ): Promise<number> {
    this.checkInitialization();
    const marketKey = BumpinUtils.getMarketPda(this.program, marketIndex)[0];
    const timestamp = BigNumber(Math.floor(Date.now() / 1000));
    const market = await this.getMarket(marketKey, sync);
    const pool = await this.getPool(market.poolKey, sync);
    const timePassed = timestamp.minus(
      pool.borrowingFee.updatedAt.toBigNumber()
    );

    return pool.balance.holdAmount
      .toBigNumber()
      .div(
        pool.balance.amount
          .toBigNumber()
          .plus(pool.balance.unSettleAmount.toBigNumber())
      )
      .multipliedBy(pool.config.borrowingInterestRate.toBigNumber())
      .multipliedBy(timePassed)
      .div(timePassed)
      .toNumber();
  }

  public async getPoolNetPrice(poolKey: PublicKey, sync: boolean = false) {
    this.checkInitialization();
    const pool = await this.getPool(poolKey, sync);
    const tradeToken = await this.getTradeTokenByMintKey(pool.mintKey, sync);
    const poolValueUsd = await this.getPoolValueUsd(poolKey, sync);
    if (pool.totalSupply.isZero()) {
      return new BigNumber(0);
    } else {
      return poolValueUsd.div(
        pool.totalSupply
          .toBigNumber()
          .div(BigNumber(10).pow(BigNumber(tradeToken.decimals)))
      );
    }
  }

  public async getPoolValueUsd(
    poolKey: PublicKey,
    sync: boolean = false
  ): Promise<BigNumber> {
    this.checkInitialization();
    const pool = await this.getPool(poolKey, sync);
    const tradeToken = await this.getTradeTokenByMintKey(pool.mintKey, sync);
    const price = (
      await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
        pool.mintKey,
        1,
        sync
      )
    )[0];
    if (!price.price) {
      throw new BumpinInvalidParameter(
        "Price not found(undefined) for mint: " + pool.mintKey.toString()
      );
    }
    const relativeMarkets = BumpinMarketUtils.getMarketsByPoolKey(
      poolKey,
      await this.getMarkets(sync)
    );
    // self usd value
    let rawValue = BumpinUtils.toUsd(
      pool.balance.amount.add(pool.balance.unSettleAmount),
      price.price,
      tradeToken.decimals
    );

    if (!pool.stable) {
      // relative market unpnl usd value
      for (let relativeMarket of relativeMarkets) {
        const marketUnPnlUsd = await this.getMarketUnPnlUsd(relativeMarket);
        rawValue = rawValue
          .plus(marketUnPnlUsd.longUnPnl)
          .plus(marketUnPnlUsd.shortUnPnl);
      }
      // relative stable pool gain and loss
      const stableAmount = pool.stableBalance.amount
        .add(pool.stableBalance.unSettleAmount)
        .sub(pool.stableBalance.lossAmount);
      if (stableAmount.gt(ZERO)) {
        const stablePrice = (
          await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
            pool.stableMintKey,
            1,
            sync
          )
        )[0];
        if (!stablePrice.price) {
          throw new BumpinInvalidParameter(
            "Stable Price not found(undefined) for mint: " +
              pool.mintKey.toString()
          );
        }
        rawValue = rawValue.plus(
          BumpinUtils.toUsd(
            stableAmount,
            stablePrice.price,
            tradeToken.decimals
          )
        );
      }
    }

    return rawValue;
  }

  public async getMarketUnPnlUsd(market: Market): Promise<MarketUnPnlUsd> {
    this.checkInitialization();
    let longUnPnl = ZERO;
    let shortUnPnl = ZERO;

    const longPosition = market.longOpenInterest;
    const shortPosition = market.shortOpenInterest;
    const price = this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
      market.indexMintOracle,
      1
    )[0];

    if (!price.price) {
      throw new BumpinInvalidParameter(
        "Price not found(undefined) for oracle: " +
          market.indexMintOracle.toString()
      );
    }

    // cal long:
    if (!longPosition.entryPrice.isZero()) {
      longUnPnl = longPosition.openInterest
        .mul(
          new BN(price.price)
            .mul(TEN.pow(new BN(Math.abs(price.exponent))))
            .sub(longPosition.entryPrice)
        )
        .div(longPosition.entryPrice)
        .mul(new BN(-1));
    }

    // cal short:
    if (!shortPosition.entryPrice.isZero()) {
      shortUnPnl = shortPosition.openInterest
        .mul(
          shortPosition.entryPrice.sub(
            new BN(price.price).mul(TEN.pow(new BN(Math.abs(price.exponent))))
          )
        )
        .div(shortPosition.entryPrice)
        .mul(new BN(-1));
    }

    return new MarketUnPnlUsd(
      new BigNumber(longUnPnl.toString()).dividedBy(
        new BigNumber(10).pow(Math.abs(price.exponent))
      ),
      new BigNumber(shortUnPnl.toString()).dividedBy(
        new BigNumber(10).pow(Math.abs(price.exponent))
      )
    );
  }

    public async getUserAccountNetValue(
        sync: boolean = false
    ): Promise<AccountValue> {
        this.checkInitialization(true);
        let accountValue = {
            netValue: new BigNumber(0),
            totalMM: new BigNumber(0),
        };
        const user = await this.getUser(sync);

        let accountNetValue = await this.userComponent!.getUserAccountNetValue(user, await this.getTradeTokens(), await this.getMarkets(), await this.getPools());
        accountValue.netValue = BumpinUtils.amount2Size(accountNetValue.accountNetValue, 8);
        accountValue.totalMM = BumpinUtils.amount2Size(accountNetValue.totalMM, 8);
        return accountValue;
    }

  public async getUserAvailableValue(
    sync: boolean = false
  ): Promise<BigNumber> {
    this.checkInitialization(true);
    const user = await this.getUser(sync);
    let pools = await this.getPools();
    const poolMap: Map<PublicKey, Pool> = new Map();
    pools.forEach((pool) => {
      poolMap.set(pool.key, pool);
    });

    let markets = await this.getMarkets();
    const marketMap: Map<number[], Market> = new Map();
    markets.forEach((market) => {
      marketMap.set(market.symbol, market);
    });
    return BumpinUtils.amount2Size(
      await this.userComponent!.getUserAvailableValue(
        user,
        await this.getTradeTokens(),
        await this.getMarkets(), await this.getPools()),
      8
    );
  }

  public async claimUserRewards(): Promise<UserClaimResult> {
    this.checkInitialization(true);
    let user = await this.getUser();
    let claimResult: UserClaimResult = {
      claimed: new BN(0),
      unClaim: new BN(0),
      total: new BN(0),
      rewards: [],
    };
    for (const stake of user.stakes) {
      if (
        stake.userStakeStatus == UserStakeStatus.USING &&
        stake.userRewards.openRewardsPerStakeToken.gt(ZERO)
      ) {
        let pool = await this.getPool(stake.poolKey);
        let oraclePriceData = await this.pythClient.getOraclePriceData(
          stake.userRewards.tokenKey
        );
        let unRealisedRewards = pool.feeReward.cumulativeRewardsPerStakeToken
          .sub(stake.userRewards.openRewardsPerStakeToken)
          .mulSmallRate(stake.stakedShare)
          .downSmallRate();

        claimResult.total = claimResult.total.add(
          unRealisedRewards
            .add(stake.userRewards.totalClaimRewardsAmount.downSmallRate())
            .mul(oraclePriceData.price)
            .downPrice()
        );
        claimResult.claimed = claimResult.claimed.add(
          stake.userRewards.totalClaimRewardsAmount
            .downSmallRate()
            .mul(oraclePriceData.price)
            .downPrice()
        );
        claimResult.unClaim = claimResult.unClaim.add(
          unRealisedRewards.mul(oraclePriceData.price).downPrice()
        );
        let userClaimRewardsResult: UserClaimRewardsResult = {
          pool: pool.name,
          rewardsAmount: unRealisedRewards
            .mul(oraclePriceData.price)
            .downPrice(),
        };
        claimResult.rewards.push(userClaimRewardsResult);
      }
    }
    return claimResult;
  }

  public checkInitialization(mustLogin: boolean = false) {
    if (!this.isInitialized) {
      throw new BumpinClientNotInitialized();
    }
    if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
      throw new BumpinClientNotInitialized("State");
    }

    if (!this.poolComponent) {
      throw new BumpinClientNotInitialized("Pool");
    }

    if (!this.rewardComponent) {
      throw new BumpinClientNotInitialized("Reward");
    }

    if (!this.tradeTokenComponent) {
      throw new BumpinClientNotInitialized("TradeToken");
    }

    if (!this.marketComponent) {
      throw new BumpinClientNotInitialized("Market");
    }

    if (mustLogin && !this.userComponent) {
      throw new BumpinUserNotLogin();
    }
  }
}
