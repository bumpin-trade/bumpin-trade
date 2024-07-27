import {BN} from '@coral-xyz/anchor';

export class C {
    public static readonly RATE_MULTIPLIER_BN: BN = new BN(100000);
    public static readonly RATE_MULTIPLIER: number = 100000;
    public static readonly RATE_MULTIPLIER_EXPONENT: number = 5;
    public static readonly RATE_MULTIPLIER_NUMBER: number =
        10 ** C.RATE_MULTIPLIER_EXPONENT;
    public static readonly SMALL_RATE_MULTIPLIER_NUMBER: number = 10;
    public static readonly FUNDING_OR_BORROWING_RATE_MULTIPLIER_NUMBER: number = 18;


    public static readonly PRICE_EXPONENT_BN: BN = new BN(8);
    public static readonly PRICE_EXPONENT: number = 100000000;
    public static readonly PRICE_EXPONENT_NUMBER: number = 8;
    public static readonly USD_EXPONENT_NUMBER: number = 10;
}
