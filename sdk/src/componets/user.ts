import {PublicKey} from '@solana/web3.js';
import {
    InnerPlaceOrderParams,
    Market,
    PlaceOrderParams,
    Pool,
    TradeToken,
    UserAccount,
    UserStakeStatus,
    UserTokenStatus
} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
import {isEqual} from 'lodash';
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
import {BumpinTokenUtils} from "../utils/token";
import {BumpinPositionUtils} from "../utils/position";
import {BumpinPoolUtils} from "../utils/pool";

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


    public async portfolioStake(amount: BN, targetTradeToken: TradeToken, allTradeTokens: TradeToken[], pool: Pool, sync: boolean = false): Promise<void> {
        let user = await this.getUser(sync);

        let stake_value = await this.checkStakeAmountFulfilRequirements(amount, targetTradeToken, pool);
        let availableValue = await this.getUserAvailableValue(user, allTradeTokens);
        if (!availableValue.gt(stake_value)) {
            throw new BumpinValueInsufficient(amount, availableValue)
        }

        let remainingAccounts = [];
        for (let token of user.tokens) {
            if (isEqual(token.userTokenStatus, UserTokenStatus.USING)) {
                remainingAccounts.push({
                    pubkey: token.tokenMintKey,
                    isWritable: false,
                    isSigner: false,
                });
                let target = BumpinTokenUtils.getTradeTokenByMintPublicKey(token.tokenMintKey, allTradeTokens);
                remainingAccounts.push({
                    pubkey: target.oracleKey,
                    isWritable: false,
                    isSigner: false,
                });
                let pda = BumpinUtils.getTradeTokenPda(this.program, target.index)[0];
                remainingAccounts.push({
                    pubkey: pda,
                    isWritable: false,
                    isSigner: false,
                });
            }
        }

        await this.program.methods.portfolioStake(
            pool.index, targetTradeToken.index, amount
        ).accounts(
            {
                authority: this.publicKey,
                bumpSigner: (await this.getState()).bumpSigner,
            }
        ).remainingAccounts(remainingAccounts)
            .signers([]).rpc();
    }

    public async walletStake(amount: BN, tradeToken: TradeToken, allTradeTokens: TradeToken[], wallet: PublicKey, pool: Pool, sync: boolean = false): Promise<void> {
        let user = await this.getUser(sync);
        await this.checkStakeAmountFulfilRequirements(amount, tradeToken, pool);
        await this.checkStakeWalletAmountSufficient(amount, wallet, tradeToken);
        let tokenAccount = await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(this.program.provider.connection, wallet, tradeToken.mintKey);

        let remainingAccounts = [];
        remainingAccounts.push({
            pubkey: tradeToken.mintKey,
            isWritable: false,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: tradeToken.oracleKey,
            isWritable: false,
            isSigner: false,
        });
        let pda = BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0];
        remainingAccounts.push({
            pubkey: pda,
            isWritable: false,
            isSigner: false,
        });

        await this.program.methods.walletStake(
            pool.index, tradeToken.index, amount
        ).accounts(
            {
                authority: wallet,
                userTokenAccount: tokenAccount.address,
            }
        ).remainingAccounts(remainingAccounts).signers([]).rpc();
    }

    public async unStake(portfolio: boolean, share: BN, tradeToken: TradeToken, wallet: PublicKey, pool: Pool): Promise<void> {
        let userStake = await this.findUsingStake(pool.key, false);
        if (share.gt(userStake.stakedShare)) {
            throw new BumpinValueInsufficient(userStake.stakedShare, share)
        }
        if (pool.totalSupply.isZero()) {
            throw new BumpinSupplyInsufficient(new BN(share), new BN(0))
        }

        let param = {
            share: share,
            poolIndex: pool.index,
            tradeTokenIndex: tradeToken.index
        };

        if (portfolio) {
            await this.program.methods.portfolioUnStake(
                param
            ).accounts({
                authority: wallet,
            }).signers([]).rpc();
        } else {
            let tokenAccount = await BumpinTokenUtils.getTokenAccountFromWalletAndMintKey(this.program.provider.connection, wallet, tradeToken.mintKey);
            await this.program.methods.walletUnStake(
                param
            ).accounts({
                authority: wallet,
                userTokenAccount: tokenAccount.address,
            }).signers([]).rpc();
        }

    }


    public async placePerpOrder(symbol: number[], marketIndex: number, param: PlaceOrderParams, wallet: PublicKey, pools: Pool[], markets: Market[], tradeTokens: TradeToken[], userTokenAccount: anchor.web3.PublicKey) {
        let pool = BumpinPoolUtils.getPoolByMintPublicKey(markets[marketIndex].poolMintKey, pools);
        let stablePool = BumpinPoolUtils.getPoolByMintPublicKey(markets[marketIndex].stablePoolMintKey, pools);
        // let indexPool = BumpinPoolUtils.getPoolByMintPublicKey(markets[marketIndex].indexMintKey, pools);
        let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(markets[marketIndex].poolMintKey, tradeTokens);
        let indexTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(markets[marketIndex].indexMintKey, tradeTokens);
        let order: InnerPlaceOrderParams = {
            ...param,
            symbol,
            placeTime: new BN(Date.now()),
            marketIndex: marketIndex,
            poolIndex: pool.index,
            stablePoolIndex: stablePool.index,
            tradeTokenIndex: tradeToken.index,
            indexTradeTokenIndex: indexTradeToken.index,
        };
        await this.program.methods.placeOrder(
            order,
        ).accounts({
            userTokenAccount: userTokenAccount,
            authority: wallet,
            bumpSigner: (await this.getState()).bumpSigner,
        }).signers([]).rpc();
    }

    async placePerpOrderValidation(){

    }

    public async getUserAvailableValue(user: UserAccount, tradeTokens: TradeToken[]) {

        let balanceOfUserTradeTokens = await BumpinTokenUtils.getUserTradeTokenBalance(this.oracleClient, user, tradeTokens);
        let balanceOfUserPositions = await BumpinPositionUtils.getUserPositionValue(this.oracleClient, user, tradeTokens);
        return balanceOfUserTradeTokens.tokenNetValue
            .add(balanceOfUserPositions.initialMarginUsdFromPortfolio)
            .add(user.hold)
            .sub(balanceOfUserTradeTokens.tokenUsedValue)
            .add(
                balanceOfUserPositions.positionUnPnl.gt(new BN(0)) ? new BN(0) : balanceOfUserPositions.positionUnPnl
            )
            .sub(balanceOfUserPositions.initialMarginUsdFromPortfolio)
            .sub(balanceOfUserTradeTokens.tokenBorrowingValue)

    }

    async checkUnStakeFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<void> {
        let priceData = await this.oracleClient.getOraclePriceData(tradeToken.mintKey);
        let value = amount.toUsd(priceData.price, tradeToken.decimals);
        if (value < pool.config.minimumStakeAmount) {
            throw new BumpinValueInsufficient(pool.config.minimumStakeAmount, value)
        }
    }

    async checkStakeAmountFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<BN> {
        let priceData = await this.oracleClient.getOraclePriceData(tradeToken.oracleKey);
        let value = amount.toUsd(priceData.price, tradeToken.decimals);
        if (value < pool.config.minimumStakeAmount) {
            throw new BumpinValueInsufficient(pool.config.minimumStakeAmount, value)
        }
        return value;
    }


    async checkStakeWalletAmountSufficient(amount: BN, wallet: PublicKey, tradeToken: TradeToken): Promise<void> {
        let balance = await BumpinTokenUtils.getTokenBalanceFromWallet(this.program.provider.connection, wallet, tradeToken.mintKey);
        let balanceAmount = new BN(balance.toString());
        if (balanceAmount.lt(amount)) {
            throw new BumpinValueInsufficient(amount, balanceAmount)
        }
    }

    public async findUsingStake(poolKey: PublicKey, sync: boolean) {
        let user = await this.getUser(sync);
        return user.stakes.find((value, index, obj) => value.userStakeStatus === UserStakeStatus.USING && value.poolKey === poolKey);
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