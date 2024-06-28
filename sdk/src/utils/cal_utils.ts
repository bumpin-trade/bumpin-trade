import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";

const TEN = new BN(10)

export function tokenValueInUsd(tokenAmount: BN, tokenPrice: BN, decimals: number): BN {
    return tokenAmount.mul(tokenPrice).mul(TEN.pow(TEN)).div(TEN.pow(new BN(decimals)));
}