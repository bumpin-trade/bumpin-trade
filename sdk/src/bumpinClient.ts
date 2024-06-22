import {Connection} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import {BumpinClientConfig} from "./bumpinClientConfig";
import {BumpinUtils} from "./utils/utils";
import {BumpinTrade} from "./types/bumpin_trade";
import {State, UserAccount} from "./types";


export class BumpinClient {
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    isInitialized: boolean = false;

    state: State | null = null;

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
        this.state = await this.getState();
        this.isInitialized = true;
    }

    public async me(): Promise<UserAccount> {
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.wallet.publicKey.toBuffer()]);
        return await this.program.account.user.fetch(pda) as any as UserAccount;
    }

    // public async getAllTradeTokens(): Promise<string[]> {
    //
    // }

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