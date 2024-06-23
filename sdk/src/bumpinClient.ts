import {Connection} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import {BumpinClientConfig} from "./bumpinClientConfig";
import {BumpinUtils} from "./utils/utils";
import {BumpinTrade} from "./types/bumpin_trade";
import {Market, Pool, State, TradeToken, UserAccount} from "./types";
import {BumpinAccountNotFound, BumpinClientNotInitialized} from "./errors";


export class BumpinClient {
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    isInitialized: boolean = false;

    state: State | null = null;
    tradeTokens: TradeToken[] = [];
    pools: Pool[] = [];
    market: Market[] = [];

    constructor(config: BumpinClientConfig) {
        this.connection = new Connection(config.endpoint);
        this.wallet = config.wallet;
        this.provider = new anchor.AnchorProvider(this.connection, this.wallet, anchor.AnchorProvider.defaultOptions());
        this.program = new anchor.Program(JSON.parse(JSON.stringify(idlBumpinTrade)), this.provider);
    }

    public async initialize() {
        if (this.isInitialized) {
            return;
        }
        await this.syncInitialize();
        this.isInitialized = true;
    }

    public async syncInitialize() {
        this.state = await this.getState();
        this.tradeTokens = await this.getTradeTokens();
        this.pools = await this.getPools();
        this.market = await this.getMarkets();
    }

    public async me(): Promise<UserAccount> {
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.wallet.publicKey.toBuffer()]);
        return await this.program.account.user.fetch(pda) as any as UserAccount;
    }

    public async getMarkets(): Promise<Market[]> {
        if (!this.isInitialized) {
            throw new BumpinClientNotInitialized();
        }
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }
        let markets = [];
        for (let i = 0; i < this.state!.number_of_markets; i++) {
            const [pda, _] = BumpinUtils.getMarketPda(this.program, i);
            markets.push(await this.program.account.market.fetch(pda) as any as Market);
        }
        return markets;
    }

    public async getPools(): Promise<Pool[]> {
        if (!this.isInitialized) {
            throw new BumpinClientNotInitialized();
        }
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }
        let pools = [];
        for (let i = 0; i < this.state!.number_of_pools; i++) {
            const [pda, _] = BumpinUtils.getPoolPda(this.program, i);
            pools.push(await this.program.account.pool.fetch(pda) as any as Pool);
        }
        return pools;
    }

    public async getTradeTokens(): Promise<TradeToken[]> {
        if (!this.isInitialized) {
            throw new BumpinClientNotInitialized();
        }
        if (!this.state) {
            throw new BumpinAccountNotFound("State")
        }

        let tradeTokens = [];
        for (let i = 0; i < this.state!.number_of_trade_tokens; i++) {
            const [pda, _] = BumpinUtils.getTradeTokenPda(this.program, i);
            tradeTokens.push(await this.program.account.tradeToken.fetch(pda) as any as TradeToken);
        }
        return tradeTokens;
    }

    public async getState(): Promise<State> {
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


}