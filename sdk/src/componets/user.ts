import {
    AccountMeta,
    AddressLookupTableAccount,
    ConfirmOptions,
    PublicKey,
} from '@solana/web3.js';
import {
    AccountNetValue,
    InnerPlaceOrderParams,
    OrderSideAccount,
    OrderTypeAccount,
    PlaceOrderParams,
    PositionSideAccount,
    StopTypeAccount,
} from '../typedef';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { BN, Program, Wallet } from '@coral-xyz/anchor';
import { BumpinUtils } from '../utils/utils';
import { BumpinTrade } from '../types/bumpin_trade';
// @ts-ignore
import { isEqual } from 'lodash';
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { PollingUserAccountSubscriber } from '../account/pollingUserAccountSubscriber';
import {
    BumpinAccountNotFound,
    BumpinInvalidParameter,
    BumpinSubscriptionFailed,
    BumpinSupplyInsufficient,
    BumpinTokenAccountUnexpected,
    BumpinValueInsufficient,
} from '../errors';
import { DataAndSlot } from '../account/types';
import { BumpinTokenUtils } from '../utils/token';
import { BumpinPositionUtils } from '../utils/position';
import { BumpinPoolUtils } from '../utils/pool';
import { Account, AccountLayout } from '@solana/spl-token';
import BigNumber from 'bignumber.js';
import { BumpinMarketUtils } from '../utils/market';
import { C } from '../consts';
import {
    Market,
    OrderSide,
    OrderType,
    Pool,
    PositionSide,
    PositionStatus,
    StopType,
    TradeToken,
    User,
    UserStakeStatus,
    UserTokenStatus,
} from '../beans/beans';
import { TradeTokenComponent } from './tradeToken';
import { PoolComponent } from './pool';
import { BumpinClientConfig } from '../bumpinClientConfig';
import { MarketComponent } from './market';
import { RewardsComponent } from './rewards';

export class UserComponent extends Component {
    publicKey: PublicKey;
    program: Program<BumpinTrade>;
    userAccountSubscriber: PollingUserAccountSubscriber;
    tradeTokenComponent: TradeTokenComponent;
    poolComponent: PoolComponent;
    marketComponent: MarketComponent;
    rewardComponent: RewardsComponent;

    constructor(
        config: BumpinClientConfig,
        defaultConfirmOptions: ConfirmOptions,
        publicKey: PublicKey,
        bulkAccountLoader: BulkAccountLoader,
        stateSubscriber: PollingStateAccountSubscriber,
        tradeTokenComponent: TradeTokenComponent,
        marketComponent: MarketComponent,
        poolComponent: PoolComponent,
        rewardsComponent: RewardsComponent,
        program: Program<BumpinTrade>,
        wallet?: Wallet,
        essentialAccounts: AddressLookupTableAccount[] = [],
    ) {
        super(
            config,
            defaultConfirmOptions,
            stateSubscriber,
            program,
            wallet,
            essentialAccounts,
        );
        this.publicKey = publicKey;
        this.program = program;
        this.tradeTokenComponent = tradeTokenComponent;
        this.marketComponent = marketComponent;
        this.poolComponent = poolComponent;
        this.rewardComponent = rewardsComponent;
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [
            Buffer.from('user'),
            this.publicKey.toBuffer(),
        ]);
        this.userAccountSubscriber = new PollingUserAccountSubscriber(
            this.program,
            pda,
            bulkAccountLoader,
            tradeTokenComponent,
            poolComponent,
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
        sync: boolean = false,
    ): Promise<void> {
        let user = await this.getUser(sync);
        let amount = BumpinUtils.size2Amount(
            new BigNumber(size),
            tradeToken.decimals,
        );
        let stake_value = await this.checkStakeAmountFulfilRequirements(
            size,
            tradeToken,
            pool,
        );
        let availableValue = await this.getUserAvailableValue(
            user,
            allTradeTokens,
            allMarkets,
            pools,
        );
        if (!availableValue.gt(stake_value)) {
            throw new BumpinValueInsufficient(
                amount.toBigNumber(),
                availableValue,
            );
        }

        let remainingAccounts = this.getUserTradeTokenRemainingAccounts(
            await this.getUser(),
            allTradeTokens,
        );
        let markets = BumpinMarketUtils.getMarketsByPoolKey(
            pool.key,
            allMarkets,
        );
        for (let market of markets.values()) {
            remainingAccounts.push({
                pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
                isWritable: true,
                isSigner: false,
            });
        }

        const ix = await this.program.methods
            .portfolioStake(pool.index, tradeToken.index, amount)
            .accounts({
                authority: this.publicKey,
                bumpSigner: (await this.getState()).bumpSigner,
            })
            .remainingAccounts(BumpinUtils.removeDuplicateAccounts(remainingAccounts.concat(await this.essentialRemainingAccounts())))
            .signers([])
            .instruction();
        await this.sendAndConfirm([ix]);
    }

    public async walletStake(
        size: number,
        tradeToken: TradeToken,
        allTradeTokens: TradeToken[],
        wallet: PublicKey,
        pool: Pool,
        allMarkets: Market[],
        sync: boolean = false,
    ): Promise<void> {
        // let user = await this.getUser(sync);
        let amount = BumpinUtils.size2Amount(
            new BigNumber(size),
            tradeToken.decimals,
        );
        await this.checkStakeAmountFulfilRequirements(size, tradeToken, pool);
        await this.checkStakeWalletAmountSufficient(amount, wallet, tradeToken);
        let tokenAccount =
            await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
                this.program.provider.connection,
                wallet,
                tradeToken.mintKey,
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
        let pda = BumpinUtils.getTradeTokenPda(
            this.program,
            tradeToken.index,
        )[0];
        remainingAccounts.push({
            pubkey: pda,
            isWritable: false,
            isSigner: false,
        });

        let markets = BumpinMarketUtils.getMarketsByPoolKey(
            pool.key,
            allMarkets,
        );
        for (let market of markets) {
            remainingAccounts.push({
                pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
                isWritable: true,
                isSigner: false,
            });
        }

        const ix = await this.program.methods
            .walletStake(pool.index, amount)
            .accounts({
                authority: wallet,
                userTokenAccount: tokenAccount.address,
            })
            .remainingAccounts(BumpinUtils.removeDuplicateAccounts(remainingAccounts.concat(await this.essentialRemainingAccounts())))
            .signers([])
            .instruction();
        await this.sendAndConfirm([ix]);
    }

    public async unStake(
        portfolio: boolean,
        share: BigNumber,
        tradeToken: TradeToken,
        wallet: PublicKey,
        pool: Pool,
        allMarkets: Market[],
    ): Promise<void> {
        let userStake = await this.findUsingStake(pool.key, false);
        if (!userStake) {
            throw new BumpinInvalidParameter('User stake not found');
        }
        if (share.gt(userStake.stakedShare)) {
            throw new BumpinValueInsufficient(userStake.stakedShare, share);
        }
        if (pool.totalSupply.isZero()) {
            throw new BumpinSupplyInsufficient(share, BigNumber(0));
        }

        let param = {
            share: BumpinUtils.size2Amount(share, tradeToken.decimals),
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
        let pda = BumpinUtils.getTradeTokenPda(
            this.program,
            tradeToken.index,
        )[0];
        remainingAccounts.push({
            pubkey: pda,
            isWritable: false,
            isSigner: false,
        });

        let markets = BumpinMarketUtils.getMarketsByPoolKey(
            pool.key,
            allMarkets,
        );
        for (let market of markets) {
            remainingAccounts.push({
                pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
                isWritable: true,
                isSigner: false,
            });
        }

        if (portfolio) {
            BumpinUtils.prettyPrintParam(param);
            const ix = await this.program.methods
                .portfolioUnStake(param)
                .accounts({
                    authority: wallet,
                })
                .remainingAccounts(remainingAccounts)
                .signers([])
                .instruction();
            await this.sendAndConfirm([ix]);
        } else {
            let tokenAccount =
                await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
                    this.program.provider.connection,
                    wallet,
                    tradeToken.mintKey,
                );
            BumpinUtils.prettyPrintParam(param);
            const ix = await this.program.methods
                .walletUnStake(param)
                .accounts({
                    authority: wallet,
                    userTokenAccount: tokenAccount.address,
                    bumpSigner: (await this.getState()).bumpSigner,
                })
                .remainingAccounts(BumpinUtils.removeDuplicateAccounts(remainingAccounts.concat(await this.essentialRemainingAccounts())))
                .signers([])
                .instruction();
            await this.sendAndConfirm([ix]);
        }
    }

    public async deposit(
        userTokenAccount: PublicKey,
        mintPublicKey: PublicKey,
        size: number,
    ) {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mintPublicKey,
            await this.tradeTokenComponent.getTradeTokens(),
        );
        let amount = BumpinUtils.size2Amount(
            new BigNumber(size),
            targetTradeToken.decimals,
        );

        console.log('Deposit:\namount: ' + amount.toString());
        BumpinUtils.prettyPrintParam(targetTradeToken);
        const ix = await this.program.methods
            .deposit(targetTradeToken.index, amount)
            .accounts({
                userTokenAccount,
            })
            .signers([])
            .instruction();
        await this.sendAndConfirm([ix]);
    }

    public async withdraw(
        userTokenAccount: PublicKey,
        mintPublicKey: PublicKey,
        size: number,
        sync: boolean = false,
    ) {
        const me = await this.getUser(sync);
        const tradeTokens = await this.tradeTokenComponent.getTradeTokens(sync);
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mintPublicKey,
            tradeTokens,
        );
        let amount = BumpinUtils.size2Amount(
            new BigNumber(size),
            targetTradeToken.decimals,
        );
        const marketRemainingAccounts = (
            await this.marketComponent.getMarkets(sync)
        ).map((e) => {
            return {
                pubkey: BumpinUtils.getMarketPda(this.program, e.index)[0],
                isWritable: false,
                isSigner: false,
            };
        });

        console.log('Withdraw:\namount: ' + amount.toString());
        BumpinUtils.prettyPrintParam(targetTradeToken);
        const ix = await this.program.methods
            .withdraw(targetTradeToken.index, amount)
            .accounts({
                userTokenAccount,
                authority: this.publicKey,
                bumpSigner: (await this.getState()).bumpSigner,
            })
            .remainingAccounts(
                marketRemainingAccounts.concat(
                    this.getUserTradeTokenRemainingAccounts(
                        me,
                        tradeTokens,
                        true,
                    ),
                ),
            )
            .signers([])
            .instruction();
        await this.sendAndConfirm([ix]);
    }

    public async placePerpOrder(
        symbol: string,
        marketIndex: number,
        param: PlaceOrderParams,
        wallet: PublicKey,
        pools: Pool[],
        markets: Market[],
        tradeTokens: TradeToken[],
    ) {
        const user = await this.getUser();
        const pool = BumpinPoolUtils.getPoolByMintPublicKey(
            markets[marketIndex].poolMintKey,
            pools,
        );
        const stablePool = BumpinPoolUtils.getPoolByMintPublicKey(
            markets[marketIndex].stablePoolMintKey,
            pools,
        );
        const tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            markets[marketIndex].poolMintKey,
            tradeTokens,
        );
        const stableTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            markets[marketIndex].stablePoolMintKey,
            tradeTokens,
        );

        let userTokenAccount = (
            await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
                this.program.provider.connection,
                wallet,
                tradeToken.mintKey,
            )
        ).address;

        if (
            (isEqual(param.positionSide, PositionSide.DECREASE) &&
                isEqual(param.orderSide, OrderSide.LONG)) ||
            (isEqual(param.positionSide, PositionSide.INCREASE) &&
                isEqual(param.orderSide, OrderSide.SHORT))
        ) {
            // When the order side is short, the userTokenAccount is the stable token.
            userTokenAccount = (
                await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(
                    this.program.provider.connection,
                    wallet,
                    markets[marketIndex].stablePoolMintKey,
                )
            ).address;
        } // When trading position by position (Isolated position), userTokenAccount is determined based on the order direction.
        let uta = userTokenAccount;
        if (!param.isPortfolioMargin) {
            let tokenAccount: Account =
                await BumpinTokenUtils.getTokenAccountFromWalletAndKey(
                    this.program.provider.connection,
                    wallet,
                    userTokenAccount,
                );
            if (isEqual(param.positionSide, PositionSide.INCREASE)) {
                if (isEqual(param.orderSide, OrderSide.LONG)) {
                    if (!tokenAccount.mint.equals(pool.mintKey)) {
                        throw new BumpinTokenAccountUnexpected(
                            'Pool mint key: ' + pool.mintKey.toString(),
                            'Token account mint key: ' +
                                tokenAccount.mint.toString(),
                        );
                    }
                } else {
                    if (!tokenAccount.mint.equals(stablePool.mintKey)) {
                        throw new BumpinTokenAccountUnexpected(
                            'Stable Pool mint key: ' +
                                stablePool.mintKey.toString(),
                            'Token account mint key: ' +
                                tokenAccount.mint.toString(),
                        );
                    }
                }
            } else {
                if (isEqual(param.orderSide, OrderSide.LONG)) {
                    if (!tokenAccount.mint.equals(stablePool.mintKey)) {
                        throw new BumpinTokenAccountUnexpected(
                            'Stable Pool mint key: ' +
                                stablePool.mintKey.toString(),
                            'Token account mint key: ' +
                                tokenAccount.mint.toString(),
                        );
                    }
                } else {
                    if (!tokenAccount.mint.equals(pool.mintKey)) {
                        throw new BumpinTokenAccountUnexpected(
                            'Pool mint key: ' + pool.mintKey.toString(),
                            'Token account mint key: ' +
                                tokenAccount.mint.toString(),
                        );
                    }
                }
            }
            uta = tokenAccount.address;
        }

        let remainingAccounts = param.isPortfolioMargin
            ? this.buildPortfolioRemainAccount(
                  marketIndex,
                  user,
                  tradeTokens,
                  markets,
                  pools,
                  param,
              )
            : this.buildIsolateRemainAccount(
                  marketIndex,
                  tradeTokens,
                  markets,
                  pools,
                  param,
              );

        let indexPrice =
            this.tradeTokenComponent.getTradeTokenPricesByOracleKey(
                markets[marketIndex].indexMintOracle,
                0,
            )[0];
        if (!indexPrice.price) {
            throw new BumpinInvalidParameter(
                'Price not found(undefined) for mint: ' +
                    pool.mintKey.toString(),
            );
        }

        let order: InnerPlaceOrderParams = {
            symbol: BumpinUtils.encodeString(symbol),
            placeTime: new BN(Date.now()),
            isPortfolioMargin: param.isPortfolioMargin,
            isNativeToken: false,
            orderSide: OrderSideAccount.from(param.orderSide),
            positionSide: PositionSideAccount.from(param.positionSide),
            orderType: OrderTypeAccount.from(param.orderType),
            stopType: StopTypeAccount.from(param.stopType),
            size: BumpinUtils.number2Precision(
                param.size,
                C.USD_EXPONENT_NUMBER,
            ),
            orderMargin: !param.isPortfolioMargin
                ? BumpinUtils.number2Precision(
                      param.orderMargin,
                      isEqual(param.positionSide, PositionSide.INCREASE)
                          ? isEqual(param.orderSide, OrderSide.LONG)
                              ? tradeToken.decimals
                              : stableTradeToken.decimals
                          : isEqual(param.orderSide, OrderSide.LONG)
                          ? stableTradeToken.decimals
                          : tradeToken.decimals,
                  )
                : BumpinUtils.number2Precision(
                      param.orderMargin * indexPrice.price,
                      C.USD_EXPONENT_NUMBER,
                  ),
            leverage: param.leverage * C.RATE_MULTIPLIER,
            triggerPrice: BumpinUtils.number2Precision(
                param.triggerPrice,
                C.PRICE_EXPONENT_NUMBER,
            ),
            acceptablePrice: BumpinUtils.number2Precision(
                param.acceptablePrice,
                C.PRICE_EXPONENT_NUMBER,
            ),
        };
        let accountMetas =
            BumpinUtils.removeDuplicateAccounts((await this.essentialRemainingAccounts()).concat(remainingAccounts));

        await this.placePerpOrderValidation(
            order,
            indexPrice.price,
            markets[marketIndex],
        );

        BumpinUtils.prettyPrintParam(order);
        const ix = await this.program.methods
            .placeOrder(order)
            .accounts({
                userTokenAccount: uta,
                authority: wallet,
                bumpSigner: (await this.getState()).bumpSigner,
            })
            .remainingAccounts(accountMetas)
            .signers([])
            .instruction();
        await this.sendAndConfirm([ix]);
    }

    //TODO: recheck this conditions
    async placePerpOrderValidation(
        order: InnerPlaceOrderParams,
        tradeTokenPrice: number,
        market: Market,
        sync: boolean = false,
    ) {
        let state = await this.getState(sync);
        if (isEqual(order.orderType, OrderType.NONE)) {
            throw new BumpinInvalidParameter(
                'Order type should not be NONE (when placing order)',
            );
        }

        if (isEqual(order.orderSide, OrderSide.NONE)) {
            throw new BumpinInvalidParameter(
                'Order side should not be NONE (when placing order)',
            );
        }

        if (
            order.size.isZero() &&
            isEqual(order.positionSide, PositionSide.DECREASE)
        ) {
            throw new BumpinInvalidParameter(
                'Order size should not be zero (when placing order with position side decrease)',
            );
        }

        if (
            isEqual(order.orderType, OrderType.STOP) &&
            (isEqual(order.stopType, StopType.NONE) ||
                order.triggerPrice.isZero())
        ) {
            throw new BumpinInvalidParameter(
                'Stop order should have stop type(not none) and trigger price(>0)',
            );
        }

        if (isEqual(order.positionSide, PositionSide.INCREASE)) {
            if (order.orderMargin.isZero()) {
                throw new BumpinInvalidParameter(
                    'Order margin should not be zero (when placing order with Increase position side)',
                );
            }
        }

        //TODO: Do better
        if (
            order.leverage / 100000 > market.config.maximumLeverage ||
            order.leverage / 100000 < market.config.minimumLeverage
        ) {
            throw new BumpinInvalidParameter(
                'Leverage should be between ' +
                    market.config.minimumLeverage +
                    ' and ' +
                    market.config.maximumLeverage,
            );
        }
    }

    public async getUserAccountNetValue(
        user: User,
        tradeTokens: TradeToken[],
        markets: Market[],
        pools: Pool[],
    ): Promise<AccountNetValue> {
        let accountNetValue = {
            accountNetValue: BigNumber(0),
            totalMM: BigNumber(0),
        };
        let balanceOfUserTradeTokens =
            await BumpinTokenUtils.getUserTradeTokenBalance(
                this.tradeTokenComponent,
                user,
                tradeTokens,
            );
        let balanceOfUserPositions =
            await BumpinPositionUtils.getUserPositionValue(
                this.tradeTokenComponent,
                user,
                tradeTokens,
                markets,
                pools,
            );
        accountNetValue.accountNetValue = balanceOfUserTradeTokens.tokenNetValue
            .plus(balanceOfUserPositions.initialMarginUsd)
            .plus(user.hold)
            .minus(balanceOfUserTradeTokens.tokenUsedValue)
            .plus(
                balanceOfUserPositions.positionUnPnl.gt(BigNumber(0))
                    ? BigNumber(0)
                    : balanceOfUserPositions.positionUnPnl,
            )
            .minus(balanceOfUserPositions.positionFee);
        accountNetValue.totalMM = balanceOfUserPositions.mmUsd;
        return accountNetValue;
    }

    public async getUserAvailableValue(
        user: User,
        tradeTokens: TradeToken[],
        markets: Market[],
        pools: Pool[],
    ): Promise<BigNumber> {
        let balanceOfUserTradeTokens =
            await BumpinTokenUtils.getUserTradeTokenBalance(
                this.tradeTokenComponent,
                user,
                tradeTokens,
            );
        let balanceOfUserPositions =
            await BumpinPositionUtils.getUserPositionValue(
                this.tradeTokenComponent,
                user,
                tradeTokens,
                markets,
                pools,
            );
        return balanceOfUserTradeTokens.tokenNetValue
            .plus(balanceOfUserPositions.initialMarginUsd)
            .plus(user.hold)
            .minus(balanceOfUserTradeTokens.tokenUsedValue)
            .plus(
                balanceOfUserPositions.positionUnPnl.gt(BigNumber(0))
                    ? BigNumber(0)
                    : balanceOfUserPositions.positionUnPnl,
            )
            .minus(
                balanceOfUserTradeTokens.tokenBorrowingValue.plus(
                    balanceOfUserPositions.initialMarginUsdFromPortfolio,
                ),
            );
    }

    public getUserTradeTokenRemainingAccounts(
        user: User,
        allTradeTokens: TradeToken[],
        isWritable: boolean = false,
    ): Array<AccountMeta> {
        let remainingAccounts: Array<AccountMeta> = [];
        for (let token of user.tokens) {
            if (isEqual(token.userTokenStatus, UserTokenStatus.USING)) {
                remainingAccounts.push({
                    pubkey: token.tokenMintKey,
                    isWritable,
                    isSigner: false,
                });
                let target = BumpinTokenUtils.getTradeTokenByMintPublicKey(
                    token.tokenMintKey,
                    allTradeTokens,
                );
                remainingAccounts.push({
                    pubkey: target.oracleKey,
                    isWritable,
                    isSigner: false,
                });
                let pda = BumpinUtils.getTradeTokenPda(
                    this.program,
                    target.index,
                )[0];
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
        size: number,
        tradeToken: TradeToken,
        pool: Pool,
    ): Promise<BigNumber> {
        const price = (
            await this.tradeTokenComponent.getTradeTokenPricesByMintKey(
                tradeToken.mintKey,
            )
        ).price;
        if (!price) {
            throw new BumpinInvalidParameter('Price data not found');
        }
        let value = price * size;
        if (pool.config.minimumStakeAmount.gt(value)) {
            throw new BumpinValueInsufficient(
                pool.config.minimumStakeAmount,
                BigNumber(value),
            );
        }
        return BigNumber(value);
    }

    async checkStakeWalletAmountSufficient(
        amount: BN,
        wallet: PublicKey,
        tradeToken: TradeToken,
    ): Promise<void> {
        let balance = await BumpinTokenUtils.getTokenBalanceFromWallet(
            this.program.provider.connection,
            wallet,
            tradeToken.mintKey,
        );
        let balanceAmount = new BN(balance.toString());
        if (balanceAmount.lt(amount)) {
            throw new BumpinValueInsufficient(
                amount.toBigNumber(),
                balanceAmount.toBigNumber(),
            );
        }
    }

    public async findUsingStake(poolKey: PublicKey, sync: boolean) {
        let user = await this.getUser(sync);
        return user.stakes.find(
            (value, index, obj) =>
                isEqual(value.userStakeStatus, UserStakeStatus.USING) &&
                value.poolKey.equals(poolKey),
        );
    }

    public async getUserTokenAccountByMint(mint: PublicKey) {
        const tokenAccount =
            await this.program.provider.connection.getTokenAccountsByOwner(
                this.publicKey,
                { mint: mint },
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
        sync: boolean = false,
    ): Promise<DataAndSlot<User>> {
        if (
            !this.userAccountSubscriber ||
            !this.userAccountSubscriber.isSubscribed
        ) {
            throw new BumpinSubscriptionFailed('User');
        }
        if (sync) {
            await this.userAccountSubscriber.fetch();
        }
        let userAccount = this.userAccountSubscriber.getAccountAndSlot();
        if (!userAccount) {
            throw new BumpinAccountNotFound('User');
        }
        return userAccount;
    }

    private buildPortfolioRemainAccount(
        marketIndex: number,
        user: User,
        tradeTokens: TradeToken[],
        markets: Market[],
        pools: Pool[],
        param: PlaceOrderParams,
    ): Array<AccountMeta> {
        let isActLong;
        let isIncrease;
        if (isEqual(param.positionSide, PositionSide.INCREASE)) {
            isActLong = isEqual(param.orderSide, OrderSide.LONG);
            isIncrease = true;
        } else {
            isActLong = isEqual(param.orderSide, OrderSide.SHORT);
            isIncrease = false;
        }
        //trade_tokens
        let accounts = this.getUserTradeTokenRemainingAccounts(
            user,
            tradeTokens,
            true,
        );
        let mainMarket = markets[marketIndex];
        let baseTokenPool = BumpinPoolUtils.getPoolByPublicKey(
            mainMarket.poolKey,
            pools,
        );
        let stablePool = BumpinPoolUtils.getPoolByPublicKey(
            mainMarket.stablePoolKey,
            pools,
        );
        accounts.push({
            pubkey: BumpinUtils.getMarketPda(this.program, mainMarket.index)[0],
            isWritable: true,
            isSigner: false,
        });
        accounts.push({
            pubkey: mainMarket.indexMintOracle,
            isWritable: true,
            isSigner: false,
        });
        let baseTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mainMarket.poolMintKey,
            tradeTokens,
        );
        accounts.push({
            pubkey: mainMarket.poolMintKey,
            isWritable: true,
            isSigner: false,
        });
        accounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(
                this.program,
                baseTradeToken.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });
        accounts.push({
            pubkey: baseTradeToken.oracleKey,
            isWritable: true,
            isSigner: false,
        });
        let stableTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mainMarket.stablePoolMintKey,
            tradeTokens,
        );
        accounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(
                this.program,
                stableTradeToken.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });
        accounts.push({
            pubkey: mainMarket.stablePoolMintKey,
            isWritable: true,
            isSigner: false,
        });
        accounts.push({
            pubkey: stableTradeToken.oracleKey,
            isWritable: true,
            isSigner: false,
        });

        if (!isIncrease) {
            console.log(isActLong);
            if (isActLong) {
                accounts.push({
                    pubkey: baseTokenPool.poolVaultKey,
                    isWritable: true,
                    isSigner: false,
                });
                accounts.push({
                    pubkey: baseTradeToken.vaultKey,
                    isWritable: true,
                    isSigner: false,
                });
            } else {
                accounts.push({
                    pubkey: stablePool.poolVaultKey,
                    isWritable: true,
                    isSigner: false,
                });
                accounts.push({
                    pubkey: stableTradeToken.vaultKey,
                    isWritable: true,
                    isSigner: false,
                });
            }
        }

        user.positions.forEach((position) => {
            if (isEqual(position.status, PositionStatus.USING)) {
                markets.forEach((market) => {
                    if (market.symbol === position.symbol) {
                        accounts.push({
                            pubkey: BumpinUtils.getMarketPda(
                                this.program,
                                market.index,
                            )[0],
                            isWritable: true,
                            isSigner: false,
                        });
                        accounts.push({
                            pubkey: market.indexMintOracle,
                            isWritable: true,
                            isSigner: false,
                        });
                    }
                });
            }
        });

        markets.forEach((market) => {
            if (market.poolKey.equals(baseTokenPool.key)) {
                accounts.push({
                    pubkey: BumpinUtils.getMarketPda(
                        this.program,
                        market.index,
                    )[0],
                    isWritable: true,
                    isSigner: false,
                });
                accounts.push({
                    pubkey: market.indexMintOracle,
                    isWritable: true,
                    isSigner: false,
                });
            }
        });
        accounts.push({
            pubkey: BumpinUtils.getPoolPda(
                this.program,
                baseTokenPool.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });

        accounts.push({
            pubkey: BumpinUtils.getPoolPda(this.program, stablePool.index)[0],
            isWritable: true,
            isSigner: false,
        });
        if (isEqual(param.positionSide, PositionSide.DECREASE)) {
            if (isEqual(param.orderSide, OrderSide.LONG)) {
                accounts.push({
                    pubkey: stablePool.poolVaultKey,
                    isWritable: true,
                    isSigner: false,
                });
            } else {
                accounts.push({
                    pubkey: baseTokenPool.poolVaultKey,
                    isWritable: true,
                    isSigner: false,
                });
            }
        }
        return accounts;
    }
    private buildIsolateRemainAccount(
        marketIndex: number,
        tradeTokens: TradeToken[],
        markets: Market[],
        pools: Pool[],
        param: PlaceOrderParams,
    ): Array<AccountMeta> {
        let isActLong;
        if (isEqual(param.positionSide, PositionSide.INCREASE)) {
            isActLong = isEqual(param.orderSide, OrderSide.LONG);
        } else {
            isActLong = isEqual(param.orderSide, OrderSide.SHORT);
        }
        let remainingAccounts: Array<AccountMeta> = [];
        //trade token
        let mainMarket = markets[marketIndex];
        let baseTokenPool = BumpinPoolUtils.getPoolByPublicKey(
            mainMarket.poolKey,
            pools,
        );
        let stablePool = BumpinPoolUtils.getPoolByPublicKey(
            mainMarket.stablePoolKey,
            pools,
        );
        remainingAccounts.push({
            pubkey: BumpinUtils.getMarketPda(this.program, mainMarket.index)[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: mainMarket.indexMintOracle,
            isWritable: true,
            isSigner: false,
        });
        let baseTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mainMarket.poolMintKey,
            tradeTokens,
        );
        let stableTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mainMarket.stablePoolMintKey,
            tradeTokens,
        );
        remainingAccounts.push({
            pubkey: BumpinUtils.getPoolPda(
                this.program,
                baseTokenPool.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: BumpinUtils.getPoolPda(this.program, stablePool.index)[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(
                this.program,
                baseTradeToken.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: baseTradeToken.oracleKey,
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(
                this.program,
                stableTradeToken.index,
            )[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: stableTradeToken.oracleKey,
            isWritable: true,
            isSigner: false,
        });
        if (isActLong) {
            remainingAccounts.push({
                pubkey: baseTokenPool.poolVaultKey,
                isWritable: true,
                isSigner: false,
            });
            remainingAccounts.push({
                pubkey: baseTradeToken.vaultKey,
                isWritable: true,
                isSigner: false,
            });
        } else {
            remainingAccounts.push({
                pubkey: stablePool.poolVaultKey,
                isWritable: true,
                isSigner: false,
            });
            remainingAccounts.push({
                pubkey: stableTradeToken.vaultKey,
                isWritable: true,
                isSigner: false,
            });
        }
        remainingAccounts.forEach((value) => {
            console.log(value.pubkey.toString());
        });
        return remainingAccounts;
    }

    private async essentialRemainingAccounts(){
        const pools = await this.poolComponent.getPools();
        const tradeTokens = await this.tradeTokenComponent.getTradeTokens();
        const markets = await this.marketComponent.getMarkets();

        let remainingAccounts: Array<AccountMeta> = [];

        //trade_tokens
        tradeTokens.forEach((tradeToken) => {
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
            let pda = BumpinUtils.getTradeTokenPda(
                this.program,
                tradeToken.index,
            )[0];
            remainingAccounts.push({
                pubkey: pda,
                isWritable: false,
                isSigner: false,
            });
        });

        //pools
        pools.forEach((pool) => {
            let poolPda = BumpinUtils.getPoolPda(this.program, pool.index)[0];
            remainingAccounts.push({
                pubkey: poolPda,
                isWritable: false,
                isSigner: false,
            });
        });

        //markets
        markets.forEach((market) => {
            let marketPda = BumpinUtils.getMarketPda(this.program, market.index)[0];
            remainingAccounts.push({
                pubkey: marketPda,
                isWritable: false,
                isSigner: false,
            });
        });

        return remainingAccounts;
    }
}
