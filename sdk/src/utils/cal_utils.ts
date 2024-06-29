import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import {FIVE} from "../constants/numericConstants";

const TEN = new BN(10)


export function tokenToUsd(tokenAmount: BN, tokenPrice: BN, decimals: number): BN {
    return tokenAmount.mul(tokenPrice).mul(TEN.pow(TEN)).div(TEN.pow(new BN(decimals)));
}

export function usdToToken(usdAmount: BN, tokenPrice: BN, decimals: number): BN {
    return usdAmount.mul(decimals).div(tokenPrice.mul(TEN.pow(10)));
}

BN.prototype.divWithDecimals = function (value2: BN, decimals: number): BN {
    return this.mul(decimals).div(value2);
}

BN.prototype.mulRate = function (rate: BN): BN {
    return this.mul(rate).div(TEN.pow(FIVE));
}

BN.prototype.divRate = function (rate: BN): BN {
    return this.div(rate).mul(TEN.pow(FIVE));
}

BN.prototype.mulSmallRate = function (rate: BN): BN {
    return this.mull(rate).div(TEN.pow(18));
}

BN.prototype.divSmallRate = function (rate: BN): BN {
    return this.div(rate).mul(TEN.pow(18));
}


export {};