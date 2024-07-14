import {BN} from "@coral-xyz/anchor";

export class BumpinConstants {
    public static readonly RATE_MULTIPLIER_BN: BN = new BN(100000);
    public static readonly RATE_MULTIPLIER: number = 100000;

    public static readonly PRICE_EXPONENT_BN: BN = new BN(8);
    public static readonly PRICE_EXPONENT: number = 100000000;
}