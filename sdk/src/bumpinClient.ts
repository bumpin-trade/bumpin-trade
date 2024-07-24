import {
    AddressLookupTableAccount,
    ConfirmOptions,
    Connection,
    PublicKey,
} from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';
import idlBumpinTrade from './idl/bumpin_trade.json';
import idlPyth from './idl/pyth.json';
import { BumpinClientConfig, NetType } from './bumpinClientConfig';
import { BumpinUtils } from './utils/utils';
import { BumpinTrade } from './types/bumpin_trade';
import {
    AccountValue,
    MarketUnPnlUsd,
    MarketWithIndexTradeTokenPrices,
    PlaceOrderParams,
    PoolSummary,
    RewardsAccount,
    TokenBalance,
    UserAccount,
    UserClaimResult,
    UserClaimRewardsResult,
    UserSummary,
    UserTokenSummary,
    WalletBalance,
} from './typedef';
import {
    BumpinAccountNotFound,
    BumpinClientNotInitialized,
    BumpinInvalidParameter,
    BumpinSubscriptionFailed,
    BumpinUserNotLogin,
} from './errors';
import { PollingUserAccountSubscriber } from './account/pollingUserAccountSubscriber';
import { BulkAccountLoader } from './account/bulkAccountLoader';
import { DataAndSlot } from './account/types';
import { PollingStateAccountSubscriber } from './account/pollingStateAccountSubscriber';
import { PoolComponent } from './componets/pool';
import { Pyth } from './types/pyth';
import { UserComponent } from './componets/user';
import { TradeTokenComponent } from './componets/tradeToken';
import { MarketComponent } from './componets/market';
import { BumpinTokenUtils } from './utils/token';
import { BumpinPoolUtils } from './utils/pool';
import { BumpinMarketUtils } from './utils/market';
import { PriceData } from '@pythnetwork/client';
import BigNumber from 'bignumber.js';
import { AccountLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { RewardsComponent } from './componets/rewards';
import './types/bnExt';
import {
    Market,
    Pool,
    State,
    TradeToken,
    User,
    UserStakeStatus,
} from './beans/beans';
import { isEqual } from 'lodash';
import { BumpinPositionUtils } from './utils/position';
import { BumpinUserUtils } from './utils/user';

export class BumpinClient {
    readonly config: BumpinClientConfig;
    readonly netType: NetType;
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    isInitialized: boolean = false;
    bulkAccountLoader: BulkAccountLoader;

    programPyth: Program<Pyth> | undefined;

    // Systems subscriptions
    stateSubscriber: PollingStateAccountSubscriber;
    userAccountSubscriber: PollingUserAccountSubscriber | undefined;

    // account lookup table
    essentialAccountAltPublicKey: PublicKey | undefined;
    essentialAccounts: AddressLookupTableAccount | null = null;

    // Components
    poolComponent: PoolComponent | undefined;
    rewardComponent: RewardsComponent | undefined;
    tradeTokenComponent: TradeTokenComponent | undefined;
    marketComponent: MarketComponent | undefined;
    userComponent: UserComponent | undefined;

    constructor(config: BumpinClientConfig) {
        this.config = config;
        this.netType = config.netType;
        this.connection = new Connection(
            config.endpoint,
            config.connectionConfig,
        );
        this.wallet = config.wallet;
        let opt: ConfirmOptions = {
            skipPreflight: false,
            commitment: 'confirmed', //default commitment: confirmed
            preflightCommitment: 'confirmed',
            maxRetries: 0,
            minContextSlot: undefined,
        };
        this.provider = new anchor.AnchorProvider(
            this.connection,
            this.wallet,
            opt,
        );
        this.program = new anchor.Program(
            JSON.parse(JSON.stringify(idlBumpinTrade)),
            this.provider,
        );
        this.bulkAccountLoader = new BulkAccountLoader(
            this.connection,
            'confirmed',
            config.pollingFrequency,
        );

        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        this.stateSubscriber = new PollingStateAccountSubscriber(
            this.program,
            statePda,
            this.bulkAccountLoader,
        );

        if (
            this.netType === NetType.LOCALNET ||
            this.netType === NetType.CUSTOM
        ) {
            this.programPyth = new anchor.Program(
                JSON.parse(JSON.stringify(idlPyth)),
                this.provider,
            );
        }
    }

    public getConnection(): Connection {
        return this.connection;
    }

    public getProgram(): Program<BumpinTrade> {
        return this.program;
    }

    public hasWallet(): boolean {
        return this.wallet !== null;
    }

    public async initialize() {
        if (this.isInitialized) {
            return;
        }

        await this.stateSubscriber.subscribe();

        const state = this.stateSubscriber.getAccountAndSlot().data;
        this.essentialAccountAltPublicKey = state.essentialAccountAlt;
        this.essentialAccounts = (
            await this.connection.getAddressLookupTable(
                state.essentialAccountAlt,
            )
        ).value;

        this.tradeTokenComponent = new TradeTokenComponent(
            this.config,
            BumpinUtils.getDefaultConfirmOptions(),
            this.bulkAccountLoader,
            this.stateSubscriber,
            this.program,
        );
        await this.tradeTokenComponent.subscribe();

        this.poolComponent = new PoolComponent(
            this.config,
            BumpinUtils.getDefaultConfirmOptions(),
            this.bulkAccountLoader,
            this.stateSubscriber,
            this.tradeTokenComponent,
            this.program,
            this.wallet,
            this.essentialAccounts === null ? [] : [this.essentialAccounts],
        );
        await this.poolComponent.subscribe();

        this.marketComponent = new MarketComponent(
            this.config,
            BumpinUtils.getDefaultConfirmOptions(),
            this.bulkAccountLoader,
            this.stateSubscriber,
            this.tradeTokenComponent,
            this.program,
            this.wallet,
            this.essentialAccounts === null ? [] : [this.essentialAccounts],
        );
        const p1 = this.marketComponent.subscribe();

        this.rewardComponent = new RewardsComponent(
            this.config,
            BumpinUtils.getDefaultConfirmOptions(),
            this.bulkAccountLoader,
            this.stateSubscriber,
            this.program,
            this.wallet,
            this.essentialAccounts === null ? [] : [this.essentialAccounts],
        );
        const p2 = this.rewardComponent.subscribe();

        await Promise.all([p1, p2]);
        this.isInitialized = true;
        console.log('BumpinClient initialized');
    }

    public async login(): Promise<UserAccount> {
        this.checkInitialization();
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [
            Buffer.from('user'),
            this.wallet.publicKey.toBuffer(),
        ]);
        try {
            let me = (await this.program.account.user.fetch(
                pda,
            )) as any as UserAccount;
            if (me) {
                this.userComponent = new UserComponent(
                    this.config,
                    BumpinUtils.getDefaultConfirmOptions(),
                    this.wallet.publicKey,
                    this.bulkAccountLoader,
                    this.stateSubscriber,
                    this.tradeTokenComponent!,
                    this.poolComponent!,
                    this.program,
                    this.wallet,
                    this.essentialAccounts === null
                        ? []
                        : [this.essentialAccounts],
                );
                await this.userComponent.subscribe();
                console.log('User logged in');
            }
            return me;
        } catch (e) {
            throw new BumpinAccountNotFound(
                'User Account, pda: ' +
                    pda.toString() +
                    ' wallet: ' +
                    this.wallet.publicKey.toString(),
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
        sync: boolean = false,
    ): Promise<WalletBalance> {
        this.checkInitialization(true);

        const tradeTokens = await this.getTradeTokens(sync);

        const balance = await this.connection.getBalance(this.wallet.publicKey);
        const userTokenAccounts = await this.connection.getTokenAccountsByOwner(
            this.wallet.publicKey,
            {
                programId: TOKEN_PROGRAM_ID,
            },
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
                    'TradeToken, mint: ' + account.mint.toString(),
                );
            }
            const tradeTokenPriceData = this.getTradeTokenPrice(
                BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0],
            );
            return new TokenBalance(
                tradeToken,
                account.amount,
                tradeTokenPriceData,
            );
        });

        return new WalletBalance(recognized, balance, 9, tokenBalances);
    }

    public getTradeTokenPrice(tradeTokenKey: PublicKey): PriceData {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokenPrices(tradeTokenKey);
    }

    public async getTradeTokenPriceByMintKey(
        mintKey: PublicKey,
    ): Promise<PriceData> {
        this.checkInitialization();
        return await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
            mintKey,
        );
    }

    public async getTradeTokenPriceByOracleKey(
        oracleKey: PublicKey,
    ): Promise<PriceData> {
        this.checkInitialization();
        let res = this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
            oracleKey,
            1,
        );
        return res[0];
    }

    public async getPoolSummary(
        stashedPrice: number = 2,
        sync: boolean = false,
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
                    let prices =
                        this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
                            market.indexMintOracle,
                            stashedPrice,
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
                poolSummary.categoryTags.push('stable_pool');
            } else if (isMixed) {
                poolSummary.categoryTags.push('mix_pool');
            } else {
                poolSummary.categoryTags.push('standard_pool');
            }
            poolSummaries.push(poolSummary);
        }

        return poolSummaries;
    }

    public async getUserSummary(sync: boolean = false): Promise<UserSummary> {
        this.checkInitialization(true);
        let userSummary = {
            accountNetValue: new BigNumber(0),
            pnl: new BigNumber(0),
            earn: new BigNumber(0),
            tokens: [] as UserTokenSummary[],
        };
        let user = await this.getUser(sync);
        let tradeTokens = await this.getTradeTokens();
        let markets = await this.getMarkets(sync);
        let pools = await this.getPools();

        let accountNetValue = await this.userComponent!.getUserAccountNetValue(
            user,
            tradeTokens,
            markets,
            pools,
        );
        let balanceOfUserPositions =
            await BumpinPositionUtils.getUserPositionValue(
                this.tradeTokenComponent!,
                user,
                tradeTokens,
                markets,
                pools,
            );
        userSummary.accountNetValue = accountNetValue.accountNetValue;
        userSummary.pnl = balanceOfUserPositions.positionUnPnl;
        for (let tradeToken of tradeTokens) {
            let userTokenSummary = {
                token: tradeToken,
                amount: new BigNumber(0),
                used: new BigNumber(0),
                borrow: new BigNumber(0),
            };
            let userToken = BumpinUserUtils.getMyTokenByMint(
                user,
                tradeToken.mintKey,
            );
            if (userToken) {
                userTokenSummary.amount = userToken.amount;
                userTokenSummary.used = userToken.usedAmount;
                userTokenSummary.borrow = userToken.usedAmount.gt(
                    userToken.amount,
                )
                    ? userToken.usedAmount
                          .minus(userToken.amount)
                          .minus(userToken.liabilityAmount)
                    : new BigNumber(0);
            }
            userSummary.tokens.push(userTokenSummary);
        }
        return userSummary;
    }

    public async stake(
        fromPortfolio: boolean,
        size: number,
        mint: PublicKey,
        sync: boolean = false,
    ) {
        this.checkInitialization(true);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mint,
            await this.getTradeTokens(),
        );
        let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(
            mint,
            await this.getPools(),
        );
        if (fromPortfolio) {
            await this.userComponent!.portfolioStake(
                size,
                targetTradeToken,
                await this.getTradeTokens(),
                targetPool,
                await this.getMarkets(sync),
                await this.getPools(sync),
                sync,
            );
        } else {
            await this.userComponent!.walletStake(
                size,
                targetTradeToken,
                await this.getTradeTokens(),
                this.wallet.publicKey,
                targetPool,
                await this.getMarkets(sync),
                sync,
            );
        }
    }

    public async unStake(
        toPortfolio: boolean,
        share: BigNumber,
        mint: PublicKey,
        sync: boolean = false,
    ) {
        this.checkInitialization(true);

        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mint,
            await this.getTradeTokens(),
        );
        let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(
            mint,
            await this.getPools(),
        );
        let markets = await this.getMarkets(sync);
        await this.userComponent!.unStake(
            toPortfolio,
            share,
            targetTradeToken,
            this.wallet.publicKey,
            targetPool,
            markets,
        );
    }

    public async deposit(
        userTokenAccount: PublicKey,
        mintPublicKey: PublicKey,
        size: number,
    ) {
        this.checkInitialization(true);

        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            mintPublicKey,
            await this.getTradeTokens(),
        );
        let amount = BumpinUtils.size2Amount(
            new BigNumber(size),
            targetTradeToken.decimals,
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
        sync: boolean = false,
    ) {
        this.checkInitialization(true);

        let market = BumpinMarketUtils.getMarketByIndex(
            marketIndex,
            await this.getMarkets(sync),
        );
        await this.userComponent!.placePerpOrder(
            market.symbol,
            marketIndex,
            param,
            this.wallet.publicKey,
            await this.poolComponent!.getPools(sync),
            await this.marketComponent!.getMarkets(sync),
            await this.tradeTokenComponent!.getTradeTokens(sync),
        );
    }

    public async getUser(sync: boolean = false): Promise<User> {
        this.checkInitialization(true);
        return this.userComponent!.getUser(sync);
    }

    public async getState(sync: boolean = false): Promise<State> {
        if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
            throw new BumpinSubscriptionFailed('State', 0);
        }

        if (sync) {
            await this.stateSubscriber.fetch();
        }

        let state = this.stateSubscriber.state;
        if (!state) {
            throw new BumpinAccountNotFound('State');
        }
        return state.data;
    }

    public async getPools(sync: boolean = false): Promise<Pool[]> {
        this.checkInitialization();
        return this.poolComponent!.getPools(sync);
    }

    public async getRewards(sync: boolean = false): Promise<RewardsAccount[]> {
        this.checkInitialization();
        return this.rewardComponent!.getRewards(sync);
    }

    public async getPoolsWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<Pool>[]> {
        this.checkInitialization();
        return this.poolComponent!.getPoolsWithSlot(sync);
    }

    public async getPool(
        poolKey: PublicKey,
        sync: boolean = false,
    ): Promise<Pool> {
        this.checkInitialization();
        return this.poolComponent!.getPool(poolKey, sync);
    }

    public async getPoolByIndex(
        poolIndex: number,
        sync: boolean = false,
    ): Promise<Pool> {
        this.checkInitialization();
        return this.poolComponent!.getPool(
            BumpinUtils.getPoolPda(this.program, poolIndex)[0],
            sync,
        );
    }

    public async getPoolWithSlot(
        poolKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<Pool>> {
        this.checkInitialization();
        return this.poolComponent!.getPoolWithSlot(poolKey, sync);
    }

    public async getTradeTokens(sync: boolean = false): Promise<TradeToken[]> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokens(sync);
    }

    public async getTradeTokensWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<TradeToken>[]> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokensWithSlot(sync);
    }

    public async getTradeToken(
        tradeTokenKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeToken(tradeTokenKey, sync);
    }

    public async getTradeTokenByIndex(
        tradeTokenIndex: number,
        sync: boolean = false,
    ): Promise<TradeToken> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeToken(
            BumpinUtils.getTradeTokenPda(this.program, tradeTokenIndex)[0],
            sync,
        );
    }

    public async getTradeTokenByMintKey(
        mintKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokenByMintKey(mintKey, sync);
    }

    public async getTradeTokenByOracleKey(
        oracleKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokenByOracleKey(
            oracleKey,
            sync,
        );
    }

    public async getTradeTokenWithSlot(
        tradeTokenKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<TradeToken>> {
        this.checkInitialization();
        return this.tradeTokenComponent!.getTradeTokenWithSlot(
            tradeTokenKey,
            sync,
        );
    }

    public async getMarkets(sync: boolean = false): Promise<Market[]> {
        this.checkInitialization();
        return this.marketComponent!.getMarkets(sync);
    }

    public async getMarketsWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<Market>[]> {
        this.checkInitialization();
        return this.marketComponent!.getMarketsWithSlot(sync);
    }

    public async getMarket(
        marketKey: PublicKey,
        sync: boolean = false,
    ): Promise<Market> {
        this.checkInitialization();
        return this.marketComponent!.getMarket(marketKey, sync);
    }

    public async getMarketByIndex(
        marketIndex: number,
        sync: boolean = false,
    ): Promise<Market> {
        this.checkInitialization();
        return this.marketComponent!.getMarket(
            BumpinUtils.getMarketPda(this.program, marketIndex)[0],
            sync,
        );
    }

    public async getMarketWithSlot(
        marketKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<Market>> {
        this.checkInitialization();
        return this.marketComponent!.getMarketWithSlot(marketKey, sync);
    }

    public async getFundingFeeRate(
        marketIndex: number,
        sync: boolean = false,
    ): Promise<{
        long: number;
        short: number;
    }> {
        this.checkInitialization();
        const marketKey = BumpinUtils.getMarketPda(
            this.program,
            marketIndex,
        )[0];
        const market = await this.getMarket(marketKey, sync);
        let long = market.fundingFee.longFundingFeeRate.div(10 ** 10);
        let short = market.fundingFee.shortFundingFeeRate.div(10 ** 10);
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
        sync: boolean = false,
    ): Promise<number> {
        this.checkInitialization();
        const marketKey = BumpinUtils.getMarketPda(
            this.program,
            marketIndex,
        )[0];
        const timestamp = BigNumber(Math.floor(Date.now() / 1000));
        const market = await this.getMarket(marketKey, sync);
        const pool = await this.getPool(market.poolKey, sync);
        const timePassed = timestamp.minus(pool.borrowingFee.updatedAt);

        return pool.balance.holdAmount
            .div(pool.balance.amount.plus(pool.balance.unSettleAmount))
            .multipliedBy(pool.config.borrowingInterestRate)
            .multipliedBy(timePassed)
            .div(timePassed)
            .toNumber();
    }

    public async getPoolNetPrice(poolKey: PublicKey, sync: boolean = false) {
        this.checkInitialization();
        const pool = await this.getPool(poolKey, sync);
        const poolValueUsd = await this.getPoolValueUsd(poolKey, sync);
        if (pool.totalSupply.isZero()) {
            return new BigNumber(0);
        } else {
            return poolValueUsd.div(pool.totalSupply);
        }
    }

    //TODO: Dean, check this
    public async getPoolValueUsd(
        poolKey: PublicKey,
        sync: boolean = false,
    ): Promise<BigNumber> {
        this.checkInitialization();
        const pool = await this.getPool(poolKey, sync);
        const tradeToken = await this.getTradeTokenByMintKey(
            pool.mintKey,
            sync,
        );
        const price =
            await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
                pool.mintKey,
                sync,
            );
        if (!price.price) {
            throw new BumpinInvalidParameter(
                'Price not found(undefined) for mint: ' +
                    pool.mintKey.toString(),
            );
        }
        const relativeMarkets = BumpinMarketUtils.getMarketsByPoolKey(
            poolKey,
            await this.getMarkets(sync),
        );
        // self usd value
        let rawValue = pool.balance.amount
            .plus(pool.balance.unSettleAmount)
            .multipliedBy(price.price!);
        if (!pool.stable) {
            // relative market unpnl usd value
            for (let relativeMarket of relativeMarkets) {
                const marketUnPnlUsd = await this.getMarketUnPnlUsd(
                    relativeMarket,
                );
                rawValue = rawValue
                    .plus(marketUnPnlUsd.longUnPnl)
                    .plus(marketUnPnlUsd.shortUnPnl);
            }
            // relative stable pool gain and loss
            const stableAmount = pool.stableBalance.amount
                .plus(pool.stableBalance.unSettleAmount)
                .minus(pool.stableBalance.lossAmount);
            if (stableAmount.gt(BigNumber(0))) {
                const stablePrice =
                    await this.tradeTokenComponent!.getTradeTokenPricesByMintKey(
                        pool.stableMintKey,
                        sync,
                    );
                if (!stablePrice.price) {
                    throw new BumpinInvalidParameter(
                        'Stable Price not found(undefined) for mint: ' +
                            pool.mintKey.toString(),
                    );
                }
                rawValue = rawValue.plus(
                    stableAmount.multipliedBy(stablePrice.price),
                );
            }
        }

        return rawValue;
    }

    //TODO: Dean, check this
    public async getMarketUnPnlUsd(market: Market): Promise<MarketUnPnlUsd> {
        this.checkInitialization();
        let longUnPnl = BigNumber(0);
        let shortUnPnl = BigNumber(0);

        const longPosition = market.longOpenInterest;
        const shortPosition = market.shortOpenInterest;
        const price = this.tradeTokenComponent!.getTradeTokenPricesByOracleKey(
            market.indexMintOracle,
            1,
        )[0];

        if (!price.price) {
            throw new BumpinInvalidParameter(
                'Price not found(undefined) for oracle: ' +
                    market.indexMintOracle.toString(),
            );
        }

        // cal long:
        if (!longPosition.entryPrice.isZero()) {
            longUnPnl = longPosition.openInterest
                .multipliedBy(
                    BigNumber(price.price).minus(longPosition.entryPrice),
                )
                .div(longPosition.entryPrice)
                .multipliedBy(BigNumber(-1));
        }

        // cal short:
        if (!shortPosition.entryPrice.isZero()) {
            shortUnPnl = shortPosition.openInterest
                .multipliedBy(shortPosition.entryPrice.minus(price.price))
                .div(shortPosition.entryPrice)
                .multipliedBy(BigNumber(-1));
        }

        return new MarketUnPnlUsd(longUnPnl, shortUnPnl);
    }

    //TODO: Dean, check this
    public async getUserAccountNetValue(
        sync: boolean = false,
    ): Promise<AccountValue> {
        this.checkInitialization(true);
        let accountValue = {
            netValue: new BigNumber(0),
            totalMM: new BigNumber(0),
        };
        const user = await this.getUser(sync);

        let accountNetValue = await this.userComponent!.getUserAccountNetValue(
            user,
            await this.getTradeTokens(),
            await this.getMarkets(),
            await this.getPools(),
        );
        accountValue.netValue = accountNetValue.accountNetValue;
        accountValue.totalMM = accountNetValue.totalMM;
        return accountValue;
    }

    //TODO: Dean, check this
    public async getUserAvailableValue(
        sync: boolean = false,
    ): Promise<BigNumber> {
        this.checkInitialization(true);
        const user = await this.getUser(sync);
        return await this.userComponent!.getUserAvailableValue(
            user,
            await this.getTradeTokens(),
            await this.getMarkets(),
            await this.getPools(),
        );
    }

    //TODO: Dean, check this
    public async claimUserRewards(): Promise<UserClaimResult> {
        this.checkInitialization(true);
        let user = await this.getUser();
        let claimResult: UserClaimResult = {
            claimed: BigNumber(0),
            unClaim: BigNumber(0),
            total: BigNumber(0),
            rewards: [],
        };
        for (const stake of user.stakes) {
            if (
                isEqual(stake.userStakeStatus, UserStakeStatus.USING) &&
                stake.userRewards.openRewardsPerStakeToken.gt(BigNumber(0))
            ) {
                let pool = await this.getPool(stake.poolKey);
                const price = this.tradeTokenComponent!.getTradeTokenPrices(
                    stake.userRewards.tokenKey,
                ).price!;
                let unRealisedRewards =
                    pool.feeReward.cumulativeRewardsPerStakeToken
                        .minus(stake.userRewards.openRewardsPerStakeToken)
                        .multipliedBy(stake.stakedShare);

                claimResult.total = claimResult.total.plus(
                    unRealisedRewards
                        .plus(stake.userRewards.totalClaimRewardsAmount)
                        .multipliedBy(price),
                );
                claimResult.claimed = claimResult.claimed.plus(
                    stake.userRewards.totalClaimRewardsAmount.multipliedBy(
                        price,
                    ),
                );
                claimResult.unClaim = claimResult.unClaim.plus(
                    unRealisedRewards.multipliedBy(price),
                );
                let userClaimRewardsResult: UserClaimRewardsResult = {
                    pool: pool.name,
                    rewardsAmount: unRealisedRewards.multipliedBy(price),
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
            throw new BumpinClientNotInitialized('State');
        }

        if (!this.poolComponent) {
            throw new BumpinClientNotInitialized('Pool');
        }

        if (!this.rewardComponent) {
            throw new BumpinClientNotInitialized('Reward');
        }

        if (!this.tradeTokenComponent) {
            throw new BumpinClientNotInitialized('TradeToken');
        }

        if (!this.marketComponent) {
            throw new BumpinClientNotInitialized('Market');
        }

        if (mustLogin && !this.userComponent) {
            throw new BumpinUserNotLogin();
        }
    }
}
