import {Keypair, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {Pyth} from "../../target/types/pyth";
import {Utils} from "../utils/utils";
import {ExchangeInitializeParams} from "./initialize_params";


export class BumpinPool {
    utils = new Utils();
    program: Program<BumpinTrade>;
    poolName: string;
    isStable: boolean;
    mint: Keypair;
    payer: Keypair;
    mintDecimals: number;
    stateNumberOfPools: number;

    constructor(program: Program<BumpinTrade>, poolName: string, isStable: boolean, payer: Keypair, mintDecimals: number, stateNumberOfPools: number) {
        this.program = program;
        this.poolName = poolName;
        this.isStable = isStable;
        this.payer = payer;
        this.mintDecimals = mintDecimals;
        this.stateNumberOfPools = stateNumberOfPools;
    }

    public async initializePool() {
        this.mint = await this.utils.create_mint_account(this.payer, this.payer, this.mintDecimals);
        await this.utils.initialize_pool(this.program, this.mint.publicKey, this.poolName, this.payer);
    }
}


export class BumpinPlayer {
    utils = new Utils();
    program: Program<BumpinTrade>;
    secretKey?: Uint8Array;
    payer: Keypair;
    user?: Keypair;
    playerName: string; // This field is ONLY for testing purposes

    constructor(program: Program<BumpinTrade>, payer: Keypair, name: string, secretKey?: Uint8Array) {
        this.program = program;
        this.payer = payer;
        this.secretKey = secretKey;
        this.playerName = name;
    }

    public async initializePlayer() {
        this.user = await this.utils.new_user(this.program.provider as AnchorProvider, this.secretKey);
        await this.utils.initialize_user(this.user, this.payer);
    }
}

export class BumpinExchange {
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;
    utils = new Utils();
    initialized = false;
    initialize_params: ExchangeInitializeParams;

    payer: Keypair;

    players: BumpinPlayer[] = [];
    pools: BumpinPool[] = [];

    public async initialize(params: ExchangeInitializeParams) {
        this.initialize_params = params;
        this.payer = await this.utils.new_user(this.program.provider as AnchorProvider, null, 1000);

        await this.utils.initialize_state(this.payer);
        for (let playerInfo of params.playerInfos) {
            let player = new BumpinPlayer(this.program, this.payer, playerInfo.name, playerInfo.secretKey);
            await player.initializePlayer();
            this.players.push(player);
        }

        for (let i = 0; i < params.poolInfos.length; i++) {
            let poolInfo = params.poolInfos[i];
            let pool = new BumpinPool(this.program, poolInfo.name, poolInfo.isStable, this.payer, poolInfo.mintDecimals, i);
            await pool.initializePool();
            const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([pool.stateNumberOfPools]).buffer);
            const [address, nonce] = PublicKey.findProgramAddressSync(
                [Buffer.from("pool"), stateNumberOfPoolsBytes],
                pool.program.programId
            );
            this.pools.push(pool);
            const state = await this.program.account.state.fetch(this.utils.getStatePda(this.program)[0]);
        }

        this.initialized = true;
    }


    public getUser(playerName: string) {
        return this.players.find(player => player.playerName === playerName);
    }

    public getUserPda(playerName: string): [PublicKey, number] {
        let user = this.getUser(playerName);
        const [address, nonce] = PublicKey.findProgramAddressSync(
            [Buffer.from("user"), user.user.publicKey.toBuffer()],
            user.program.programId
        );

        return [address, nonce];
    }

    public getPool(poolName: string) {
        return this.pools.find(pool => pool.poolName === poolName);
    }

    public getPoolPda(poolName: string): [PublicKey, number] {
        let pool = this.getPool(poolName);
        const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([pool.stateNumberOfPools]).buffer);
        const [address, nonce] = PublicKey.findProgramAddressSync(
            [Buffer.from("pool"), stateNumberOfPoolsBytes],
            pool.program.programId
        );

        return [address, nonce];
    }

}