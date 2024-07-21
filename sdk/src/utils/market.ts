import { Market } from "../typedef";
import { BumpinAccountNotFound } from "../errors";
import { PublicKey } from "@solana/web3.js";
import {BumpinUtils} from "./utils";

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

  public static getMarketBySymbol(symbol: number[], markets: Market[]): Market {
    let market = markets.find((market) => {
      return BumpinUtils.decodeString(market.symbol) === BumpinUtils.decodeString(symbol);
    });
    if (market === undefined) {
      throw new BumpinAccountNotFound("Market: " + BumpinUtils.decodeString(symbol));
    }
    return market;
  }
}
