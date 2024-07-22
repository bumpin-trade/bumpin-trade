import { AccountMeta, PublicKey } from "@solana/web3.js";
import {
  AccountNetValue,
  InnerPlaceOrderParams,
  OrderSideAccount,
  OrderTypeAccount,
  PlaceOrderParams,
  PoolAccount,
  PositionSideAccount,
  StopTypeAccount,
  TradeTokenAccount,
  UserStakeStatusAccount,
  UserTokenStatusAccount,
} from "../typedef";
import { BulkAccountLoader } from "../account/bulkAccountLoader";
import { BN, Program } from "@coral-xyz/anchor";
import { BumpinUtils } from "../utils/utils";
import { BumpinTrade } from "../types/bumpin_trade";
// @ts-ignore
import { isEqual } from "lodash";
import { Component } from "./componet";
import { PollingStateAccountSubscriber } from "../account/pollingStateAccountSubscriber";
import { PollingUserAccountSubscriber } from "../account/pollingUserAccountSubscriber";
import {
  BumpinAccountNotFound,
  BumpinInvalidParameter,
  BumpinSubscriptionFailed,
  BumpinSupplyInsufficient,
  BumpinTokenAccountUnexpected,
  BumpinValueInsufficient,
} from "../errors";
import { DataAndSlot } from "../account/types";
import { BumpinTokenUtils } from "../utils/token";
import { BumpinPositionUtils } from "../utils/position";
import { BumpinPoolUtils } from "../utils/pool";
import { Account, AccountLayout } from "@solana/spl-token";
import BigNumber from "bignumber.js";
import { BumpinMarketUtils } from "../utils/market";
import { C } from "../consts";
import { ZERO } from "../constants/numericConstants";
import { Market, Pool, TradeToken, User } from "../beans/beans";
import { TradeTokenComponent } from "./tradeToken";
import { PoolComponent } from "./pool";

export class UserComponent extends Component {
  publicKey: PublicKey;
  program: Program<BumpinTrade>;
  userAccountSubscriber: PollingUserAccountSubscriber;
  tradeTokenComponent: TradeTokenComponent;
  poolComponent: PoolComponent;

  constructor(
    publicKey: PublicKey,
    bulkAccountLoader: BulkAccountLoader,
    stateSubscriber: PollingStateAccountSubscriber,
    tradeTokenComponent: TradeTokenComponent,
    poolComponent: PoolComponent,
    program: Program<BumpinTrade>
  ) {
    super(stateSubscriber, program);
    this.publicKey = publicKey;
    this.program = program;
    this.tradeTokenComponent = tradeTokenComponent;
    this.poolComponent = poolComponent;
    const [pda, _] = BumpinUtils.getPdaSync(this.program, [
      Buffer.from("user"),
      this.publicKey.toBuffer(),
    ]);
    this.userAccountSubscriber = new PollingUserAccountSubscriber(
      this.program,
      pda,
      bulkAccountLoader,
      tradeTokenComponent,
      poolComponent
    );
  }

  public async subscribe() {
    await this.userAccountSubscriber.subscribe();
  }

  public async unsubscribe() {
    await this.userAccountSubscriber.unsubscribe();
  }

  public async portfolioStake(
    size: number,
    tradeToken: TradeToken,
    allTradeTokens: TradeToken[],
    pool: Pool,
    allMarkets: Market[],
    pools: Pool[],
    sync: boolean = false
  ): Promise<void> {
    let user = await this.getUser(sync);
    let amount = BumpinUtils.size2Amount(
      new BigNumber(size),
      tradeToken.decimals
    );
    let stake_value = await this.checkStakeAmountFulfilRequirements(
      amount,
      tradeToken,
      pool
    );
    let availableValue = await this.getUserAvailableValue(
      user,
      allTradeTokens,
      allMarkets,
      pools
    );
    if (!availableValue.gt(stake_value)) {
      throw new BumpinValueInsufficient(amount.toBigNumber(), availableValue);
    }

    let remainingAccounts = this.getUserRemainingAccounts(
      await this.getUser(),
      allTradeTokens
    );
    let markets = BumpinMarketUtils.getMarketsByPoolKey(pool.key, allMarkets);
    for (let market of markets.values()) {
      remainingAccounts.push({
        pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
        isWritable: true,
        isSigner: false,
      });
    }

    await this.program.methods
      .portfolioStake(pool.index, tradeToken.index, amount)
      .accounts({
        authority: this.publicKey,
        bumpSigner: (await this.getState()).bumpSigner,
      })
      .remainingAccounts(remainingAccounts)
      .signers([])
      .rpc();
  }

  public async walletStake(
    size: number,
    tradeToken: TradeToken,
    allTradeTokens: TradeToken[],
    wallet: PublicKey,
    pool: Pool,
    allMarkets: Market[],
    sync: boolean = false
  ): Promise<void> {
    // let user = await this.getUser(sync);
    let amount = BumpinUtils.size2Amount(
      new BigNumber(size),
      tradeToken.decimals
    );
    await this.checkStakeAmountFulfilRequirements(amount, tradeToken, pool);
    await this.checkStakeWalletAmountSufficient(amount, wallet, tradeToken);
    let tokenAccount =
      await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
        this.program.provider.connection,
        wallet,
        tradeToken.mintKey
      );

    let remainingAccounts = [];
    remainingAccounts.push({
      pubkey: tradeToken.mintKey,
      isWritable: false,
      isSigner: false,
    });
    remainingAccounts.push({
      pubkey: tradeToken.oracleKey,
      isWritable: false,
      isSigner: false,
    });
    let pda = BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0];
    remainingAccounts.push({
      pubkey: pda,
      isWritable: false,
      isSigner: false,
    });

    let markets = BumpinMarketUtils.getMarketsByPoolKey(pool.key, allMarkets);
    for (let market of markets) {
      remainingAccounts.push({
        pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
        isWritable: true,
        isSigner: false,
      });
    }

    await this.program.methods
      .walletStake(pool.index, tradeToken.index, amount)
      .accounts({
        authority: wallet,
        userTokenAccount: tokenAccount.address,
      })
      .remainingAccounts(remainingAccounts)
      .signers([])
      .rpc();
  }

  public async unStake(
    portfolio: boolean,
    share: BigNumber,
    tradeToken: TradeToken,
    wallet: PublicKey,
    pool: Pool,
    allMarkets: Market[]
  ): Promise<void> {
    let userStake = await this.findUsingStake(pool.key, false);
    if (!userStake) {
      throw new BumpinInvalidParameter("User stake not found");
    }
    if (share.gt(userStake.stakedShare)) {
      throw new BumpinValueInsufficient(userStake.stakedShare, share);
    }
    if (pool.totalSupply.isZero()) {
      throw new BumpinSupplyInsufficient(share, BigNumber(0));
    }

    let param = {
      share: new BN(share.toString()),
      poolIndex: pool.index,
      tradeTokenIndex: tradeToken.index,
    };

    let remainingAccounts = [];
    remainingAccounts.push({
      pubkey: tradeToken.mintKey,
      isWritable: false,
      isSigner: false,
    });
    remainingAccounts.push({
      pubkey: tradeToken.oracleKey,
      isWritable: false,
      isSigner: false,
    });
    let pda = BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0];
    remainingAccounts.push({
      pubkey: pda,
      isWritable: false,
      isSigner: false,
    });

    let markets = BumpinMarketUtils.getMarketsByPoolKey(pool.key, allMarkets);
    for (let market of markets) {
      remainingAccounts.push({
        pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
        isWritable: true,
        isSigner: false,
      });
    }

    if (portfolio) {
      await this.program.methods
        .portfolioUnStake(param)
        .accounts({
          authority: wallet,
        })
        .remainingAccounts(remainingAccounts)
        .signers([])
        .rpc();
    } else {
      let tokenAccount =
        await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
          this.program.provider.connection,
          wallet,
          tradeToken.mintKey
        );
      await this.program.methods
        .walletUnStake(param)
        .accounts({
          authority: wallet,
          userTokenAccount: tokenAccount.address,
          bumpSigner: (await this.getState()).bumpSigner,
        })
        .remainingAccounts(remainingAccounts)
        .signers([])
        .rpc();
    }
  }

  public async placePerpOrder(
    symbol: string,
    marketIndex: number,
    param: PlaceOrderParams,
    wallet: PublicKey,
    pools: Pool[],
    markets: Market[],
    tradeTokens: TradeToken[]
  ) {
    const user = await this.getUser();
    const pool = BumpinPoolUtils.getPoolByMintPublicKey(
      markets[marketIndex].poolMintKey,
      pools
    );
    const stablePool = BumpinPoolUtils.getPoolByMintPublicKey(
      markets[marketIndex].stablePoolMintKey,
      pools
    );
    const tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
      markets[marketIndex].poolMintKey,
      tradeTokens
    );
    const stableTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
      markets[marketIndex].stablePoolMintKey,
      tradeTokens
    );

    let userTokenAccount = (
      await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
        this.program.provider.connection,
        wallet,
        tradeToken.mintKey
      )
    ).address;

    if (
      (isEqual(param.positionSide, PositionSideAccount.DECREASE) &&
        isEqual(param.orderSide, OrderSideAccount.LONG)) ||
      (isEqual(param.positionSide, PositionSideAccount.INCREASE) &&
        isEqual(param.orderSide, OrderSideAccount.SHORT))
    ) {
      // When the order side is short, the userTokenAccount is the stable token.
      userTokenAccount = (
        await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
          this.program.provider.connection,
          wallet,
          markets[marketIndex].stablePoolMintKey
        )
      ).address;
    } // When trading position by position (Isolated position), userTokenAccount is determined based on the order direction.
    let uta = userTokenAccount;
    if (!param.isPortfolioMargin) {
      let tokenAccount: Account =
        await BumpinTokenUtils.getTokenAccountFromWalletAndKey(
          this.program.provider.connection,
          wallet,
          userTokenAccount
        );
      if (isEqual(param.positionSide, PositionSideAccount.INCREASE)) {
        if (isEqual(param.orderSide, OrderSideAccount.LONG)) {
          if (!tokenAccount.mint.equals(pool.mintKey)) {
            throw new BumpinTokenAccountUnexpected(
              "Pool mint key: " + pool.mintKey.toString(),
              "Token account mint key: " + tokenAccount.mint.toString()
            );
          }
        } else {
          if (!tokenAccount.mint.equals(stablePool.mintKey)) {
            throw new BumpinTokenAccountUnexpected(
              "Stable Pool mint key: " + stablePool.mintKey.toString(),
              "Token account mint key: " + tokenAccount.mint.toString()
            );
          }
        }
      } else {
        if (isEqual(param.orderSide, OrderSideAccount.LONG)) {
          if (!tokenAccount.mint.equals(stablePool.mintKey)) {
            throw new BumpinTokenAccountUnexpected(
              "Stable Pool mint key: " + stablePool.mintKey.toString(),
              "Token account mint key: " + tokenAccount.mint.toString()
            );
          }
        } else {
          if (!tokenAccount.mint.equals(pool.mintKey)) {
            throw new BumpinTokenAccountUnexpected(
              "Pool mint key: " + pool.mintKey.toString(),
              "Token account mint key: " + tokenAccount.mint.toString()
            );
          }
        }
      }
      uta = tokenAccount.address;
    }

    let remainingAccounts = this.getUserRemainingAccounts(
      user,
      tradeTokens,
      true
    );

    for (let market of markets) {
      //append all markets which base token is pool.mint
      if (market.poolKey.equals(pool.key)) {
        remainingAccounts.push({
          pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
          isWritable: true,
          isSigner: false,
        });

        remainingAccounts.push({
          pubkey: market.indexMintOracle,
          isWritable: true,
          isSigner: false,
        });
      }
    }

    remainingAccounts.push({
      pubkey: BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0],
      isWritable: true,
      isSigner: false,
    });
    remainingAccounts.push({
      pubkey: tradeToken.oracleKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: BumpinUtils.getPoolPda(this.program, pool.index)[0],
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: BumpinUtils.getPoolPda(this.program, stablePool.index)[0],
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: pool.poolVaultKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: stablePool.poolVaultKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: tradeToken.vaultKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: stableTradeToken.vaultKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: BumpinUtils.getTradeTokenPda(
        this.program,
        stableTradeToken.index
      )[0],
      isWritable: true,
      isSigner: false,
    });
    remainingAccounts.push({
      pubkey: stableTradeToken.oracleKey,
      isWritable: false,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: markets[marketIndex].poolMintKey,
      isWritable: true,
      isSigner: false,
    });

    remainingAccounts.push({
      pubkey: markets[marketIndex].stablePoolKey,
      isWritable: true,
      isSigner: false,
    });

    let indexPrice = this.tradeTokenComponent.getTradeTokenPricesByOracleKey(
      markets[marketIndex].indexMintOracle,
      0
    )[0];
    if (!indexPrice.price) {
      throw new BumpinInvalidParameter(
        "Price not found(undefined) for mint: " + pool.mintKey.toString()
      );
    }

    let order: InnerPlaceOrderParams = {
      symbol: BumpinUtils.encodeString(symbol),
      placeTime: new BN(Date.now()),
      isPortfolioMargin: false,
      isNativeToken: false,
      orderSide: param.orderSide,
      positionSide: param.positionSide,
      orderType: param.orderType,
      stopType: param.stopType,
      size: BumpinUtils.number2Precision(param.size, C.USD_EXPONENT_NUMBER),
      orderMargin: !param.isPortfolioMargin
        ? BumpinUtils.number2Precision(
            param.orderMargin,
            isEqual(param.positionSide, PositionSideAccount.INCREASE)
              ? isEqual(param.orderSide, OrderSideAccount.LONG)
                ? tradeToken.decimals
                : stableTradeToken.decimals
              : isEqual(param.orderSide, OrderSideAccount.LONG)
              ? stableTradeToken.decimals
              : tradeToken.decimals
          )
        : BumpinUtils.number2Precision(
            param.orderMargin * indexPrice.price,
            C.USD_EXPONENT_NUMBER
          ),
      leverage: param.leverage * C.RATE_MULTIPLIER,
      triggerPrice: BumpinUtils.number2Precision(
        param.triggerPrice,
        C.PRICE_EXPONENT_NUMBER
      ),
      //TODO: recheck this
      acceptablePrice: BumpinUtils.number2Precision(
        param.acceptablePrice,
        C.PRICE_EXPONENT_NUMBER
      ),
    };

    await this.placePerpOrderValidation(
      param,
      indexPrice.price,
      markets[marketIndex]
    );
    await this.program.methods
      .placeOrder(order)
      .accounts({
        userTokenAccount: uta,
        authority: wallet,
        bumpSigner: (await this.getState()).bumpSigner,
      })
      .remainingAccounts(BumpinUtils.removeDuplicateAccounts(remainingAccounts))
      .signers([])
      .rpc();
  }

  //TODO: recheck this conditions
  async placePerpOrderValidation(
    order: PlaceOrderParams,
    tradeTokenPrice: number,
    market: Market,
    sync: boolean = false
  ) {
    let state = await this.getState(sync);
    if (isEqual(order.orderType, OrderTypeAccount.NONE)) {
      throw new BumpinInvalidParameter(
        "Order type should not be NONE (when placing order)"
      );
    }

    if (isEqual(order.orderSide, OrderSideAccount.NONE)) {
      throw new BumpinInvalidParameter(
        "Order side should not be NONE (when placing order)"
      );
    }

    if (
      order.size == 0 &&
      isEqual(order.positionSide, PositionSideAccount.DECREASE)
    ) {
      throw new BumpinInvalidParameter(
        "Order size should not be zero (when placing order with position side decrease)"
      );
    }

    if (
      isEqual(order.orderType, OrderTypeAccount.LIMIT) &&
      isEqual(order.positionSide, PositionSideAccount.DECREASE)
    ) {
      throw new BumpinInvalidParameter(
        "Decrease position does not support limit order"
      );
    }

    if (
      isEqual(order.orderType, OrderTypeAccount.STOP) &&
      (isEqual(order.stopType, StopTypeAccount.NONE) ||
        order.triggerPrice == 0)
    ) {
      throw new BumpinInvalidParameter(
        "Stop order should have stop type(not none) and trigger price(>0)"
      );
    }

    if (isEqual(order.positionSide, PositionSideAccount.INCREASE)) {
      if (order.orderMargin == 0) {
        throw new BumpinInvalidParameter(
          "Order margin should not be zero (when placing order with Increase position side)"
        );
      }
    }

    if (
      order.isPortfolioMargin &&
      (order.orderMargin == 0 ||
        order.orderMargin < state.minimumOrderMarginUsd.toNumber())
    ) {
      throw new BumpinInvalidParameter(
        "Order margin should be greater than minimum order margin: " +
          state.minimumOrderMarginUsd.toString()
      );
    }

    if (
        (order.orderMargin * tradeTokenPrice) < state.minimumOrderMarginUsd.toNumber()
    ) {
      throw new BumpinInvalidParameter(
        "Order margin should be greater than minimum order margin: " +
          state.minimumOrderMarginUsd.toString()
      );
    }

    if (
      order.leverage > market.config.maximumLeverage ||
      order.leverage < market.config.minimumLeverage
    ) {
      throw new BumpinInvalidParameter(
        "Leverage should be between " +
          market.config.minimumLeverage +
          " and " +
          market.config.maximumLeverage
      );
    }
  }

  public async getUserAccountNetValue(
    user: User,
    tradeTokens: TradeToken[],
    markets: Market[],
    pools: Pool[]
  ): Promise<AccountNetValue> {
    let accountNetValue = {
      accountNetValue: BigNumber(0),
      totalMM: BigNumber(0),
    };
    let balanceOfUserTradeTokens =
      await BumpinTokenUtils.getUserTradeTokenBalance(
        this.tradeTokenComponent,
        user,
        tradeTokens
      );
    let balanceOfUserPositions = await BumpinPositionUtils.getUserPositionValue(
      this.tradeTokenComponent,
      user,
      tradeTokens,
      markets,
      pools
    );
    accountNetValue.accountNetValue = balanceOfUserTradeTokens.tokenNetValue
      .plus(balanceOfUserPositions.initialMarginUsd)
      .plus(user.hold)
      .minus(balanceOfUserTradeTokens.tokenUsedValue)
      .plus(
        balanceOfUserPositions.positionUnPnl.gt(BigNumber(0))
          ? BigNumber(0)
          : balanceOfUserPositions.positionUnPnl
      )
      .minus(balanceOfUserPositions.positionFee);
    accountNetValue.totalMM = balanceOfUserPositions.mmUsd;
    return accountNetValue;
  }

  public async getUserAvailableValue(
    user: User,
    tradeTokens: TradeToken[],
    markets: Market[],
    pools: Pool[]
  ): Promise<BigNumber> {
    let balanceOfUserTradeTokens =
      await BumpinTokenUtils.getUserTradeTokenBalance(
        this.tradeTokenComponent,
        user,
        tradeTokens
      );
    let balanceOfUserPositions = await BumpinPositionUtils.getUserPositionValue(
      this.tradeTokenComponent,
      user,
      tradeTokens,
      markets,
      pools
    );
    return balanceOfUserTradeTokens.tokenNetValue
      .plus(balanceOfUserPositions.initialMarginUsd)
      .plus(user.hold)
      .minus(balanceOfUserTradeTokens.tokenUsedValue)
      .plus(
        balanceOfUserPositions.positionUnPnl.gt(BigNumber(0))
          ? BigNumber(0)
          : balanceOfUserPositions.positionUnPnl
      )
      .minus(
        balanceOfUserTradeTokens.tokenBorrowingValue.plus(
          balanceOfUserPositions.initialMarginUsdFromPortfolio
        )
      );
  }

  public getUserRemainingAccounts(
    user: User,
    allTradeTokens: TradeToken[],
    isWritable: boolean = false
  ): Array<AccountMeta> {
    let remainingAccounts: Array<AccountMeta> = [];
    for (let token of user.tokens) {
      if (isEqual(token.userTokenStatus, UserTokenStatusAccount.USING)) {
        remainingAccounts.push({
          pubkey: token.tokenMintKey,
          isWritable,
          isSigner: false,
        });
        let target = BumpinTokenUtils.getTradeTokenByMintPublicKey(
          token.tokenMintKey,
          allTradeTokens
        );
        remainingAccounts.push({
          pubkey: target.oracleKey,
          isWritable,
          isSigner: false,
        });
        let pda = BumpinUtils.getTradeTokenPda(this.program, target.index)[0];
        remainingAccounts.push({
          pubkey: pda,
          isWritable,
          isSigner: false,
        });
      }
    }

    return remainingAccounts;
  }

  async checkStakeAmountFulfilRequirements(
    amount: BN,
    tradeToken: TradeToken,
    pool: Pool
  ): Promise<BigNumber> {
    const price = this.tradeTokenComponent.getTradeTokenPricesByOracleKey(
      tradeToken.mintKey,
      1
    )[0].price;
    if (!price) {
      throw new BumpinInvalidParameter("Price data not found");
    }
    let value = BumpinUtils.toUsd(amount, price, tradeToken.decimals);
    if (value < pool.config.minimumStakeAmount) {
      throw new BumpinValueInsufficient(pool.config.minimumStakeAmount, value);
    }
    return value;
  }

  async checkStakeWalletAmountSufficient(
    amount: BN,
    wallet: PublicKey,
    tradeToken: TradeToken
  ): Promise<void> {
    let balance = await BumpinTokenUtils.getTokenBalanceFromWallet(
      this.program.provider.connection,
      wallet,
      tradeToken.mintKey
    );
    let balanceAmount = new BN(balance.toString());
    if (balanceAmount.lt(amount)) {
      throw new BumpinValueInsufficient(
        amount.toBigNumber(),
        balanceAmount.toBigNumber()
      );
    }
  }

  public async findUsingStake(poolKey: PublicKey, sync: boolean) {
    let user = await this.getUser(sync);
    return user.stakes.find(
      (value, index, obj) =>
        isEqual(value.userStakeStatus, UserStakeStatusAccount.USING) &&
        value.poolKey.equals(poolKey)
    );
  }

  public async getUserTokenAccountByMint(mint: PublicKey) {
    const tokenAccount =
      await this.program.provider.connection.getTokenAccountsByOwner(
        this.publicKey,
        { mint: mint }
      );
    return tokenAccount.value.map((accountInfo: any) => {
      const accountData = AccountLayout.decode(accountInfo.account.data);
      const mint = new PublicKey(accountData.mint).toBase58();
      const amount = accountData.amount; // Assuming the token has 9 decimal places
      return {
        pubkey: accountInfo.pubkey.toBase58(),
        mint,
        amount,
      };
    });
  }

  public async getUser(sync: boolean = false): Promise<User> {
    let userWithSlot = await this.getUserWithSlot(sync);
    return userWithSlot.data;
  }

  public async getUserWithSlot(
    sync: boolean = false
  ): Promise<DataAndSlot<User>> {
    if (
      !this.userAccountSubscriber ||
      !this.userAccountSubscriber.isSubscribed
    ) {
      throw new BumpinSubscriptionFailed("User");
    }
    if (sync) {
      await this.userAccountSubscriber.fetch();
    }
    let userAccount = this.userAccountSubscriber.getAccountAndSlot();
    if (!userAccount) {
      throw new BumpinAccountNotFound("User");
    }
    return userAccount;
  }
}
