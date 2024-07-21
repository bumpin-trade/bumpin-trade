import { Connection, PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import {
  TradeTokenAccount,
  TradeTokenBalance,
  UserAccount,
  UserTokenAccount,
  UserTokenStatusAccount,
} from "../typedef";
import { BumpinAccountNotFound, BumpinTokenNotFound } from "../errors";
import { Account, getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { OracleClient , OraclePriceData} from "../oracles/types";
// @ts-ignore
import { isEqual } from "lodash";

export class BumpinTokenUtils {
  public static async getTradeTokenPrice(
    oracle: OracleClient,
    tradeToken: TradeTokenAccount
  ): Promise<OraclePriceData> {
    return await oracle.getOraclePriceData(tradeToken.oracleKey);

  }

  public static async getUserTradeTokenBalance(
    oracle: OracleClient,
    user: UserAccount,
    tradeTokens: TradeTokenAccount[]
  ): Promise<TradeTokenBalance> {
    let totalBalance = {
      tokenNetValue: new BN(0),
      tokenUsedValue: new BN(0),
      tokenBorrowingValue: new BN(0),
    };

    for (let userToken of user.tokens) {
      if (isEqual(userToken.userTokenStatus, UserTokenStatusAccount.INIT)) {
        continue;
      }
      let tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
        userToken.tokenMintKey,
        tradeTokens
      );
      let tokenBalance = await BumpinTokenUtils.getTradeTokenBalance(
        oracle,
        userToken,
        tradeToken
      );
      totalBalance.tokenNetValue = totalBalance.tokenNetValue.add(
        tokenBalance.tokenNetValue
      );
      totalBalance.tokenUsedValue = totalBalance.tokenUsedValue.add(
        tokenBalance.tokenUsedValue
      );
      totalBalance.tokenBorrowingValue = totalBalance.tokenBorrowingValue.add(
        tokenBalance.tokenBorrowingValue
      );
    }
    totalBalance.tokenUsedValue = totalBalance.tokenUsedValue.add(user.hold);
    totalBalance.tokenBorrowingValue = totalBalance.tokenBorrowingValue.add(
      user.hold
    );
    return totalBalance;
  }

  public static async getTradeTokenBalance(
    oracle: OracleClient,
    userToken: UserTokenAccount,
    tradeToken: TradeTokenAccount
  ): Promise<TradeTokenBalance> {
    let priceData = await oracle.getOraclePriceData(tradeToken.oracleKey);
    let tokenNetValue = new BN(0);
    if (userToken.amount.gt(userToken.usedAmount)) {
      tokenNetValue = userToken.amount
        .sub(userToken.usedAmount)
        .toUsd(priceData.price, tradeToken.decimals)
        .mulRate(new BN(tradeToken.discount));
    }

    let tokenBorrowingValue = new BN(0);
    let tokenUsedValue = new BN(0);

    if (userToken.usedAmount.gt(userToken.amount)) {
      tokenBorrowingValue = userToken.usedAmount
        .sub(userToken.amount)
        .sub(userToken.liabilityAmount)
        .mul(priceData.price);

      tokenUsedValue = userToken.usedAmount
        .sub(userToken.amount)
        .mul(priceData.price)
        .mul(new BN(tradeToken.liquidationFactor).add(new BN(1)));
    }
    return {
      tokenNetValue: tokenNetValue,
      tokenUsedValue: tokenUsedValue,
      tokenBorrowingValue: tokenBorrowingValue,
    };
  }

  public static getUserTokenByMintPublicKey(
    mint: PublicKey,
    userTokens: UserTokenAccount[]
  ): UserTokenAccount {
    let userToken = userTokens.find((userToken) => {
      return userToken.tokenMintKey.equals(mint);
    });
    if (userToken === undefined) {
      throw new BumpinAccountNotFound("UserToken: " + mint);
    }
    return userToken;
  }

  public static getTradeTokenByMintPublicKey(
    mint: PublicKey,
    tradeTokens: TradeTokenAccount[]
  ): TradeTokenAccount {
    let tradeToken = tradeTokens.find((tradeToken) => {
      return tradeToken.mintKey.equals(mint);
    });
    if (tradeToken === undefined) {
      throw new BumpinAccountNotFound("TradeToken: " + mint);
    }
    return tradeToken;
  }

  public static getTradeTokenByOraclePublicKey(
    oracle: PublicKey,
    tradeTokens: TradeTokenAccount[]
  ): TradeTokenAccount {
    let tradeToken = tradeTokens.find((tradeToken) => {
      return tradeToken.oracleKey.equals(oracle);
    });
    if (tradeToken === undefined) {
      throw new BumpinAccountNotFound("TradeToken: " + oracle);
    }
    return tradeToken;
  }

  public static async getTokenAccountFromWalletAndKey(
    connection: Connection,
    walletPublicKey: PublicKey,
    tokenAccountKey: PublicKey
  ): Promise<Account> {
    const walletPubKey = new PublicKey(walletPublicKey);
    const tokenAccounts = await connection.getTokenAccountsByOwner(
      walletPubKey,
      {
        programId: TOKEN_PROGRAM_ID,
      }
    );

    for (let accountInfo of tokenAccounts.value) {
      const accountPubKey: PublicKey = accountInfo.pubkey;
      if (accountPubKey.toString() === tokenAccountKey.toString()) {
        return await getAccount(connection, accountPubKey);
      }
    }
    throw new BumpinAccountNotFound("TokenAccount: " + tokenAccountKey);
  }

  public static async getTokenAccountFromWalletAndMintKey(
    connection: Connection,
    walletPublicKey: PublicKey,
    mintPublicKey: PublicKey
  ): Promise<Account> {
    const walletPubKey = new PublicKey(walletPublicKey);
    const mintPubKey = new PublicKey(mintPublicKey);
    const tokenAccounts = await connection.getTokenAccountsByOwner(
      walletPubKey,
      {
        programId: TOKEN_PROGRAM_ID,
      }
    );

    for (let accountInfo of tokenAccounts.value) {
      const accountPubKey = accountInfo.pubkey;
      const tokenAccount = await getAccount(connection, accountPubKey);

      if (tokenAccount.mint.equals(mintPubKey)) {
        return tokenAccount;
      }
    }
    throw new BumpinAccountNotFound("TokenAccount: " + mintPublicKey);
  }

  public static async getTokenBalanceFromWallet(
    connection: Connection,
    walletPublicKey: PublicKey,
    mintPublicKey: PublicKey
  ): Promise<bigint> {
    const walletPubKey = new PublicKey(walletPublicKey);
    const mintPubKey = new PublicKey(mintPublicKey);
    const tokenAccounts = await connection.getTokenAccountsByOwner(
      walletPubKey,
      {
        programId: TOKEN_PROGRAM_ID,
      }
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
