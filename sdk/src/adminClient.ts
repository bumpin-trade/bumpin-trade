import {
    AccountInfo,
    ConfirmOptions,
    Connection,
    PublicKey,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction,
} from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, BN, Program, Wallet } from '@coral-xyz/anchor';
import idlBumpinTrade from './idl/bumpin_trade.json';
import idlPyth from './idl/pyth.json';
import { BumpinTrade } from './types/bumpin_trade';
import {
    InitializeMarketParams,
    InitializePoolParams,
    InitializeStateParams,
    MarketAccount,
    ModifyStateParams,
    PoolAccount,
    PoolConfigAccount,
    StateAccount,
    TradeTokenAccount,
} from './typedef';
import { BumpinAdminConfig } from './bumpinAdminConfig';
import { BumpinUtils } from './utils/utils';
import { Pyth } from './types/pyth';
import { parsePriceData, PriceData } from '@pythnetwork/client';

export class BumpinAdmin {
    connection: Connection;
    wallet: Wallet;
    provider: AnchorProvider;
    public program: Program<BumpinTrade>;

    TEST_PYTH: Program<Pyth>;

    isInitialized: boolean = false;

    state: StateAccount | null = null;
    tradeTokens: TradeTokenAccount[] = [];
    pools: PoolAccount[] = [];
    market: MarketAccount[] = [];

    constructor(config: BumpinAdminConfig) {
        this.connection = new Connection(config.endpoint);
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
        //TEST ONLY
        this.TEST_PYTH = new anchor.Program(
            JSON.parse(JSON.stringify(idlPyth)),
            this.provider,
        );
    }

    public getConnection(): Connection {
        return this.connection;
    }

    public getProgram(): Program<BumpinTrade> {
        return this.program;
    }

    public async getOnlinePriceData(
        priceAccountPublicKey: PublicKey,
    ): Promise<PriceData> {
        let connection = new Connection(
            'https://solana-mainnet.g.alchemy.com/v2/F9RT25tFEPdrMzTGOzst2hOqu4agu8Zw',
        );
        let buffer: AccountInfo<Buffer> | null =
            await connection.getAccountInfo(priceAccountPublicKey);
        return parsePriceData(buffer!.data);
    }

    public async modifyState(param: ModifyStateParams) {
        await this.program.methods
            .modifyState(param)
            .accounts({
                admin: this.wallet.publicKey,
            })
            .signers([])
            .rpc();
    }

    public async initializeAll(
        stateParam: InitializeStateParams,
        tradeTokenParams: WrappedInitializeTradeTokenParams[],
        poolParams: WrappedInitializePoolParams[],
        marketParams: WrappedInitializeMarketParams[],
        rewardPrams: WrappedInitializeRewardsParams[],
    ) {
        const [pda, _] = BumpinUtils.getBumpinStatePda(this.program);

        console.log(
            'Initializing All, From state, confirm level: root (may need many time)',
        );
        await this.initState(stateParam);
        console.log('State initialized');

        let tradeTokenOracleMap = new Map<string, string>();

        // ////////// init tradeToken
        // //TODO: remove oracle init when using Prod env.
        for (let p of tradeTokenParams) {
            let oracleKey = await this.initTradeToken(
                p.tradeTokenName,
                p.tradeTokenMint,
                p.discount,
                p.liquidationFactor,
                p.exponent,
                p.trueOraclePublicKey,
            );
            tradeTokenOracleMap.set(p.tradeTokenMint, oracleKey.toString());
            console.log(
                'TradeToken initialized: ',
                p.tradeTokenName,
                ' oracle: ',
                oracleKey.toString(),
            );
        }

        ///////// init pools
        for (let poolParam of poolParams) {
            await this.program.methods
                .initializePool(poolParam.param)
                .accounts({
                    poolMint: poolParam.poolMint,
                    bumpSigner: pda,
                })
                .signers([])
                .rpc(BumpinUtils.getDefaultConfirmOptions());
            console.log(
                'Pool initialized: ',
                BumpinUtils.decodeString(poolParam.param.name),
            );
        }

        //////// init rewards
        for (let rewardsPram of rewardPrams) {
            let daoRewardsPublicKey = new PublicKey(rewardsPram.daoVault);
            await this.program.methods
                .initializeRewards(rewardsPram.poolIndex)
                .accounts({
                    poolMint: rewardsPram.poolMint,
                    daoRewardsVault: daoRewardsPublicKey,
                    bumpSigner: pda,
                })
                .signers([])
                .rpc(BumpinUtils.getDefaultConfirmOptions());
            console.log('Reward initialized: ', rewardsPram.poolIndex);
        }

        //////// init markets
        for (let marketParam of marketParams) {
            let oracleKey = new PublicKey(
                tradeTokenOracleMap.get(marketParam.indexMint.toString())!,
            );
            await this.program.methods
                .initializeMarket(marketParam.params)
                .accounts({
                    indexMintOracle: oracleKey,
                    bumpSigner: pda,
                })
                .signers([])
                .rpc(BumpinUtils.getRootConfirmOptions());
            console.log(
                'Market initialized: ',
                BumpinUtils.decodeString(marketParam.params.symbol),
            );
        }
        console.log('All initialized!');
    }

    public async initState(param: InitializeStateParams) {
        await this.program.methods
            .initializeState(param)
            .accounts({
                admin: this.wallet.publicKey,
            })
            .signers([])
            .rpc({
                skipPreflight: false,
                commitment: 'confirmed', //default commitment: confirmed
                preflightCommitment: 'confirmed',
                maxRetries: 0,
                minContextSlot: undefined,
            });
    }

    public async initPool(
        poolName: string,
        poolMint: anchor.web3.PublicKey,
        stable: boolean,
        stableMint: PublicKey,
        iconId: number,
        tagsMask: number,
        config: PoolConfigAccount,
    ) {
        const [pda, _] = BumpinUtils.getBumpinStatePda(this.program);
        let params: InitializePoolParams = {
            name: BumpinUtils.encodeString(poolName),
            stableMintKey: BumpinUtils.encodeString(stableMint.toString()),
            poolConfig: config,
            stable: stable,
        };
        await this.program.methods
            .initializePool(params)
            .accounts({
                poolMint,
                bumpSigner: pda,
            })
            .signers([])
            .rpc(BumpinUtils.getDefaultConfirmOptions());
    }

    public async DEV_TEST_ONLY__INIT_ORACLE(
        initPrice: number,
        confidence: number,
        expo: number,
    ): Promise<anchor.web3.Keypair> {
        let oracleKeypair = anchor.web3.Keypair.generate();
        await BumpinUtils.manualCreateAccount(
            this.provider,
            this.wallet,
            oracleKeypair,
            3312,
            await this.TEST_PYTH.provider.connection.getMinimumBalanceForRentExemption(
                3312,
            ),
            this.TEST_PYTH.programId,
        );
        const conf =
            new BN(confidence!) || new BN((initPrice / 10) * 10 ** -expo);
        await this.TEST_PYTH.methods
            .initialize(new anchor.BN(initPrice), expo, conf)
            .accounts({
                price: oracleKeypair.publicKey,
            })
            .signers([])
            .rpc({
                skipPreflight: true,
                commitment: 'confirmed',
                preflightCommitment: 'confirmed',
                maxRetries: 1,
                minContextSlot: undefined,
            });
        return oracleKeypair;
    }

    public async setPrice(oraclePublicKey: PublicKey, price: BN) {
        await this.TEST_PYTH.methods
            .setPrice(price, new BN(0))
            .accounts({
                price: oraclePublicKey,
            })
            .signers([])
            .rpc();
    }

    public async initTradeToken(
        tradeTokenName: string,
        tradeTokenMint: string,
        discount: number,
        liquidationFactor: number,
        exponent: number,
        trueOraclePublicKey: string | undefined,
    ): Promise<PublicKey> {
        const tradeTokenMintPublicKey = new PublicKey(tradeTokenMint);
        let oraclePublicKey: PublicKey;
        if (trueOraclePublicKey) {
            oraclePublicKey = new PublicKey(trueOraclePublicKey);
        } else {
            oraclePublicKey = (
                await this.DEV_TEST_ONLY__INIT_ORACLE(70000, 1.0, exponent)
            ).publicKey;
        }

        const s = BumpinUtils.encodeString(tradeTokenName);
        let [pda, nonce] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods
            .initializeTradeToken(discount, s, liquidationFactor)
            .accounts({
                tradeTokenMint: tradeTokenMintPublicKey,
                oracle: oraclePublicKey,
                bumpSigner: pda,
            })
            .signers([])
            .rpc(BumpinUtils.getDefaultConfirmOptions());
        return oraclePublicKey;
    }

    public async initRewards(
        pool_index: number,
        poolMint: string,
        daoReward: string,
    ): Promise<void> {
        let poolMintPublicKey = new PublicKey(poolMint);
        let daoRewardsPublicKey = new PublicKey(daoReward);

        let [pda, nonce] = BumpinUtils.getBumpinStatePda(this.program);
        await this.program.methods
            .initializeRewards(pool_index)
            .accounts({
                poolMint: poolMintPublicKey,
                bumpSigner: pda,
                daoRewardsVault: daoRewardsPublicKey,
            })
            .signers([])
            .rpc(BumpinUtils.getDefaultConfirmOptions());
    }

    public async initMarket(
        poolName: string,
        poolIndex: number,
        stablePoolIndex: number,
        indexMintOracle: anchor.web3.PublicKey,
    ) {
        const [pda, _] = BumpinUtils.getBumpinStatePda(this.program);
        //TODO: params
        let params: InitializeMarketParams = {
            symbol: BumpinUtils.encodeString(poolName),
            tickSize: new BN(1),
            openFeeRate: new BN(1000),
            closeFeeRate: new BN(1000),
            maximumLongOpenInterestCap: new BN(1000),
            maximumShortOpenInterestCap: new BN(1000),
            longShortRatioLimit: new BN(1000),
            longShortOiBottomLimit: new BN(1000),
            maximumLeverage: 1000000,
            minimumLeverage: 10000,
            poolIndex: poolIndex,
            stablePoolIndex: stablePoolIndex,
        };
        await this.program.methods
            .initializeMarket(params)
            .accounts({
                indexMintOracle,
                bumpSigner: pda,
            })
            .signers([])
            .rpc(BumpinUtils.getRootConfirmOptions());
    }

    async sendAndConfirmTransaction(
        instructions: Array<TransactionInstruction>,
    ) {
        let lastBlockHash =
            await this.program.provider.connection.getLatestBlockhash();
        let blockhash = lastBlockHash.blockhash;
        let lastValidBlockHeight = lastBlockHash.lastValidBlockHeight;

        const messageV0 = new TransactionMessage({
            instructions: instructions,
            payerKey: this.wallet.publicKey,
            recentBlockhash: blockhash,
        }).compileToV0Message();
        const transaction = new VersionedTransaction(messageV0);
        let signedTransaction = await this.wallet.signTransaction(transaction);
        const signature = await this.provider.connection.sendTransaction(
            signedTransaction,
        );
        await this.provider.connection.confirmTransaction(
            {
                blockhash,
                lastValidBlockHeight,
                signature,
            },
            'confirmed',
        );
    }
}

export type WrappedInitializePoolParams = {
    poolMint: PublicKey;
    param: InitializePoolParams;
};

export type WrappedInitializeTradeTokenParams = {
    //tradeTokenName: string, tradeTokenMint: string, discount: number, liquidationFactor: number, exponent: number
    tradeTokenName: string;
    tradeTokenMint: string;
    discount: number;
    liquidationFactor: number;
    exponent: number;
    trueOraclePublicKey: string | undefined;
};

export type WrappedInitializeMarketParams = {
    indexMint: PublicKey;
    params: InitializeMarketParams;
};

export type WrappedInitializeRewardsParams = {
    poolIndex: number;
    poolMint: PublicKey;
    daoVault: string;
};
