import {PublicKey} from '@solana/web3.js';
import {Pool, State, TradeToken, UserAccount, UserStakeStatus} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {BN, Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {PollingUserAccountSubscriber} from "../account/pollingUserAccountSubscriber";
import {
    BumpinAccountNotFound,
    BumpinSubscriptionFailed,
    BumpinSupplyInsufficient,
    BumpinValueInsufficient
} from "../errors";
import {DataAndSlot} from "../account/types";
import {OracleClient} from "../oracles/types";
import {tokenValueInUsd} from "../utils/cal_utils";

export class UserComponent extends Component {
    publicKey: PublicKey;
    oracleClient: OracleClient
    program: Program<BumpinTrade>;
    userAccountSubscriber: PollingUserAccountSubscriber;

    constructor(publicKey: PublicKey, oracleClient: OracleClient, bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        this.publicKey = publicKey;
        this.oracleClient = oracleClient;
        this.program = program;
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.publicKey.toBuffer()]);
        this.userAccountSubscriber = new PollingUserAccountSubscriber(this.program, pda, bulkAccountLoader);
    }

    public async subscribe() {
        await this.userAccountSubscriber.subscribe();
    }

    public async unsubscribe() {
        await this.userAccountSubscriber.unsubscribe();
    }


    public async portfolioStake(amount: BN, tradeToken: TradeToken, state: State, pool: Pool): Promise<void> {
        await this.checkStakeAmountFulfilRequirements(amount, tradeToken, pool);
    }

    public async walletStake(amount: BN, tradeToken: TradeToken, wallet: PublicKey, state: State, pool: Pool): Promise<void> {
        await this.checkStakeAmountFulfilRequirements(amount, tradeToken, pool);
        await this.checkStakeWalletAmountSufficient(amount, wallet, tradeToken);
        let tokenAccount = await BumpinUtils.getTokenAccountFromWallet(this.program.provider.connection, wallet, tradeToken.mint);
        let param = {
            requestTokenAmount: amount,
            poolIndex: pool.poolIndex,
            tradeTokenIndex: tradeToken.tokenIndex
        };
        await this.program.methods.walletStake(
            param
        ).accounts(
            {
                authority: wallet,
                userTokenAccount: tokenAccount.address,
            }
        ).signers([]).rpc();
    }


    public async unStake(portfolio: boolean, share: BN, tradeToken: TradeToken, wallet: PublicKey, state: State, pool: Pool): Promise<void> {
        let userStake = await this.findUsingStake(pool.poolKey, false);
        if (share.gt(userStake.stakedShare)) {
            throw new BumpinValueInsufficient(userStake.stakedShare, share)
        }
        if (pool.totalSupply.isZero()) {
            throw new BumpinSupplyInsufficient(new BN(share), new BN(0))
        }

        let param = {
            share: share,
            poolIndex: pool.poolIndex,
            tradeTokenIndex: tradeToken.tokenIndex
        };

        if (portfolio) {
            await this.program.methods.portfolioUnStake(
                param
            ).accounts({
                authority: wallet,
            }).signers([]).rpc();
        } else {
            let tokenAccount = await BumpinUtils.getTokenAccountFromWallet(this.program.provider.connection, wallet, tradeToken.mint);
            await this.program.methods.walletUnStake(
                param
            ).accounts({
                authority: wallet,
                userTokenAccount: tokenAccount.address,
            }).signers([]).rpc();
        }

    }

    async checkUnStakeFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<void> {
        let priceData = await this.oracleClient.getOraclePriceData(tradeToken.mint);
        let value = tokenValueInUsd(amount, priceData.price, tradeToken.decimals);
        if (value < pool.poolConfig.miniStakeAmount) {
            throw new BumpinValueInsufficient(pool.poolConfig.miniStakeAmount, value)
        }
    }

    async checkStakeAmountFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<void> {
        let priceData = await this.oracleClient.getOraclePriceData(tradeToken.mint);
        let value = tokenValueInUsd(amount, priceData.price, tradeToken.decimals);
        if (value < pool.poolConfig.miniStakeAmount) {
            throw new BumpinValueInsufficient(pool.poolConfig.miniStakeAmount, value)
        }
    }


    async checkStakeWalletAmountSufficient(amount: BN, wallet: PublicKey, tradeToken: TradeToken): Promise<void> {
        let balance = await BumpinUtils.getTokenBalanceFromWallet(this.program.provider.connection, wallet, tradeToken.mint);
        let balanceAmount = new BN(balance.toString());
        if (balanceAmount.lt(amount)) {
            throw new BumpinValueInsufficient(amount, balanceAmount)
        }
    }

    public async findUsingStake(poolKey: PublicKey, sync: boolean) {
        let user = await this.getUser(sync);
        return user.userStakes.find((value, index, obj) => value.userStakeStatus === UserStakeStatus.USING && value.poolKey === poolKey);
    }

    public async getUser(sync: boolean = false): Promise<UserAccount> {
        let userWithSlot = await this.getUserWithSlot(sync);
        return userWithSlot.data;
    }


    public async getUserWithSlot(sync: boolean = false): Promise<DataAndSlot<UserAccount>> {
        if (!this.userAccountSubscriber || !this.userAccountSubscriber.isSubscribed) {
            throw new BumpinSubscriptionFailed("User")
        }
        if (sync) {
            await this.userAccountSubscriber.fetch();
        }
        let userAccount = this.userAccountSubscriber.getAccountAndSlot();
        if (!userAccount) {
            throw new BumpinAccountNotFound("User")
        }
        return userAccount;
    }
}