import {PublicKey} from '@solana/web3.js';
import * as fs from 'fs';
import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {ACCOUNT_SIZE} from "@solana/spl-token";

export class Utils {
    programId: PublicKey;
    bump_state_pk: PublicKey;
    bump_state_nonce: number;


    constructor(program: { programId: PublicKey }) {
        this.programId = program.programId;
        const [bump_state_pk, bump_state_nonce] = PublicKey.findProgramAddressSync(
            [Buffer.from("bump_state")],
            this.programId
        );
        this.bump_state_pk = bump_state_pk;
        this.bump_state_nonce = bump_state_nonce;
    }


    public async new_user(secretKey?: Uint8Array, lamportMultiplier: number = 100.0): Promise<anchor.web3.Keypair> {
        //TODO: Do better, maybe can switch to a different provider
        const provider = anchor.AnchorProvider.local();
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
        const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
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

        const tx = await program.methods.initializeState(
            param
        ).accounts( {
            admin: admin.publicKey,
        }).signers([admin]).rpc();
        console.log("Your transaction signature", tx);
    }


    public async airdrop_lamports(provider: anchor.Provider, receiver: PublicKey, lamports: number) {
        const airdropSignature = await provider.connection.requestAirdrop(receiver, lamports); // Request 2x lamports for safety
        await provider.connection.confirmTransaction(airdropSignature);
    }


    private read_json_from_file(file_path: string) {
        const paramsData = fs.readFileSync(file_path, 'utf8');
        return JSON.parse(paramsData);
    }

    public get_bump_state_pk(): PublicKey {
        return this.bump_state_pk;
    }

    public get_bump_state_nonce(): number {
        return this.bump_state_nonce;
    }
}