import {
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionMessage,
    VersionedTransaction
} from '@solana/web3.js';
import * as fs from 'fs';
import * as anchor from "@coral-xyz/anchor";
import {Program, Provider} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {Pyth} from "../../target/types/pyth";
import {
    ACCOUNT_SIZE,
    createInitializeMintInstruction,
    getOrCreateAssociatedTokenAccount,
    MintLayout,
    mintToChecked,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import BN from "bn.js";
import {PlaceOrderParams} from "../exchange/order_params";
import {BumpinMarket, BumpinPlayer, BumpinTradeToken} from "../exchange/exchange";

export class Utils {
    provider = anchor.AnchorProvider.local();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;


    public getStatePda(): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("bump_state")],
            this.program.programId
        );
    }

    public async new_user(provider: anchor.AnchorProvider, secretKey?: Uint8Array, lamportMultiplier: number = 100.0,): Promise<anchor.web3.Keypair> {
        //TODO: Do better, maybe can switch to a different provider
        let user: anchor.web3.Keypair | undefined;
        if (secretKey) {
            user = anchor.web3.Keypair.fromSecretKey(secretKey);

        } else {
            user = anchor.web3.Keypair.generate();

        }

        if (lamportMultiplier > 0.0) {
            const lamports = await provider.connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE);
            await this.airdrop_lamports(provider, user.publicKey, lamports * lamportMultiplier);
        }
        return user;
    }


    public async initialize_state(admin: anchor.web3.Keypair): Promise<void> {
        const params = this.read_json_from_file('tests/params/states/state_params.json');
        const param = {
            minOrderMarginUsd: new anchor.BN(params.min_order_margin_usd),
            maxMaintenanceMarginRate: new anchor.BN(params.max_maintenance_margin_rate),
            fundingFeeBaseRate: new anchor.BN(params.funding_fee_base_rate),
            maxFundingBaseRate: new anchor.BN(params.max_funding_base_rate),
            tradingFeeStakingRewardsRatio: new anchor.BN(params.trading_fee_staking_rewards_ratio),
            tradingFeePoolRewardsRatio: new anchor.BN(params.trading_fee_pool_rewards_ratio),
            tradingFeeUsdPoolRewardsRatio: new anchor.BN(params.trading_fee_usd_pool_rewards_ratio),
            borrowingFeeStakingRewardsRatio: new anchor.BN(params.borrowing_fee_staking_rewards_ratio),
            borrowingFeePoolRewardsRatio: new anchor.BN(params.borrowing_fee_pool_rewards_ratio),
            minPrecisionMultiple: new anchor.BN(params.min_precision_multiple),
            mintFeeStakingRewardsRatio: new anchor.BN(params.mint_fee_staking_rewards_ratio),
            mintFeePoolRewardsRatio: new anchor.BN(params.mint_fee_pool_rewards_ratio),
            redeemFeeStakingRewardsRatio: new anchor.BN(params.redeem_fee_staking_rewards_ratio),
            redeemFeePoolRewardsRatio: new anchor.BN(params.redeem_fee_pool_rewards_ratio),
            poolRewardsIntervalLimit: new anchor.BN(params.pool_rewards_interval_limit),
            initFee: new anchor.BN(params.init_fee),
            stakingFeeRewardRatio: new anchor.BN(params.staking_fee_reward_ratio),
            poolFeeRewardRatio: new anchor.BN(params.pool_fee_reward_ratio)
        };

        await this.program.methods.initializeState(
            param
        ).accounts({
            admin: admin.publicKey,
        }).signers([admin]).rpc();
    }

    public async create_mint_account(mintAuthority: anchor.web3.Keypair, payer: anchor.web3.Keypair, decimals: number = 9): Promise<anchor.web3.Keypair> {
        const lamports = await this.provider.connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE);
        let account = anchor.web3.Keypair.generate();
        const transaction = new Transaction();
        transaction.add(
            SystemProgram.createAccount({
                fromPubkey: payer.publicKey,
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
                mintAuthority.publicKey,
                mintAuthority.publicKey,
                TOKEN_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            this.provider.connection,
            transaction,
            [payer, account]
        );

        return account;

    }


    public async initialize_user(authority: anchor.web3.Keypair, payer: anchor.web3.Keypair): Promise<void> {
        let [pda, nonce] = this.getStatePda();
        const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
        await program.methods.initializeUser().accounts({
            state: pda,
            authority: authority.publicKey,
            payer: payer.publicKey
        }).signers([authority, payer]).rpc();
    }


    public async initialize_pool(program: Program<BumpinTrade>, poolMint: PublicKey, name: string, admin: anchor.web3.Keypair): Promise<void> {
        let [pda, nonce] = this.getStatePda();
        const poolName = Buffer.from(name, 'utf-8');
        const paddedPoolName = Buffer.concat([poolName, Buffer.alloc(32 - poolName.length)]);
        const paddedPoolNameArray = Array.from(paddedPoolName);

        await program.methods.initializePool(
            paddedPoolNameArray,
        ).accounts({
            poolMint: poolMint,
            bumpSigner: pda,
            admin: admin.publicKey,
        }).signers([admin]).rpc();
    }

    public async initialize_oracle(oracle: PublicKey, initPrice: number, confidence = undefined, expo = -4): Promise<void> {
        const conf = new BN(confidence) || new BN((initPrice / 10) * 10 ** -expo);
        await this.programPyth.methods.initialize(
            new anchor.BN(initPrice),
            expo,
            conf
        ).accounts({
            price: oracle,
        }).rpc();
    }


    public async initialize_trade_token(tradeTokenName: string, tradeTokenMint: PublicKey, admin: anchor.web3.Keypair, oracle: PublicKey, discount: BN, liquidationFactor: BN): Promise<void> {
        const s = this.string2Padded32Bytes(tradeTokenName);
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

    public async initialize_market(symbol: string, admin: anchor.web3.Keypair, pool: PublicKey, stablePool: PublicKey, indexMint: PublicKey): Promise<void> {
        const s = this.string2Padded32Bytes(symbol);
        const [state, _] = this.getStatePda();
        await this.program.methods.initializeMarket(
            s
        ).accounts({
            pool,
            stablePool,
            indexMint,
            admin: admin.publicKey,
            bumpSigner: state,
        }).signers([admin]).rpc();
    }


    public async deposit(authority: anchor.web3.Keypair, userTokenAccount: PublicKey, tokenIndex: number, amount: BN): Promise<void> {
        await this.program.methods.deposit(
            tokenIndex, amount
        ).accounts({
            authority: authority.publicKey,
            userTokenAccount
        }).signers([authority]).rpc();
    }


    public async placePerpOrder(player: BumpinPlayer,
                                oracle: PublicKey,
                                param: PlaceOrderParams
    ): Promise<void> {
        await this.program.methods.placeOrder(
            param,
        ).accounts({
            user: player.getPda()[0],
            authority: player.user.publicKey,
            bumpSigner: this.getStatePda()[0],
        }).remainingAccounts([{
            pubkey: oracle,
            isWritable: false,
            isSigner: false
        }]).signers([player.user]).rpc();
    }

    public async manualCreateAccount(provider: Provider, fromPk: anchor.web3.Keypair, newAccountPk: anchor.web3.Keypair, space: number, lamports: number, programId: PublicKey) {
        let i = anchor.web3.SystemProgram.createAccount({
            fromPubkey: fromPk.publicKey,
            newAccountPubkey: newAccountPk.publicKey,
            space: space,
            lamports: lamports,
            programId: programId,
        });
        let lastBlockHash = await provider.connection
            .getLatestBlockhash();
        let blockhash = lastBlockHash.blockhash;
        let lastValidBlockHeight = lastBlockHash.lastValidBlockHeight;


        // create v0 compatible message
        const messageV0 = new TransactionMessage({
            instructions: [i],
            payerKey: fromPk.publicKey,
            recentBlockhash: blockhash,
        }).compileToV0Message();
        const transaction = new VersionedTransaction(messageV0);
        transaction.sign([fromPk, newAccountPk]);
        const signature = await provider.connection.sendTransaction(transaction);
        await provider.connection.confirmTransaction({
            blockhash,
            lastValidBlockHeight,
            signature
        });

    }


    public async createTokenAccount(provider: Provider, payer: anchor.web3.Keypair, mint: PublicKey, owner: PublicKey) {
        return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer,
            mint,
            owner
        );
    }

    public async mintTo(provider: Provider, payer: anchor.web3.Keypair, mint: PublicKey, destination: PublicKey, amount: number, decimals: number) {
        await mintToChecked(provider.connection, payer, mint, destination, payer, amount, decimals);
    }

    public async airdrop_lamports(provider: anchor.Provider, receiver: PublicKey, lamports: number) {
        const airdropSignature = await provider.connection.requestAirdrop(receiver, lamports); // Request 2x lamports for safety
        await provider.connection.confirmTransaction(airdropSignature);
    }

    public string2Padded32Bytes(str: string): number[] {
        const buffer = Buffer.from(str, 'utf-8');
        const paddedBuffer = Buffer.concat([buffer, Buffer.alloc(32 - buffer.length)]);
        return Array.from(paddedBuffer);
    }

    private read_json_from_file(file_path: string) {
        const paramsData = fs.readFileSync(file_path, 'utf8');
        return JSON.parse(paramsData);
    }


}