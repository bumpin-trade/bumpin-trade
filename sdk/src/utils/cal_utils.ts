import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import {FIVE} from "../constants/numericConstants";

const TEN = new BN(10)


export function tokenToUsd(tokenAmount: BN, tokenPrice: BN, decimals: number): BN {
    return tokenAmount.mul(tokenPrice).mul(TEN.pow(TEN)).div(TEN.pow(new BN(decimals)));
}

export function usdToToken(usdAmount: BN, tokenPrice: BN, decimals: number): BN {
    return usdAmount.mul(decimals).div(tokenPrice.mul(TEN.pow(10)));
}

export function mulRate(value: BN, rate: number): BN {
    return value.mull(rate).div(TEN.pow(FIVE));
}

export function divRate(value: BN, rate: number): BN {
    return value.div(rate).mul(TEN.pow(FIVE));
}

export function mulSmallRate(value: BN, rate: number): BN {
    return value.mull(rate).div(TEN.pow(18));
}

export function divSmallRate(value: BN, rate: number): BN {
    return value.div(rate).mul(TEN.pow(18));
}

export function divWithDecimals(value1: BN, value2: BN, decimals: number): BN {
    return value1.mul(decimals).div(value2);
}