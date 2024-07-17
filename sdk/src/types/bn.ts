import { BN } from "@coral-xyz/anchor";
import { FIVE, TEN } from "../constants/numericConstants";
import BigNumber from "bignumber.js";

declare module "@coral-xyz/anchor" {
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

BN.prototype.toBigNumber = function (): BigNumber {
  return new BigNumber(this.toString());
};

BN.prototype.toUsd = function (tokenPrice: BN, decimals: number): BN {
  return this.mul(tokenPrice)
    .mul(TEN.pow(TEN))
    .div(TEN.pow(new BN(decimals)));
};

BN.prototype.toToken = function (tokenPrice: BN, decimals: number): BN {
  return this.mul(new BN(decimals)).div(tokenPrice.mul(TEN.pow(new BN(10))));
};
BN.prototype.divWithDecimals = function (decimals: number): BN {
  return this.div(TEN.pow(new BN(decimals)));
};

BN.prototype.mulRate = function (rate: BN): BN {
  return this.mul(rate).div(TEN.pow(FIVE));
};

BN.prototype.divRate = function (rate: BN): BN {
  return this.div(rate).mul(TEN.pow(FIVE));
};

BN.prototype.mulSmallRate = function (rate: BN): BN {
  return this.mul(rate).div(TEN.pow(new BN(18)));
};

BN.prototype.divSmallRate = function (rate: BN): BN {
  return this.div(rate).mul(TEN.pow(new BN(18)));
};

BN.prototype.downRate = function (): BN {
  return this.div(TEN.pow(new BN(FIVE)));
};

BN.prototype.downSmallRate = function (): BN {
  return this.div(TEN.pow(new BN(18)));
};

BN.prototype.downWithDecimals = function (decimals: number): BN {
  return this.div(TEN.pow(new BN(decimals)));
};

BN.prototype.downPrice = function (): BN {
  return this.div(TEN.pow(new BN(8)));
};
export {};
