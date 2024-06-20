import {Keypair, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {Pyth} from "../../target/types/pyth";
import {Utils} from "../utils/utils";
import {ExchangeInitializeParams} from "./initialize_params";
import BN from "bn.js";
import {Account} from "@solana/spl-token";


export class BumpinPlayer {
    utils = new Utils();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    secretKey?: Uint8Array;
    payer: Keypair;
    user?: Keypair;
    playerName: string;

    // name - keypair
    tradeTokenAccounts: Map<string, Account> = new Map<string, Account>();


    constructor(payer: Keypair, name: string, secretKey?: Uint8Array) {
        this.payer = payer;
        this.secretKey = secretKey;
        this.playerName = name;
    }

    public async initializePlayer() {
        this.user = await this.utils.new_user(this.program.provider as AnchorProvider, this.secretKey);
        await this.utils.initialize_user(this.user, this.payer);
    }

    public async createTradeTokenAccount(tradeTokenName: string, mint: PublicKey) {
        let tradeTokenAccount = await this.utils.createTokenAccount(this.program.provider, this.user, mint, this.user.publicKey);
        this.tradeTokenAccounts.set(tradeTokenName, tradeTokenAccount);
    }

    public async mintTradeToken(tradeTokenName: string, mint: PublicKey, amount: number, decimals: number) {
        let account = this.getTradeTokenAccount(tradeTokenName);
        await this.utils.mintTo(this.program.provider, this.payer, mint, account.address, amount, decimals);
    }

    public getTradeTokenAccount(tradeTokenName: string) {
        return this.tradeTokenAccounts.get(tradeTokenName);
    }
}


export class BumpinPool {
    utils = new Utils();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    poolName: string;
    isStable: boolean;
    mint: Keypair;
    payer: Keypair;
    mintDecimals: number;
    stateNumberOfPools: number;

    constructor(poolName: string, isStable: boolean, payer: Keypair, mintDecimals: number, stateNumberOfPools: number) {
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

    public getPda(): [PublicKey, number] {
        const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([this.stateNumberOfPools]).buffer);
        const [address, nonce] = PublicKey.findProgramAddressSync(
            [Buffer.from("pool"), stateNumberOfPoolsBytes],
            this.program.programId
        );
        return [address, nonce];
    }
}

export class BumpinMarket {
    utils = new Utils();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;
    symbol: string;
    payer: Keypair;
    pool: BumpinPool;
    indexToken: BumpinTradeToken;
    stablePool: BumpinPool;

    constructor(symbol: string, payer: Keypair, pool: BumpinPool, indexToken: BumpinTradeToken, stablePool: BumpinPool) {
        this.symbol = symbol;
        this.payer = payer;
        this.pool = pool;
        this.indexToken = indexToken;
        this.stablePool = stablePool;
    }

    public async initializeMarket() {
        let [poolPda, _] = this.pool.getPda();
        let [stablePoolPda, __] = this.stablePool.getPda();
        await this.utils.initialize_market(this.symbol, this.payer, poolPda, stablePoolPda, this.indexToken.mint.publicKey);
    }

}

export class BumpinTradeToken {
    utils = new Utils();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;

    tradeTokenName: string;
    payer: Keypair;
    mint: Keypair;
    oracle: PublicKey;
    discount: BN;
    liquidationFactor: BN;

    numberOfTradeTokens: number;

    constructor(tradeTokenName: string, payer: Keypair, oracle: PublicKey, discount: BN, liquidationFactor: BN, numberOfTradeTokens: number) {
        this.tradeTokenName = tradeTokenName;
        this.payer = payer;
        this.oracle = oracle;
        this.discount = discount;
        this.liquidationFactor = liquidationFactor;
        this.numberOfTradeTokens = numberOfTradeTokens;
    }

    public async initializeTradeToken() {
        this.mint = await this.utils.create_mint_account(this.payer, this.payer);
        await this.utils.initialize_trade_token(this.mint.publicKey, this.payer, this.oracle, this.discount, this.liquidationFactor);
    }

    public getMint() {
        return this.mint;
    }
}

export class BumpinExchange {
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;
    utils = new Utils();
    initialized = false;
    initialize_params: ExchangeInitializeParams;

    payer: Keypair;

    oracle: Keypair;

    players: BumpinPlayer[] = [];
    pools: BumpinPool[] = [];
    tradeTokens: Map<string, BumpinTradeToken> = new Map<string, BumpinTradeToken>();

    public async initialize(params: ExchangeInitializeParams) {
        this.initialize_params = params;
        this.payer = await this.utils.new_user(this.program.provider as AnchorProvider, null, 10000);

        //init oracle
        let oracle_payer = await this.utils.new_user(this.programPyth.provider as anchor.AnchorProvider);
        this.oracle = anchor.web3.Keypair.generate();
        await this.utils.manualCreateAccount(this.programPyth.provider, oracle_payer, this.oracle, 3312,
            await this.programPyth.provider.connection.getMinimumBalanceForRentExemption(
                3312
            ), this.programPyth.programId);

        await this.utils.initialize_state(this.payer);
        for (let playerInfo of params.playerInfos) {
            let player = new BumpinPlayer(this.payer, playerInfo.name, playerInfo.secretKey);
            await player.initializePlayer();
            this.players.push(player);
        }

        for (let i = 0; i < params.poolInfos.length; i++) {
            let poolInfo = params.poolInfos[i];
            let pool = new BumpinPool(poolInfo.name, poolInfo.isStable, this.payer, poolInfo.mintDecimals, i);
            await pool.initializePool();
            const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([pool.stateNumberOfPools]).buffer);
            const [address, nonce] = PublicKey.findProgramAddressSync(
                [Buffer.from("pool"), stateNumberOfPoolsBytes],
                pool.program.programId
            );
            this.pools.push(pool);
            // const state = await this.program.account.state.fetch(this.utils.getStatePda(this.program)[0]);
        }

        for (let i = 0; i < params.tradeTokenInfos.length; i++) {
            let tradeTokenInfo = params.tradeTokenInfos[i];
            let tradeToken = new BumpinTradeToken(tradeTokenInfo.name, this.payer, this.oracle.publicKey, tradeTokenInfo.discount, tradeTokenInfo.liquidationFactor, i);
            await tradeToken.initializeTradeToken();
            this.tradeTokens.set(tradeTokenInfo.name, tradeToken);
            let mint = tradeToken.getMint();
            for (let player of this.players) {
                await player.createTradeTokenAccount(tradeTokenInfo.name, mint.publicKey);
            }
        }

        for (let marketInfo of params.marketInfos) {
            let pool = this.pools.find(pool => pool.poolName === marketInfo.poolName);
            let indexToken = this.tradeTokens.get(marketInfo.indexTokenName);
            let stablePool = this.pools.find(pool => pool.poolName === marketInfo.stablePoolName);
            let market = new BumpinMarket(marketInfo.symbol, this.payer, pool, indexToken, stablePool);
            await market.initializeMarket();
        }

        this.initialized = true;
    }

    public async playerDeposit(playerName: string, tradeTokenName: string, amount: number) {
        let player = this.getPlayer(playerName);
        let tradeToken = this.getTradeToken(tradeTokenName);
        let tradeTokenAccount = player.getTradeTokenAccount(tradeTokenName);
        await this.utils.deposit(player.user, tradeTokenAccount.address, tradeToken.numberOfTradeTokens, new BN(amount));
    }


    public getPlayer(playerName: string) {
        return this.players.find(player => player.playerName === playerName);
    }

    public getPlayers() {
        return this.players;
    }

    public getTradeToken(tradeTokenName: string) {
        return this.tradeTokens.get(tradeTokenName);
    }

    public getTradeTokens() {
        return this.tradeTokens;
    }


    public getUserPda(playerName: string): [PublicKey, number] {
        let user = this.getPlayer(playerName);
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