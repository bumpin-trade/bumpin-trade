import {Connection, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import {BumpinClientConfig} from "./bumpinClientConfig";
import {BumpinUtils} from "./utils/utils";
import {BumpinTrade} from "./types/bumpin_trade";
import {Market, Pool, State, TradeToken, UserAccount} from "./types";
import {BumpinAccountNotFound, BumpinSubscriptionFailed} from "./errors";
import {PollingUserAccountSubscriber} from "./account/pollingUserAccountSubscriber";
import {BulkAccountLoader} from "./account/bulkAccountLoader";
import {DataAndSlot} from "./account/types";


export class BumpinClient {
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    isInitialized: boolean = false;
    bulkAccountLoader: BulkAccountLoader;

    //subscriptions
    userAccountSubscriber: PollingUserAccountSubscriber;

    state: State | null = null;
    tradeTokens: TradeToken[] = [];
    pools: Pool[] = [];
    market: Market[] = [];

    constructor(config: BumpinClientConfig) {
        this.connection = new Connection(config.endpoint);
        this.wallet = config.wallet;
        this.provider = new anchor.AnchorProvider(this.connection, this.wallet, anchor.AnchorProvider.defaultOptions());
        this.program = new anchor.Program(JSON.parse(JSON.stringify(idlBumpinTrade)), this.provider);
        this.bulkAccountLoader = new BulkAccountLoader(this.connection, "confirmed", config.pollingFrequency);
    }


    public async initialize() {
        if (this.isInitialized) {
            return;
        }
        await this.syncInitialize();
        this.isInitialized = true;
    }


    public getState(): State | null {
        return this.state;
    }

    public async syncInitialize() {
        this.state = await this.syncState();
        this.tradeTokens = await this.syncTradeTokens();
        this.pools = await this.syncPools();
        this.market = await this.syncMarket();
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

    public async login(autoSubscription: boolean = true): Promise<UserAccount> {
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.wallet.publicKey.toBuffer()]);
        let me = await this.program.account.user.fetch(pda) as any as UserAccount;
        if (autoSubscription) {
            await this.subscriptionMe();
        }
        return me;
    }

    public async getUserAccount(sync: boolean = false, withSlot: boolean = false, autoSubscription: boolean = true): Promise<UserAccount | DataAndSlot<UserAccount>> {
        if (!this.userAccountSubscriber || !this.userAccountSubscriber.isSubscribed) {
            if (autoSubscription) {
                await this.subscriptionMe();
            } else {
                throw new BumpinAccountNotFound("User")
            }
        }

        if (sync) {
            await this.userAccountSubscriber.fetch();
        }

        let userAccount = this.userAccountSubscriber.user;
        if (!userAccount) {
            throw new BumpinAccountNotFound("User")
        }
        if (withSlot) {
            return userAccount;
        }
        return userAccount.data;
    }

    public async syncMarket(): Promise<Market[]> {
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }
        let markets = [];
        for (let i = 0; i < this.state!.numberOfMarkets; i++) {
            const [pda, _] = BumpinUtils.getMarketPda(this.program, i);
            markets.push(await this.program.account.market.fetch(pda) as any as Market);
        }
        return markets;
    }

    public async syncPools(): Promise<Pool[]> {
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }
        let pools = [];
        for (let i = 0; i < this.state!.numberOfPools; i++) {
            const [pda, _] = BumpinUtils.getPoolPda(this.program, i);
            pools.push(await this.program.account.pool.fetch(pda) as any as Pool);
        }
        return pools;
    }

    public async syncTradeTokens(): Promise<TradeToken[]> {
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }

        let tradeTokens = [];
        for (let i = 0; i < this.state!.numberOfTradeTokens; i++) {
            const [pda, _] = BumpinUtils.getTradeTokenPda(this.program, i);
            let tradeToken = (await this.program.account.tradeToken.fetch(pda)) as TradeToken;
            tradeTokens.push(tradeToken);
        }
        return tradeTokens;
    }

    public async syncState(): Promise<State> {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        return await this.program.account.state.fetch(statePda) as any as State;
    }

    public async initializeUser() {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods.initializeUser().accounts({
            state: statePda,
            authority: this.wallet.publicKey,
            payer: this.wallet.publicKey
        }).signers([]).rpc();
    }

    public async deposit(userTokenAccount: PublicKey, mintPublicKey: PublicKey, amount: BN) {
        const [statePda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let targetTradeToken = BumpinUtils.getTradeTokenByMintPublicKey(mintPublicKey, this.tradeTokens);
        await this.program.methods.deposit(targetTradeToken.tokenIndex, amount).accounts({
            userTokenAccount,
        }).signers([]).rpc();

    }

}