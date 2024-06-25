import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";

const TEN = new BN(10)

export function tokenToUsd(tokenAmount: BN, decimals: BN, tokenPrice: BN): BN {
    return tokenAmount.multiple(tokenPrice).mul(TEN.pow(TEN)).div(TEN.pow(decimals));
}