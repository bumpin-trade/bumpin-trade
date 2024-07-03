import {Market} from "../types";
import {BumpinAccountNotFound} from "../errors";

export class BumpinMarketUtils {
    public static getMarketByIndex(index: number, tradeTokens: Market[]): Market {
        let market = tradeTokens.find((market) => {
            return market.index === index;
        });
        if (market === undefined) {
            throw new BumpinAccountNotFound("Market: " + index);
        }
        return market;
    }
}