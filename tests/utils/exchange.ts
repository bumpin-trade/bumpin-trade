import {Keypair} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {Pyth} from "../../target/types/pyth";
import {Utils} from "./utils";


export class BumpinPool {
    utils = new Utils();
    program: Program;
    poolName: string;
    isStable: boolean;
    mint: Keypair;
    payer: Keypair;
    mintDecimals: number;


    constructor(program: Program, poolName: string, isStable: boolean, payer: Keypair, mintDecimals: number) {
        this.program = program;
        this.poolName = poolName;
        this.isStable = isStable;
        this.payer = payer;
        this.mintDecimals = mintDecimals;
    }

    public async initializePool() {
        this.mint = await this.utils.create_mint_account(this.payer, this.payer, this.mintDecimals);
        await this.utils.initialize_pool(this.mint.publicKey, this.poolName, this.payer);
    }
}


export class BumpinPlayer {
    utils = new Utils();
    program: Program;
    secretKey?: Uint8Array;
    payer: Keypair;
    user?: Keypair;

    constructor(program: Program, payer: Keypair, secretKey?: Uint8Array) {
        this.program = program;
        this.payer = payer;
        this.secretKey = secretKey;
    }

    public async initializePlayer() {
        this.user = await this.utils.new_user(this.program.provider as AnchorProvider, this.secretKey);
        await this.utils.initialize_user(this.user, this.payer);
    }
}

export class BumpinExchange {
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;

    public async initialize() {}
}