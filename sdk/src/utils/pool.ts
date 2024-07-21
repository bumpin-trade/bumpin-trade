import { PublicKey } from "@solana/web3.js";
import { PoolAccount } from "../typedef";
import { BumpinPoolNotFound } from "../errors";

export class BumpinPoolUtils {
  public static getPoolByMintPublicKey(
    mint: PublicKey,
    pools: PoolAccount[]
  ): PoolAccount {
    let pool = pools.find((pool) => {
      return pool.mintKey.equals(mint);
    });
    if (pool === undefined) {
      throw new BumpinPoolNotFound(mint);
    }
    return pool;
  }

  public static getPoolByIndex(
    index: number,
    pools: PoolAccount[]
  ): PoolAccount {
    let pool = pools.find((pool) => {
      return pool.index === index;
    });
    if (pool === undefined) {
      throw new BumpinPoolNotFound(new PublicKey(""));
    }
    return pool;
  }

  public static getPoolByPublicKey(
    poolKey: PublicKey,
    pools: PoolAccount[]
  ): PoolAccount {
    let pool = pools.find((pool) => {
      return pool.key.equals(poolKey);
    });
    if (pool === undefined) {
      throw new BumpinPoolNotFound(new PublicKey(""));
    }
    return pool;
  }
}
