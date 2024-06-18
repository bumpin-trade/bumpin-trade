"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Utils = void 0;
const web3_js_1 = require("@solana/web3.js");
const fs = __importStar(require("fs"));
const anchor = __importStar(require("@coral-xyz/anchor"));
const spl_token_1 = require("@solana/spl-token");
class Utils {
    constructor(program) {
        this.provider = anchor.AnchorProvider.local();
        this.program = anchor.workspace.BumpinTrade;
        this.programId = program.programId;
        const [bump_state_pk, bump_state_nonce] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bump_state")], this.programId);
        this.bump_state_pk = bump_state_pk;
        this.bump_state_nonce = bump_state_nonce;
    }
    new_user(secretKey_1) {
        return __awaiter(this, arguments, void 0, function* (secretKey, lamportMultiplier = 100.0) {
            //TODO: Do better, maybe can switch to a different provider
            let user;
            if (secretKey) {
                user = anchor.web3.Keypair.fromSecretKey(secretKey);
            }
            else {
                user = anchor.web3.Keypair.generate();
            }
            if (lamportMultiplier > 0.0) {
                const lamports = yield this.provider.connection.getMinimumBalanceForRentExemption(spl_token_1.ACCOUNT_SIZE);
                yield this.airdrop_lamports(this.provider, user.publicKey, lamports * lamportMultiplier);
            }
            return user;
        });
    }
    initialize_state(admin) {
        return __awaiter(this, void 0, void 0, function* () {
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
            yield this.program.methods.initializeState(param).accounts({
                admin: admin.publicKey,
            }).signers([admin]).rpc();
        });
    }
    create_mint_account(mintAuthority_1, payer_1) {
        return __awaiter(this, arguments, void 0, function* (mintAuthority, payer, decimals = 9) {
            const lamports = yield this.provider.connection.getMinimumBalanceForRentExemption(spl_token_1.ACCOUNT_SIZE);
            let account = anchor.web3.Keypair.generate();
            const transaction = new web3_js_1.Transaction();
            transaction.add(web3_js_1.SystemProgram.createAccount({
                fromPubkey: payer.publicKey,
                newAccountPubkey: account.publicKey,
                space: spl_token_1.MintLayout.span,
                lamports,
                programId: spl_token_1.TOKEN_PROGRAM_ID,
            }));
            transaction.add((0, spl_token_1.createInitializeMintInstruction)(account.publicKey, decimals, mintAuthority.publicKey, mintAuthority.publicKey, spl_token_1.TOKEN_PROGRAM_ID));
            yield (0, web3_js_1.sendAndConfirmTransaction)(this.provider.connection, transaction, [payer, account]);
            return account;
        });
    }
    initialize_user(authority, payer) {
        return __awaiter(this, void 0, void 0, function* () {
            const program = anchor.workspace.BumpinTrade;
            yield program.methods.initializeUser().accounts({
                state: this.bump_state_pk,
                authority: authority.publicKey,
                payer: payer.publicKey
            }).signers([authority, payer]).rpc();
        });
    }
    initialize_pool(poolMint, name, admin) {
        return __awaiter(this, void 0, void 0, function* () {
            const poolName = Buffer.from(name, 'utf-8');
            const paddedPoolName = Buffer.concat([poolName, Buffer.alloc(32 - poolName.length)]);
            const paddedPoolNameArray = Array.from(paddedPoolName);
            const program = anchor.workspace.BumpinTrade;
            yield program.methods.initializePool(paddedPoolNameArray).accounts({
                poolMint: poolMint,
                bumpSigner: this.bump_state_pk,
                admin: admin.publicKey,
            }).signers([admin]).rpc();
        });
    }
    airdrop_lamports(provider, receiver, lamports) {
        return __awaiter(this, void 0, void 0, function* () {
            const airdropSignature = yield provider.connection.requestAirdrop(receiver, lamports); // Request 2x lamports for safety
            yield provider.connection.confirmTransaction(airdropSignature);
        });
    }
    read_json_from_file(file_path) {
        const paramsData = fs.readFileSync(file_path, 'utf8');
        return JSON.parse(paramsData);
    }
    get_bump_state_pk() {
        return this.bump_state_pk;
    }
    get_bump_state_nonce() {
        return this.bump_state_nonce;
    }
}
exports.Utils = Utils;
