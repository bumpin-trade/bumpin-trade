import {ConfirmOptions, Connection, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import idlPyth from "./idl/pyth.json"
import {BumpinClientConfig, NetType} from "./bumpinClientConfig";
import {BumpinUtils} from "./utils/utils";
import {BumpinTrade} from "./types/bumpin_trade";
import {Market, PlaceOrderParams, Pool, State, TradeToken, UserAccount} from "./types";
import {BumpinAccountNotFound, BumpinSubscriptionFailed, BumpinUserNotLogin} from "./errors";
import {PollingUserAccountSubscriber} from "./account/pollingUserAccountSubscriber";
import {BulkAccountLoader} from "./account/bulkAccountLoader";
import {DataAndSlot} from "./account/types";
import {PollingStateAccountSubscriber} from "./account/pollingStateAccountSubscriber";
import {PoolComponent} from "./componets/pool";
import {Pyth} from "./types/pyth";
import {PythClient} from "./oracles/pythClient";
import {UserComponent} from "./componets/user";
import {TradeTokenComponent} from "./componets/tradeToken";
import {MarketComponent} from "./componets/market";
import {BumpinTokenUtils} from "./utils/token";
import {BumpinPoolUtils} from "./utils/pool";
import {BumpinMarketUtils} from "./utils/market";


export class BumpinClient {
    netType: NetType;
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;
    programPyth: Program<Pyth>;

    isInitialized: boolean = false;
    bulkAccountLoader: BulkAccountLoader;

    pythClient: PythClient;

    // Systems subscriptions
    stateSubscriber: PollingStateAccountSubscriber;
    userAccountSubscriber: PollingUserAccountSubscriber;

    // Components
    poolComponent: PoolComponent;
    tradeTokenComponent: TradeTokenComponent;
    marketComponent: MarketComponent;
    userComponent: UserComponent;

    state: State | null = null;
    market: Market[] = [];

    constructor(config: BumpinClientConfig) {
        this.netType = config.netType;
        this.connection = new Connection(config.endpoint);
        this.wallet = config.wallet;
        let opt: ConfirmOptions = {
            skipPreflight: false,
            commitment: "root", //default commitment: root
            preflightCommitment: "root",
            maxRetries: 0,
            minContextSlot: null
        };
        this.provider = new anchor.AnchorProvider(this.connection, this.wallet, opt);
        this.program = new anchor.Program(JSON.parse(JSON.stringify(idlBumpinTrade)), this.provider);
        this.bulkAccountLoader = new BulkAccountLoader(this.connection, "confirmed", config.pollingFrequency);

        if (this.netType === NetType.LOCALNET || this.netType === NetType.CUSTOM) {
            this.programPyth = new anchor.Program(JSON.parse(JSON.stringify(idlPyth)), this.provider);
            this.pythClient = new PythClient(this.programPyth.provider.connection);
        }
    }


    public async initialize() {
        if (this.isInitialized) {
            return;
        }

        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        this.stateSubscriber = new PollingStateAccountSubscriber(this.program, statePda, this.bulkAccountLoader);
        await this.stateSubscriber.subscribe();


        this.poolComponent = new PoolComponent(this.pythClient, this.bulkAccountLoader, this.stateSubscriber, this.program);
        await this.poolComponent.subscribe();

        this.tradeTokenComponent = new TradeTokenComponent(this.bulkAccountLoader, this.stateSubscriber, this.program);
        await this.tradeTokenComponent.subscribe();

        this.marketComponent = new MarketComponent(this.bulkAccountLoader, this.stateSubscriber, this.program);
        await this.marketComponent.subscribe();

        this.isInitialized = true;
        console.log("BumpinClient initialized");
    }


    public async subscriptionMe(): Promise<PollingUserAccountSubscriber> {
        if (this.userAccountSubscriber) {
            await this.userAccountSubscriber.unsubscribe();
            this.userAccountSubscriber = undefined;
        }

        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.wallet.publicKey.toBuffer()]);
        let subscriptionMe = new PollingUserAccountSubscriber(this.program, pda, this.bulkAccountLoader);
        let success = subscriptionMe.subscribe();
        if (!success) {
            throw new BumpinSubscriptionFailed("User Account, pda: " + pda.toString() + " wallet: " + this.wallet.publicKey.toString());
        }
        this.userAccountSubscriber = subscriptionMe;
        return subscriptionMe;
    }

    public async login(): Promise<UserAccount> {
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.wallet.publicKey.toBuffer()]);
        let me = await this.program.account.user.fetch(pda) as any as UserAccount;
        if (me) {
            this.userComponent = new UserComponent(this.wallet.publicKey, this.pythClient, this.bulkAccountLoader, this.stateSubscriber, this.program);
            await this.userComponent.subscribe();
        }
        return me;
    }


    public async initializeUser() {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods.initializeUser().accounts({
            state: statePda,
            authority: this.wallet.publicKey,
            payer: this.wallet.publicKey
        }).signers([]).rpc();
    }

    public async stake(fromPortfolio: boolean, amount: BN, mint: PublicKey, sync: boolean = false) {
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(mint, await this.getTradeTokens());
        let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(mint, await this.getPools());
        if (fromPortfolio) {
            await this.userComponent.portfolioStake(amount, targetTradeToken, await this.getTradeTokens(), targetPool, sync);
        } else {
            await this.userComponent.walletStake(amount, targetTradeToken, await this.getTradeTokens(), this.wallet.publicKey, targetPool, sync);
        }
    }

    public async unStake(toPortfolio: boolean, share: BN, mint: PublicKey) {
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(mint, await this.getTradeTokens());
        let targetPool = BumpinPoolUtils.getPoolByMintPublicKey(mint, await this.getPools());
        await this.userComponent.unStake(toPortfolio, share, targetTradeToken, this.wallet.publicKey, targetPool);
    }

    public async deposit(userTokenAccount: PublicKey, mintPublicKey: PublicKey, amount: BN) {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let targetTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(mintPublicKey, await this.getTradeTokens());
        await this.program.methods.deposit(targetTradeToken.index, amount).accounts({
            userTokenAccount,
        }).signers([]).rpc();
    }

    public async placePerpOrder(marketIndex: number, param: PlaceOrderParams, sync: boolean = false) {
        let market = BumpinMarketUtils.getMarketByIndex(marketIndex, await this.getMarkets(sync));
        await this.userComponent.placePerpOrder(market.symbol, marketIndex, param, this.wallet.publicKey
            , await this.poolComponent.getPools(sync), await this.marketComponent.getMarkets(), await this.tradeTokenComponent.getTradeTokens(sync));
    }


    public async getUser(sync: boolean = false): Promise<UserAccount> {
        if (!this.userComponent) {
            throw new BumpinUserNotLogin()
        }
        return this.userComponent.getUser(sync);
    }

    public async getState(sync: boolean = false): Promise<State> {
        if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
            throw new BumpinSubscriptionFailed("State")
        }

        if (sync) {
            await this.stateSubscriber.fetch();
        }

        let state = this.stateSubscriber.state;
        if (!state) {
            throw new BumpinAccountNotFound("State")
        }
        return state.data;
    }

    public async getPools(sync: boolean = false): Promise<Pool[]> {
        return this.poolComponent.getPools(sync);
    }

    public async getPoolsWithSlot(sync: boolean = false): Promise<DataAndSlot<Pool>[]> {
        return this.poolComponent.getPoolsWithSlot(sync);
    }

    public async getPool(poolKey: PublicKey, sync: boolean = false): Promise<Pool> {
        return this.poolComponent.getPool(poolKey, sync);
    }

    public async getPoolWithSlot(poolKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<Pool>> {
        return this.poolComponent.getPoolWithSlot(poolKey, sync);
    }

    public async getTradeTokens(sync: boolean = false): Promise<TradeToken[]> {
        return this.tradeTokenComponent.getTradeTokens(sync);
    }

    public async getTradeTokensWithSlot(sync: boolean = false): Promise<DataAndSlot<TradeToken>[]> {
        return this.tradeTokenComponent.getTradeTokensWithSlot(sync);
    }

    public async getTradeToken(tradeTokenKey: PublicKey, sync: boolean = false): Promise<TradeToken> {
        return this.tradeTokenComponent.getTradeToken(tradeTokenKey, sync);
    }

    public async getTradeTokenWithSlot(tradeTokenKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<TradeToken>> {
        return this.tradeTokenComponent.getTradeTokenWithSlot(tradeTokenKey, sync);
    }

    public async getMarkets(sync: boolean = false): Promise<Market[]> {
        return this.marketComponent.getMarkets(sync);
    }

    public async getMarketsWithSlot(sync: boolean = false): Promise<DataAndSlot<Market>[]> {
        return this.marketComponent.getMarketsWithSlot(sync);
    }

    public async getMarket(marketKey: PublicKey, sync: boolean = false): Promise<Market> {
        return this.marketComponent.getMarket(marketKey, sync);
    }

    public async getMarketWithSlot(marketKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<Market>> {
        return this.marketComponent.getMarketWithSlot(marketKey, sync);
    }


}