import BigNumber from "bignumber.js";

export declare module "@coral-xyz/anchor" {
  interface BN {
    toBigNumber(): BigNumber;

    toUsd(tokenPrice: BN, decimals: number): BN;

    toToken(tokenPrice: BN, decimals: number): BN;

    divWithDecimals(decimals: number): BN;

    mulRate(rate: BN): BN;

    divRate(rate: BN): BN;

    mulSmallRate(rate: BN): BN;

    divSmallRate(rate: BN): BN;

    downRate(): BN;

    downSmallRate(): BN;

    downWithDecimals(decimals: number): BN;

    downPrice(): BN;
  }
}



