import {Connection, PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";
import {TradeToken, TradeTokenBalance, UserAccount, UserToken, UserTokenStatus} from "../types";
import {BumpinAccountNotFound, BumpinTokenNotFound} from "../errors";
import {Account, getAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {OracleClient} from "../oracles/types";
import "./cal_utils";

export class BumpinTokenUtils {

    public static async getUserTradeTokenBalance(oracle: OracleClient, user: UserAccount, tradeTokens: TradeToken[]): Promise<TradeTokenBalance> {
        let totalBalance = {
            tokenNetValue: new BN(0),
            tokenUsedValue: new BN(0),
            tokenBorrowingValue: new BN(0)
        };

        for (let userToken of user.tokens) {
            if (userToken.userTokenStatus === UserTokenStatus.INIT) {
                continue;
            }
            let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(userToken.tokenMintKey, tradeTokens);
            let tokenBalance = await BumpinTokenUtils.getTradeTokenBalance(oracle, userToken, tradeToken);
            totalBalance.tokenNetValue = totalBalance.tokenNetValue.add(tokenBalance.tokenNetValue);
            totalBalance.tokenUsedValue = totalBalance.tokenUsedValue.add(tokenBalance.tokenUsedValue);
            totalBalance.tokenBorrowingValue = totalBalance.tokenBorrowingValue.add(tokenBalance.tokenBorrowingValue);
        }

        return totalBalance;
    }


    public static async getTradeTokenBalance(oracle: OracleClient, userToken: UserToken, tradeToken: TradeToken): Promise<TradeTokenBalance> {
        let priceData = await oracle.getOraclePriceData(tradeToken.oracleKey);
        let tokenNetValue = new BN(0);
        if (userToken.amount.gt(userToken.usedAmount)) {
            tokenNetValue = userToken.amount.sub(userToken.usedAmount).toUsd(priceData.price, tradeToken.decimals).mulRate(new BN(tradeToken.discount));
        }
        let tokenUsedValue = userToken.usedAmount.sub(userToken.amount).mul(priceData.price).mul(new BN(tradeToken.liquidationFactor).add(new BN(1)));
        let tokenBorrowingValue = new BN(0);
        if (userToken.usedAmount.gt(userToken.amount)) {
            let tokenBorrowing = userToken.usedAmount.sub(userToken.amount).sub(userToken.liabilityAmount);
            if (tokenBorrowing.gt(new BN(0))) {
                tokenBorrowingValue = tokenBorrowing.mul(priceData.price);
            }
        }
        return {
            tokenNetValue: tokenNetValue,
            tokenUsedValue: tokenUsedValue,
            tokenBorrowingValue: tokenBorrowingValue
        }
    }

    public static getUserTokenByMintPublicKey(mint: PublicKey, userTokens: UserToken[]): UserToken {
        let userToken = userTokens.find((userToken) => {
            return userToken.tokenMintKey.equals(mint);
        });
        if (userToken === undefined) {
            throw new BumpinAccountNotFound("UserToken: " + mint);
        }
        return userToken;
    }

    public static getTradeTokenByMintPublicKey(mint: PublicKey, tradeTokens: TradeToken[]): TradeToken {
        let tradeToken = tradeTokens.find((tradeToken) => {
            return tradeToken.mintKey.equals(mint);
        });
        if (tradeToken === undefined) {
            throw new BumpinAccountNotFound("TradeToken: " + mint);
        }
        return tradeToken;
    }


    public static async getTokenAccountFromWallet(connection: Connection, walletPublicKey: PublicKey, mintPublicKey: PublicKey): Promise<Account> {
        const walletPubKey = new PublicKey(walletPublicKey);
        const mintPubKey = new PublicKey(mintPublicKey);
        const tokenAccounts = await connection.getTokenAccountsByOwner(walletPubKey, {
            programId: TOKEN_PROGRAM_ID,
        });

        for (let accountInfo of tokenAccounts.value) {
            const accountPubKey = accountInfo.pubkey;
            const tokenAccount = await getAccount(connection, accountPubKey);

            if (tokenAccount.mint.equals(mintPubKey)) {
                return tokenAccount;
            }
        }
        throw new BumpinAccountNotFound("TokenAccount: " + mintPublicKey);
    }

    public static async getTokenBalanceFromWallet(connection: Connection, walletPublicKey: PublicKey, mintPublicKey: PublicKey): Promise<bigint> {
        const walletPubKey = new PublicKey(walletPublicKey);
        const mintPubKey = new PublicKey(mintPublicKey);
        const tokenAccounts = await connection.getTokenAccountsByOwner(walletPubKey, {
            programId: TOKEN_PROGRAM_ID,
        });

        for (let accountInfo of tokenAccounts.value) {
            const accountPubKey = accountInfo.pubkey;

            const tokenAccount = await getAccount(connection, accountPubKey);

            if (tokenAccount.mint.equals(mintPubKey)) {

                return tokenAccount.amount;
            }
        }
        throw new BumpinTokenNotFound(mintPublicKey);
    }
}