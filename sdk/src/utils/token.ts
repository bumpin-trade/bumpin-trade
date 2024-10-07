import { Connection, PublicKey } from '@solana/web3.js';
import { TradeTokenBalance, UserTokenAccount } from '../typedef';
import { BumpinAccountNotFound, BumpinTokenNotFound } from '../errors';
import { Account, getAccount, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { OracleClient, OraclePriceData } from '../oracles/types';
// @ts-ignore
import { isEqual } from 'lodash';
import { TradeToken, User, UserToken, UserTokenStatus } from '../beans/beans';
import { TradeTokenComponent } from '../componets/tradeToken';
import BigNumber from 'bignumber.js';

export class BumpinTokenUtils {
    public static async getUserAllTokenEquity(
        tradeTokenComponent: TradeTokenComponent,
        user: User,
        tradeTokens: TradeToken[],
    ): Promise<BigNumber> {
        let equity = new BigNumber(0);
        for (let token of user.tokens) {
            if (!isEqual(token.userTokenStatus, UserTokenStatus.INIT)) {
                let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
                    token.tokenMintKey,
                    tradeTokens,
                );
                const price =
                    tradeTokenComponent.getTradeTokenPricesByOracleKey(
                        tradeToken.oracleKey,
                        1,
                    )[0].price!;
                equity = equity.plus(
                    token.amount
                        .minus(token.liabilityAmount)
                        .multipliedBy(price),
                );
            }
        }
        return equity;
    }

    public static async getTradeTokenPrice(
        oracle: OracleClient,
        tradeToken: TradeToken,
    ): Promise<OraclePriceData> {
        return await oracle.getOraclePriceData(tradeToken.oracleKey);
    }

    public static async getUserTradeTokenBalance(
        tradeTokenComponent: TradeTokenComponent,
        user: User,
        tradeTokens: TradeToken[],
    ): Promise<TradeTokenBalance> {
        let totalBalance = {
            tokenNetValue: BigNumber(0),
            tokenUsedValue: BigNumber(0),
            tokenBorrowingValue: BigNumber(0),
        };

        for (let userToken of user.tokens) {
            if (isEqual(userToken.userTokenStatus, UserTokenStatus.INIT)) {
                continue;
            }
            let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
                userToken.tokenMintKey,
                tradeTokens,
            );
            let tokenBalance = await BumpinTokenUtils.getTradeTokenBalance(
                tradeTokenComponent,
                userToken,
                tradeToken,
            );
            totalBalance.tokenNetValue = totalBalance.tokenNetValue.plus(
                tokenBalance.tokenNetValue,
            );
            totalBalance.tokenUsedValue = totalBalance.tokenUsedValue.plus(
                tokenBalance.tokenUsedValue,
            );
            totalBalance.tokenBorrowingValue =
                totalBalance.tokenBorrowingValue.plus(
                    tokenBalance.tokenBorrowingValue,
                );
        }
        totalBalance.tokenUsedValue = totalBalance.tokenUsedValue.plus(
            user.hold,
        );
        totalBalance.tokenBorrowingValue =
            totalBalance.tokenBorrowingValue.plus(user.hold);
        return totalBalance;
    }

    public static async getTradeTokenBalance(
        tradeTokenComponent: TradeTokenComponent,
        userToken: UserToken,
        tradeToken: TradeToken,
    ): Promise<TradeTokenBalance> {
        const price = tradeTokenComponent.getTradeTokenPricesByOracleKey(
            tradeToken.oracleKey,
            1,
        )[0].price!;
        let tokenNetValue = BigNumber(0);
        if (userToken.amount.gt(userToken.usedAmount)) {
            tokenNetValue = userToken.amount
                .minus(userToken.usedAmount)
                .multipliedBy(price)
                .multipliedBy(tradeToken.discount);
        }

        let tokenBorrowingValue = BigNumber(0);
        let tokenUsedValue = BigNumber(0);

        if (
            userToken.usedAmount
                .minus(userToken.liabilityAmount)
                .gt(userToken.amount)
        ) {
            tokenBorrowingValue = userToken.usedAmount
                .minus(userToken.amount)
                .minus(userToken.liabilityAmount)
                .multipliedBy(price);
        }
        if (userToken.usedAmount.gt(userToken.amount)) {
            tokenUsedValue = userToken.usedAmount
                .minus(userToken.amount)
                .multipliedBy(price)
                .multipliedBy(tradeToken.liquidationFactor + 1);
        }
        return {
            tokenNetValue: tokenNetValue,
            tokenUsedValue: tokenUsedValue,
            tokenBorrowingValue: tokenBorrowingValue,
        };
    }

    public static getUserTokenByMintPublicKey(
        mint: PublicKey,
        userTokens: UserToken[],
    ): UserToken {
        let userToken = userTokens.find((userToken) => {
            return userToken.tokenMintKey.equals(mint);
        });
        if (userToken === undefined) {
            throw new BumpinAccountNotFound('UserToken: ' + mint);
        }
        return userToken;
    }

    public static getTradeTokenByMintPublicKey(
        mint: PublicKey,
        tradeTokens: TradeToken[],
    ): TradeToken {
        let tradeToken = tradeTokens.find((tradeToken) => {
            return tradeToken.mintKey.equals(mint);
        });
        if (tradeToken === undefined) {
            throw new BumpinAccountNotFound('TradeToken: ' + mint);
        }
        return tradeToken;
    }

    public static getTradeTokenByOraclePublicKey(
        oracle: PublicKey,
        tradeTokens: TradeToken[],
    ): TradeToken {
        let tradeToken = tradeTokens.find((tradeToken) => {
            return tradeToken.oracleKey.equals(oracle);
        });
        if (tradeToken === undefined) {
            throw new BumpinAccountNotFound('TradeToken: ' + oracle);
        }
        return tradeToken;
    }

    public static async getTokenAccountFromWalletAndKey(
        connection: Connection,
        walletPublicKey: PublicKey,
        tokenAccountKey: PublicKey,
    ): Promise<Account> {
        const walletPubKey = new PublicKey(walletPublicKey);
        const tokenAccounts = await connection.getTokenAccountsByOwner(
            walletPubKey,
            {
                programId: TOKEN_PROGRAM_ID,
            },
        );

        for (let accountInfo of tokenAccounts.value) {
            const accountPubKey: PublicKey = accountInfo.pubkey;
            if (accountPubKey.toString() === tokenAccountKey.toString()) {
                return await getAccount(connection, accountPubKey);
            }
        }
        throw new BumpinAccountNotFound('TokenAccount: ' + tokenAccountKey);
    }

    public static async getTokenAccountFromWalletAndMintKey(
        connection: Connection,
        walletPublicKey: PublicKey,
        mintPublicKey: PublicKey,
    ): Promise<Account> {
        const walletPubKey = new PublicKey(walletPublicKey);
        const mintPubKey = new PublicKey(mintPublicKey);
        const tokenAccounts = await connection.getTokenAccountsByOwner(
            walletPubKey,
            {
                programId: TOKEN_PROGRAM_ID,
            },
        );

        for (let accountInfo of tokenAccounts.value) {
            const accountPubKey = accountInfo.pubkey;
            const tokenAccount = await getAccount(connection, accountPubKey);

            if (tokenAccount.mint.equals(mintPubKey)) {
                return tokenAccount;
            }
        }
        throw new BumpinAccountNotFound('TokenAccount: ' + mintPublicKey);
    }

    public static async getTokenBalanceFromWallet(
        connection: Connection,
        walletPublicKey: PublicKey,
        mintPublicKey: PublicKey,
    ): Promise<bigint> {
        const walletPubKey = new PublicKey(walletPublicKey);
        const mintPubKey = new PublicKey(mintPublicKey);
        const tokenAccounts = await connection.getTokenAccountsByOwner(
            walletPubKey,
            {
                programId: TOKEN_PROGRAM_ID,
            },
        );

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
