import {AccountMeta, PublicKey} from '@solana/web3.js';
import {
    InnerPlaceOrderParams,
    Market,
    OrderSide,
    OrderType,
    PlaceOrderParams,
    Pool,
    PositionSide,
    StopType,
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
    BumpinInvalidParameter,
    BumpinSubscriptionFailed,
    BumpinSupplyInsufficient,
    BumpinTokenAccountUnexpected,
    BumpinValueInsufficient
} from "../errors";
import {DataAndSlot} from "../account/types";
import {OracleClient} from "../oracles/types";
import {BumpinTokenUtils} from "../utils/token";
import {BumpinPositionUtils} from "../utils/position";
import {BumpinPoolUtils} from "../utils/pool";
import {Account} from "@solana/spl-token";
import BigNumber from "bignumber.js";
import {BumpinMarketUtils} from "../utils/market";

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


    public async portfolioStake(size: number, tradeToken: TradeToken, allTradeTokens: TradeToken[], pool: Pool, allMarkets: Market[], sync: boolean = false): Promise<void> {
        let user = await this.getUser(sync);
        let amount = BumpinUtils.size2Amount(new BigNumber(size), tradeToken.decimals);
        let stake_value = await this.checkStakeAmountFulfilRequirements(amount, tradeToken, pool);
        let availableValue = await this.getUserAvailableValue(user, allTradeTokens);
        if (!availableValue.gt(stake_value)) {
            throw new BumpinValueInsufficient(amount, availableValue)
        }

        let remainingAccounts = this.getUserRemainingAccounts(await this.getUser(), allTradeTokens);
        let markets = BumpinMarketUtils.getMarketsByPoolKey(pool.key, allMarkets);
        for (let market of markets) {
            remainingAccounts.push({
                pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
                isWritable: true,
                isSigner: false,
            });
        }

        await this.program.methods.portfolioStake(
            pool.index, tradeToken.index, amount
        ).accounts(
            {
                authority: this.publicKey,
                bumpSigner: (await this.getState()).bumpSigner,
            }
        ).remainingAccounts(remainingAccounts)
            .signers([]).rpc();
    }

    public async walletStake(size: number, tradeToken: TradeToken, allTradeTokens: TradeToken[], wallet: PublicKey, pool: Pool, allMarkets: Market[], sync: boolean = false): Promise<void> {
        // let user = await this.getUser(sync);
        let amount = BumpinUtils.size2Amount(new BigNumber(size), tradeToken.decimals);
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

        let markets = BumpinMarketUtils.getMarketsByPoolKey(pool.key, allMarkets);
        for (let market of markets) {
            remainingAccounts.push({
                pubkey: BumpinUtils.getMarketPda(this.program, market.index)[0],
                isWritable: true,
                isSigner: false,
            });
        }

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
        let user = await this.getUser();
        let pool = BumpinPoolUtils.getPoolByMintPublicKey(markets[marketIndex].poolMintKey, pools);
        let stablePool = BumpinPoolUtils.getPoolByMintPublicKey(markets[marketIndex].stablePoolMintKey, pools);
        let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(markets[marketIndex].poolMintKey, tradeTokens);
        let stableTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(markets[marketIndex].stablePoolMintKey, tradeTokens);

        // When trading position by position (Isolated position), userTokenAccount is determined based on the order direction.
        let uta = userTokenAccount;
        if (!param.isPortfolioMargin) {
            let tokenAccount: Account;
            if (isEqual(param.orderSide, OrderSide.LONG)) {
                tokenAccount = await BumpinTokenUtils.getTokenAccountFromWalletAndKey(this.program.provider.connection, wallet, userTokenAccount);
                if (!tokenAccount.mint.equals(pool.mintKey)) {
                    throw new BumpinTokenAccountUnexpected("Pool mint key: " + pool.mintKey.toString(), "Token account mint key: " + tokenAccount.mint.toString());
                }
            } else {
                tokenAccount = await BumpinTokenUtils.getTokenAccountFromWalletAndKey(this.program.provider.connection, wallet, userTokenAccount);
                if (!tokenAccount.mint.equals(stablePool.mintKey)) {
                    throw new BumpinTokenAccountUnexpected("Stable pool mint key: " + stablePool.mintKey.toString(), "Token account mint key: " + tokenAccount.mint.toString());
                }
            }
            uta = tokenAccount.address;
        }

        let remainingAccounts = this.getUserRemainingAccounts(user, tradeTokens);
        remainingAccounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(this.program, stablePool.index)[0],
            isWritable: true,
            isSigner: false,
        });

        remainingAccounts.push({
            pubkey: markets[marketIndex].indexMintOracle,
            isWritable: false,
            isSigner: false,
        });

        remainingAccounts.push({
            pubkey: BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0],
            isWritable: true,
            isSigner: false,
        });
        remainingAccounts.push({
            pubkey: tradeToken.oracleKey,
            isWritable: false,
            isSigner: false,
        });

        let order: InnerPlaceOrderParams = {
            ...param,
            symbol,
            placeTime: new BN(Date.now()),
            marketIndex: marketIndex,
            poolIndex: pool.index,
            stablePoolIndex: stablePool.index,
            tradeTokenIndex: tradeToken.index,
            stableTradeTokenIndex: stableTradeToken.index,
            orderId: new BN(0)
        };

        let tradeTokenPrice = await BumpinTokenUtils.getTradeTokenPrice(this.oracleClient, tradeToken);
        await this.placePerpOrderValidation(order, tradeTokenPrice, markets[marketIndex]);
        await this.program.methods.placeOrder(
            order,
        ).accounts({
            userTokenAccount: uta,
            authority: wallet,
            bumpSigner: (await this.getState()).bumpSigner,
        }).remainingAccounts(remainingAccounts)
            .signers([]).rpc();
    }

    //TODO: recheck this conditions
    async placePerpOrderValidation(order: InnerPlaceOrderParams, tradeTokenPrice: BN, market: Market, sync: boolean = false) {
        let state = await this.getState(sync);
        if (isEqual(order.orderType, OrderType.NONE)) {
            throw new BumpinInvalidParameter("Order type should not be NONE (when placing order)");
        }

        if (isEqual(order.orderSide, OrderSide.NONE)) {
            throw new BumpinInvalidParameter("Order side should not be NONE (when placing order)");
        }

        if (order.size.isZero() && isEqual(order.positionSide, PositionSide.DECREASE)) {
            throw new BumpinInvalidParameter("Order size should not be zero (when placing order with position side decrease)");
        }

        if (isEqual(order.orderType, OrderType.LIMIT) && isEqual(order.positionSide, PositionSide.DECREASE)) {
            throw new BumpinInvalidParameter("Decrease position does not support limit order");
        }

        if (isEqual(order.orderType, OrderType.STOP) && (isEqual(order.stopType, StopType.NONE) || order.triggerPrice.isZero())) {
            throw new BumpinInvalidParameter("Stop order should have stop type(not none) and trigger price(>0)");
        }

        if (isEqual(order.positionSide, PositionSide.INCREASE)) {
            if (order.orderMargin.isZero()) {
                throw new BumpinInvalidParameter("Order margin should not be zero (when placing order with Increase position side)");
            }
        }

        if (order.isPortfolioMargin && (order.orderMargin.isZero() || order.orderMargin.lt(state.minimumOrderMarginUsd))) {
            throw new BumpinInvalidParameter("Order margin should be greater than minimum order margin: " + state.minimumOrderMarginUsd.toString());
        }

        if (!order.isPortfolioMargin && order.orderMargin.mul(tradeTokenPrice).lt(state.minimumOrderMarginUsd)) {
            throw new BumpinInvalidParameter("Order margin should be greater than minimum order margin: " + state.minimumOrderMarginUsd.toString());
        }

        if (order.leverage > market.config.maximumLeverage || order.leverage < market.config.minimumLeverage) {
            throw new BumpinInvalidParameter("Leverage should be between " + market.config.minimumLeverage + " and " + market.config.maximumLeverage);
        }
    }

    public async getUserAvailableValue(user: UserAccount, tradeTokens: TradeToken[]) {

        let balanceOfUserTradeTokens = await BumpinTokenUtils.getUserTradeTokenBalance(this.oracleClient, user, tradeTokens);
        let balanceOfUserPositions = await BumpinPositionUtils.getUserPositionValue(this.oracleClient, user, tradeTokens);
        return balanceOfUserTradeTokens.tokenNetValue
            .add(balanceOfUserPositions.initialMarginUsd)
            .add(user.hold)
            .sub(balanceOfUserTradeTokens.tokenUsedValue)
            .add(
                balanceOfUserPositions.positionUnPnl.gt(new BN(0)) ? new BN(0) : balanceOfUserPositions.positionUnPnl
            )
            .sub(balanceOfUserPositions.initialMarginUsdFromPortfolio)
            .sub(user.hold)
            .sub(balanceOfUserTradeTokens.tokenBorrowingValue)
    }

    public getUserRemainingAccounts(user: UserAccount, allTradeTokens: TradeToken[], isWritable: boolean = false): Array<AccountMeta> {
        let remainingAccounts: Array<AccountMeta> = [];
        for (let token of user.tokens) {
            if (isEqual(token.userTokenStatus, UserTokenStatus.USING)) {
                remainingAccounts.push({
                    pubkey: token.tokenMintKey,
                    isWritable,
                    isSigner: false,
                });
                let target = BumpinTokenUtils.getTradeTokenByMintPublicKey(token.tokenMintKey, allTradeTokens);
                remainingAccounts.push({
                    pubkey: target.oracleKey,
                    isWritable,
                    isSigner: false,
                });
                let pda = BumpinUtils.getTradeTokenPda(this.program, target.index)[0];
                remainingAccounts.push({
                    pubkey: pda,
                    isWritable,
                    isSigner: false,
                });
            }
        }
        return remainingAccounts;
    }

    async checkUnStakeFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<void> {
        let priceData = await this.oracleClient.getOraclePriceData(tradeToken.mintKey);
        let value = amount.toUsd(priceData.price, tradeToken.decimals);
        if (value < pool.config.minimumStakeAmount) {
            throw new BumpinValueInsufficient(pool.config.minimumStakeAmount, value)
        }
    }

    async checkStakeAmountFulfilRequirements(amount: BN, tradeToken: TradeToken, pool: Pool): Promise<BN> {
        let priceData = await this.oracleClient.getPriceData(tradeToken.oracleKey);
        let value = BumpinUtils.toUsdBN(amount, priceData.price, tradeToken.decimals);
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
        return user.stakes.find((value, index, obj) => isEqual(value.userStakeStatus, UserStakeStatus.USING) && value.poolKey.equals(poolKey));
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