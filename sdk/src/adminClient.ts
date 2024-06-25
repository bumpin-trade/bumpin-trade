import {Connection, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import idlPyth from "./idl/pyth.json"
import {BumpinTrade} from "./types/bumpin_trade";
import {Market, Pool, State, TradeToken} from "./types";
import {BumpinAdminConfig} from "./bumpinAdminConfig";
import {BumpinUtils} from "./utils/utils";
import {Pyth} from "./types/pyth";

export class BumpinAdmin {
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    TEST_PYTH: Program<Pyth>;

    isInitialized: boolean = false;

    state: State | null = null;
    tradeTokens: TradeToken[] = [];
    pools: Pool[] = [];
    market: Market[] = [];

    constructor(config: BumpinAdminConfig) {
        this.connection = new Connection(config.endpoint);
        this.wallet = config.wallet;
        this.provider = new anchor.AnchorProvider(this.connection, this.wallet, anchor.AnchorProvider.defaultOptions());
        this.program = new anchor.Program(JSON.parse(JSON.stringify(idlBumpinTrade)), this.provider);
        //TEST ONLY
        this.TEST_PYTH = new anchor.Program(JSON.parse(JSON.stringify(idlPyth)), this.provider);
    }


    public async initState(param: any) {
        await this.program.methods.initializeState(
            param
        ).accounts({
            admin: this.wallet.publicKey,
        }).signers([]).rpc();
    }

    public async initPool(poolName: string, poolMint: anchor.web3.PublicKey) {
        const [pda, _] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods.initializePool(
            BumpinUtils.string2Padded32Bytes(poolName),
        ).accounts({
            poolMint,
            bumpSigner: pda,
        }).signers([]).rpc();
    }

    public async DEV_TEST_ONLY__INIT_ORACLE(initPrice: number, confidence = undefined, expo = -4): Promise<anchor.web3.Keypair> {
        let oracleKeypair = anchor.web3.Keypair.generate();
        await BumpinUtils.manualCreateAccount(this.provider, this.wallet, oracleKeypair, 3312,
            await this.TEST_PYTH.provider.connection.getMinimumBalanceForRentExemption(
                3312
            ), this.TEST_PYTH.programId);
        const conf = new BN(confidence) || new BN((initPrice / 10) * 10 ** -expo);
        await this.TEST_PYTH.methods.initialize(
            new anchor.BN(initPrice),
            expo,
            conf
        ).accounts({
            price: oracleKeypair.publicKey,
        }).signers([])
            .rpc();
        return oracleKeypair;
    }

    public async initTradeToken(tradeTokenName: string, tradeTokenMint: string, discount: BN, liquidationFactor: BN) {
        let tradeTokenMintPublicKey = new PublicKey(tradeTokenMint);
        const s = BumpinUtils.string2Padded32Bytes(tradeTokenName);
        let oracleKeypair = await this.DEV_TEST_ONLY__INIT_ORACLE(70000, 1.0, -4);

        let [pda, nonce] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods.initializeTradeToken(
            discount, s, liquidationFactor
        ).accounts({
            tradeTokenMint: tradeTokenMintPublicKey,
            oracle: oracleKeypair.publicKey,
            bumpSigner: pda,
        }).signers([]).rpc();

    }


    public async initMarket(poolName: string, pool: anchor.web3.PublicKey, stablePool: anchor.web3.PublicKey, indexMint: anchor.web3.PublicKey) {
        const [pda, _] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods.initializeMarket(
            BumpinUtils.string2Padded32Bytes(poolName)
        ).accounts({
            pool,
            stablePool,
            indexMint,
            bumpSigner: pda,
        }).signers([]).rpc();
    }


}