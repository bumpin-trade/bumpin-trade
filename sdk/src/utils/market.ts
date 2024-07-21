import { BumpinAccountNotFound } from "../errors";
import { PublicKey } from "@solana/web3.js";
import { Market } from "../beans/beans";

export class BumpinMarketUtils {
  public static getMarketByIndex(index: number, markets: Market[]): Market {
    let market = markets.find((market) => {
      return market.index === index;
    });
    if (market === undefined) {
      throw new BumpinAccountNotFound("Market: " + index);
    }
    return market;
  }

  public static getMarketsByPoolKey(
    poolKey: PublicKey,
    markets: Market[]
  ): Market[] {
    return markets.filter((market) => {
      return market.poolKey.equals(poolKey);
    });
  }

  public static getMarketBySymbol(symbol: string, markets: Market[]): Market {
    let market = markets.find((market) => {
      return market.symbol === symbol;
    });
    if (market === undefined) {
      throw new BumpinAccountNotFound("Market: " + symbol);
    }
    return market;
  }
}
