import {BN} from "@coral-xyz/anchor";
import {FIVE} from "../constants/numericConstants";

const TEN = new BN(10)
declare module "@coral-xyz/anchor" {
    interface BN {
        toUsd(tokenPrice: BN, decimals: number): BN;

        toToken(tokenPrice: BN, decimals: BN): BN;

        divWithDecimals(value2: BN, decimals: number): BN;

        mulRate(rate: BN): BN;

        divRate(rate: BN): BN;

        mulSmallRate(rate: BN): BN;

        divSmallRate(rate: BN): BN;
    }
}


BN.prototype.toUsd = function (tokenPrice: BN, decimals: number): BN {
    return this.mul(tokenPrice).mul(TEN.pow(TEN)).div(TEN.pow(new BN(decimals)));
}

BN.prototype.toToken = function (tokenPrice: BN, decimals: number) {
    return this.mul(new BN(decimals)).div(tokenPrice.mul(TEN.pow(new BN(10))));
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
    return this.mull(rate).div(TEN.pow(new BN(18)));
}

BN.prototype.divSmallRate = function (rate: BN): BN {
    return this.div(rate).mul(TEN.pow(new BN(18)));
}


export {};