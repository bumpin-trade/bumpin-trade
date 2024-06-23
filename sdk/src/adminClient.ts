import {Connection, sendAndConfirmTransaction, SystemProgram, Transaction} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"
import idlPyth from "./idl/pyth.json"
import {BumpinTrade} from "./types/bumpin_trade";
import {Market, Pool, State, TradeToken} from "./types";
import {BumpinAdminConfig} from "./bumpinAdminConfig";
import {ACCOUNT_SIZE, createInitializeMintInstruction, MintLayout, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {BumpinUtils} from "./utils/utils";
import {Pyth} from "./types/pyth";
import BN from "bn.js";

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


    public async initPool(param: any) {
        await this.program.methods.initializeState(
            param
        ).accounts({
            admin: this.wallet.publicKey,
        }).signers([]).rpc();
    }

    public async DEV_TEST_ONLY__INIT_ORACLE(initPrice: number, confidence = undefined, expo = -4): Promise<anchor.web3.Keypair> {
        let oracleKeypair = anchor.web3.Keypair.generate();
        await BumpinUtils.manualCreateAccount(this.provider, this.wallet.publicKey, oracleKeypair, 3312,
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
        }).rpc();
        return oracleKeypair;
    }

    public async initTradeToken(tradeTokenName: string, decimals: number) {
        const lamports = await this.provider.connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE);
        let account = anchor.web3.Keypair.generate();
        const transaction = new Transaction();
        transaction.add(
            SystemProgram.createAccount({
                fromPubkey: this.wallet.publicKey,
                newAccountPubkey: account.publicKey,
                space: MintLayout.span,
                lamports,
                programId: TOKEN_PROGRAM_ID,
            })
        );

        transaction.add(
            createInitializeMintInstruction(
                account.publicKey,
                decimals,
                this.wallet.publicKey,
                this.wallet.publicKey,
                TOKEN_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            this.provider.connection,
            transaction,
            []
        );

        const s = BumpinUtils.string2Padded32Bytes(tradeTokenName);
        await this.initialize_oracle(oracle, 70000, 1.0, -4);
        let [pda, nonce] = this.getStatePda();
        await this.program.methods.initializeTradeToken(
            discount, s, liquidationFactor
        ).accounts({
            tradeTokenMint,
            oracle,
            bumpSigner: pda,
            admin: admin.publicKey,
        }).signers([admin]).rpc();

    }


    public async initMarket(param: any) {
        await this.program.methods.initializeState(
            param
        ).accounts({
            admin: this.wallet.publicKey,
        }).signers([]).rpc();
    }


}